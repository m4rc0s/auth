use skytable::{
    types::{FromSkyhashBytes, IntoSkyhashBytes},
    Element, SkyResult
};
use skytable::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

// Implement this for our type so that we can directly add it to queries
impl IntoSkyhashBytes for &Login {
    fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_string(self).unwrap().into_bytes()
    }
}

// Implement this for our type so that we can directly use it with actions/queries
impl FromSkyhashBytes for Login {
    fn from_element(e: Element) -> SkyResult<Self> {
        // we want our JSON as a string
        let my_value_as_string: String = e.try_element_into()?;
        // now let us convert it into our struct
        match serde_json::from_str(&my_value_as_string) {
            // good, we got it
            Ok(v) => Ok(v),
            // nah, something bad happened. We'll turn the error into a string
            // and return it
            Err(e) => Err(Error::ParseError(e.to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub verified: bool,
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LoginResult {
    pub token: String,
}
