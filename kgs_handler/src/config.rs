extern crate config;

pub struct Config {
    pub username: String,
    pub password: String,
}

impl Config {
    // Loads the config file or creates it if it doesn't exist
    pub fn load() -> Self {
        let mut settings = config::Config::default();
        // We panic! if we cannot parse the credentials file
        if let Err(err) = settings
            // Add in `./Credentials.toml`
            .merge(config::File::with_name("Credentials"))
        {
            panic!("{}", err);
        }

        let username = settings
            .get_str("username")
            .expect("Cannot parse username in credentials");
        let password = settings
            .get_str("password")
            .expect("Cannot parse password in credentials");

        Config { username, password }
    }

    // Build a config programmaticaly
    pub fn new(username: &str, password: &str) -> Self {
        Config {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
}
