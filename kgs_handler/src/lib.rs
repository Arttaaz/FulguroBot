use reqwest::r#async::{Client, ClientBuilder, RequestBuilder, Response};
use reqwest::Error as ReqwestError;
use reqwest::StatusCode;

use futures::future::*;

use std::collections::HashMap;
use std::time::Duration;

mod config;
mod game_management;
mod protocol;

#[cfg(test)]
mod tests;

pub use crate::config::*;
use game_management::*;
use protocol::*;

// This file contains the main structure of the lib KGSClient
// It also contains the boilerplate code to make requests and the login protocol
// Other KGS features are in other files

// A client to manage a connection to KGS and fetch data
pub struct KGSClient {
    client: Client,            // Reqwest Client to perform asynchrone HTTP requests
    logged: bool,              // Wether or not the client is logged to KGS
    game_manager: GameManager, // A game manager to keep track of games and progress
}

// Error that can occur while communicating with KGS
#[derive(Debug)]
pub enum KGSError {
    CommunicationError(ReqwestError),
    ServerError(StatusCode),
    InvalidCredential,
    ParsingError,
    NotLoggedIn,
    Unkown,
}

impl KGSClient {
    // Instantiate a new client with the given username and password
    // Cannot log in as a guest
    // TODO: do it from a config file
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::new(60, 0))
            .cookie_store(true)
            .gzip(true)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            logged: false,
            game_manager: GameManager::new(),
        }
    }

    // Returns a future that POST the given JSON data
    fn post_kgs(&self, data: &JSONData) -> impl Future<Item = Response, Error = KGSError> {
        send_and_assert_200(
            self.client
                .post("http://localhost:8080/kgs_json_api/access")
                .json(&data),
        )
    }

    // Returns a GET response in a future, the client has to be logged in
    fn get_kgs(&self) -> impl Future<Item = Response, Error = KGSError> {
        send_and_assert_200(self.client.get("http://localhost:8080/kgs_json_api/access"))
    }

    fn get_kgs_json(&self) -> impl Future<Item = JSONResponse, Error = KGSError> {
        self.get_kgs().and_then(get_messages_map)
    }

    // Tries to login the given user (TODO give the user as a Config parameter)
    pub fn login(&mut self, config: Config) -> Result<(), KGSError> {
        if self.logged {
            return Ok(());
        }
        // POSTing the login request
        let login_data = hash_map!(
            "type" => "LOGIN",
            "name"=> &config.username,
            "password" => &config.password,
            "locale" => "fr_FR",
        );

        let mut rt =
            tokio::runtime::current_thread::Runtime::new().expect("Failed to access tokio runtime");
        // Post the request
        // Check that the body is OK
        if rt.block_on(self.post_kgs(&login_data).and_then(get_response_text))? == "OK" {
            self.logged = true;

            // The login process is fully synchronous, we wait for the GET response
            // We discard the HELLO message
            if !rt.block_on(self.get_kgs_json())?.contains_key("HELLO") {
                return Err(KGSError::ParsingError);
            }
            // We look at the login result
            let login_result = rt.block_on(self.get_kgs_json())?;
            if login_result.contains_key("LOGIN_FAILED_NO_SUCH_USER")
                || login_result.contains_key("LOGIN_FAILED_BAD_PASSWORD")
                || login_result.contains_key("LOGIN_FAILED_USER_ALREADY_EXISTS")
            {
                // Wait for the LOGOUT message and fail
                if !rt.block_on(self.get_kgs_json())?.contains_key("LOGOUT") {
                    Err(KGSError::Unkown)
                } else {
                    self.logged = false;
                    Err(KGSError::InvalidCredential)
                }
            } else {
                // We check that the connection is indeed a sucess
                if !login_result.contains_key("LOGIN_SUCCESS") {
                    return Err(KGSError::Unkown);
                }

                // TODO parsing all the info that we have from kgs

                Ok(())
            }
        } else {
            Err(KGSError::Unkown)
        }
    }
}

impl Default for KGSClient {
    fn default() -> Self {
        Self::new()
    }
}

// Build and send the given request and returns a future containing the response checking that we
// got a status code of 200
fn send_and_assert_200(request: RequestBuilder) -> impl Future<Item = Response, Error = KGSError> {
    request
        .send()
        .map_err(KGSError::from)
        // Assert that we get a 200 status code
        .and_then(|res| {
            if res.status() == StatusCode::OK {
                ok::<_, KGSError>(res)
            } else {
                err::<_, KGSError>(KGSError::ServerError(res.status()))
            }
        })
}

// We can only use string valued JSON to push to KGS
type JSONData<'a, 'b> = HashMap<&'a str, &'b str>;

// We interpret any reqwest error as a communication error
impl From<ReqwestError> for KGSError {
    fn from(error: ReqwestError) -> Self {
        KGSError::CommunicationError(error)
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
