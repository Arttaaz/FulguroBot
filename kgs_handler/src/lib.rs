use reqwest::header::{CONNECTION, COOKIE, SET_COOKIE};
use reqwest::Client;
use reqwest::Error as ReqwestError;
use reqwest::Response;
use reqwest::StatusCode;

use serde::Deserialize;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

// A client to manage a connection to KGS and fetch data
pub struct KGSClient {
    user: User,         // This is us
    client: Client,     // Reqwest Client to perform HTTP requests
    logged: bool,       // Wether or not the client is logged to KGS
    session_id: String, // Session id to use in the connection cookie
}

pub struct User {
    username: String,
    password: String,

    rank: String,
    flags: String,
}

// We can only use string valued JSON to push to KGS
type JSONData<'a, 'b> = HashMap<&'a str, &'b str>;

// Error that can occur while logging in
#[derive(Debug)]
pub enum LoginError {
    CommunicationError(ReqwestError),
    ServerError(StatusCode),
    InvalidCredential,
    Unkown,
}

// Error that can occur while fetching data from KGS
#[derive(Debug)]
pub enum GetError {
    CommunicationError(ReqwestError),
    NotLoggedIn,
    Unkown,
}

// The HELLO message we recieve at each request
// NOTE: used only for parsing, the data is actually discarded
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[allow(non_snake_case)]
struct HelloMessage {
    versionBugfix: usize,
    versionMajor: usize,
    versionMinor: usize,
    jsonClientBuild: String,
}

#[derive(Deserialize, Debug)]
struct Hello {
    messages: [HelloMessage; 1],
}

impl KGSClient {
    // Instantiate a new client with the given username and password
    // Cannot log in as a guest
    // TODO: do it from a config file
    pub fn new(username: &str, password: &str) -> Self {
        let user = User {
            username: username.to_owned(),
            password: password.to_owned(),
            rank: String::from("U"),
            flags: String::from(""),
        };

        Self {
            user,
            client: Client::new(),
            logged: false,
            session_id: "".to_owned(),
        }
    }

    // Posts JSON data only
    fn post(&self, data: &JSONData) -> Result<Response, ReqwestError> {
        self.client
            .post("http://localhost:8080/kgs_json_api/access")
            .json(data)
            .header(CONNECTION, "keep-alive")
            .send()
    }

    // Gets with the current session_id, if the client is not logged in it does nothing
    fn get(&self) -> Result<Response, GetError> {
        if !self.logged {
            return Err(GetError::NotLoggedIn);
        }
        let mut session_id = String::from("JSESSIONID=");
        session_id.push_str(&self.session_id);
        self.client
            .get("http://localhost:8080/kgs_json_api/access")
            .header(CONNECTION, "keep-alive")
            .header(COOKIE, session_id)
            .send()
            .map_err(|err| GetError::CommunicationError(err))
    }

    // Tries to login to KGS
    pub fn login(&mut self) -> Result<(), LoginError> {
        // If the client is already logged in it does nothing
        if self.logged {
            return Ok(());
        }

        // POSTing the login request
        let login_data = hash_map!(
            "type" => "LOGIN",
            "name"=> &self.user.username,
            "password" => &self.user.password,
            "locale" => "fr_FR",
        );
        // We check that the POST returns "OK"
        let mut res = self.post(&login_data)?;
        if !res.status().is_success() {
            return Err(LoginError::ServerError(res.status()));
        }
        if res.text()? != "OK" {
            return Err(LoginError::Unkown);
        }

        // We can now get the session id from the KGS response
        let mut cookie_chars: Vec<_> = res
            .headers()
            .get(SET_COOKIE)
            .ok_or(LoginError::Unkown)?
            .to_str()
            .map_err(|_| LoginError::Unkown)?
            .chars()
            .collect();
        // Hardcoded filter
        // TODO maybe nice parsing
        let session_chars: Vec<u8> = cookie_chars.drain(11..43).map(|c| c as u8).collect();
        self.session_id = String::from_utf8(session_chars).map_err(|_| LoginError::Unkown)?;

        self.logged = true;

        // We are finaly ready to get the result of the connection
        // Hello message
        self.get()?.json::<Hello>()?;
        // Result
        let result = parse_type_message(&self.get()?.text()?)?;
        if result.contains("LOGIN_FAILED") {
            // Consume the logout message
            // We still need to be logged to get a response from KGS
            if parse_type_message(&self.get()?.text()?)? != "LOGOUT" {
                // We didn't get the logout message from KGS
                self.logged = false;
                Err(LoginError::Unkown)
            } else {
                // We entered invalid credentials
                self.logged = false;
                Err(LoginError::InvalidCredential)
            }
        } else {
            // Everything went fine we are logged in
            Ok(())
        }
    }
}

fn parse_type_message(message_text: &str) -> Result<String, GetError> {
    // Again hardcoded parsing (cannot fail)
    // TODO better parsing
    // Go to the oppening quote we want
    let chars = message_text.chars().skip(22);
    // Go to the next closing quote
    let message_type: Vec<_> = chars.take_while(|&c| c != '"').map(|c| c as u8).collect();
    // We build the String
    String::from_utf8(message_type).map_err(|_| GetError::Unkown)
}

// We interpret any reqwest error as a communication error
impl From<ReqwestError> for LoginError {
    fn from(error: ReqwestError) -> Self {
        LoginError::CommunicationError(error)
    }
}

impl From<GetError> for LoginError {
    fn from(get_error: GetError) -> Self {
        if let GetError::CommunicationError(error) = get_error {
            LoginError::CommunicationError(error)
        } else {
            LoginError::Unkown
        }
    }
}

#[macro_export]
macro_rules! hash_map {
    // This needs a trailing comma
    ( $($key: expr => $value: expr),* ,) => {
        {

        let mut hash_map: HashMap<_, _> = HashMap::new();

        $(
            hash_map.insert($key, $value);
        )*

        hash_map
        }
    };
}
