// Structures that represent the JSON objects of the protocol
extern crate serde_json;

use futures::*;
use reqwest::r#async::Response;
use serde_json::{Map, Value};

use crate::KGSError;

use std::collections::HashMap;

pub type JSONResponse = HashMap<String, Map<String, Value>>;

// Converts a response into a map of messages

// shortcut
const PARSING_ERROR: KGSError = KGSError::ParsingError;

// From a response returns a future with the response text
pub fn get_response_text(res: Response) -> impl Future<Item = String, Error = KGSError> {
    res.into_body()
        .concat2()
        .map(|body| String::from_utf8_lossy(&body).to_string())
        .map_err(KGSError::CommunicationError)
}

pub fn get_messages_map(res: Response) -> impl Future<Item = JSONResponse, Error = KGSError> {
    get_json_messages(res).map(|messages| {
        let mut map = HashMap::new();

        // For all messages we insert it in the map with the type as key
        // TODO use collect() ?
        for message in messages {
            if let Value::Object(message_map) = message {
                // Find the type and use it as the key
                // TODO error handling
                let type_string = message_map.get("type").unwrap().to_owned();
                if let Value::String(type_string) = type_string {
                    map.insert(type_string, message_map);
                } else {
                    panic!("type string is not a string");
                }
            } else {
                // TODO error handling
                panic!("Cannot parse JSON object in KGS message");
            }
        }

        map
    })
}

fn get_json_messages(res: Response) -> impl Future<Item = Vec<Value>, Error = KGSError> {
    get_response_text(res).map(|text| {
        // TODO propagate parsing errors
        get_kgs_messages(&text).expect("Can't parse JSON response")
    })
}

// Returns a vector of messages contained in the response
fn get_kgs_messages(response: &str) -> Result<Vec<Value>, KGSError> {
    let response: Value = serde_json::from_str(response)?;
    // Checking that the main object is a map
    if let Value::Object(mut map) = response {
        // With a key: "messages"
        map.get_mut("messages")
            .map_or(Err(PARSING_ERROR), |messages| {
                let messages = std::mem::replace(messages, Value::Null);
                // The value of this key should be an array and we return it
                if let Value::Array(values) = messages {
                    Ok(values)
                } else {
                    Err(PARSING_ERROR)
                }
            })
    } else {
        Err(PARSING_ERROR)
    }
}

impl From<serde_json::Error> for KGSError {
    fn from(_err: serde_json::Error) -> KGSError {
        PARSING_ERROR
    }
}
