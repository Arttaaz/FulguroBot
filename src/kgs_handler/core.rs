use reqwest;
use std::collections::HashMap;

pub struct Client {
    user: User, //This is us
}

pub struct User {
    username: String,
    password: String,
    rank: String,
    flags: String,
}

impl Client {
    pub fn start(username:String, password:String) -> Client {
        let user = User {
            username: username,
            password : password,
            rank : String::from("U"),
            flags : String::from(""),
        };

        let client = Client {
            user: user,
        };

        client
    }
    pub fn login(&self) {
        let mut data = HashMap::new();

        data.insert("type","LOGIN");
        data.insert("name",&self.user.username);
        data.insert("password",&self.user.password);
        data.insert("locale","fr_FR");

        let client = reqwest::Client::new();
        let mut res = client.post("localhost::8080/access")
        .json(&data)
        .send().expect("Could not login");
        
        println!("{}",res.text().expect("Unvalid response text"));
    }
}
