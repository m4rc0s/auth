use serde::{Deserialize, Serialize};
use skytable::{Connection, SkyResult};
use skytable::ddl::{Ddl, Keymap, KeymapType};
use skytable::actions::Actions;
use serde_json;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct User {
    username: String,
    password: String,
    verified: bool
}

fn create_user(username: String, password: String) -> User {
    let user = User { 
        username: username, 
        password: password, 
        verified: true
    };
    
    return user;
}

fn main() -> SkyResult<()> {
    let mut con = Connection::new("127.0.0.1", 2003)?;

    let username = String::from("test6@gmail.com");
    let password = String::from("123456");

    let user = create_user(username, password);

    con.set(&user.username, serde_json::to_string(&user).unwrap()).unwrap();

    Ok(())
}

#[cfg(test)]
mod unit_test {

    use super::*;

    #[test]
    fn should_create_user() {
        let username = String::from("test");
        let password = String::from("test");

        let user = create_user(username, password);

        assert_eq!(user.username, String::from("test"));
        assert_eq!(user.password, String::from("test"));
    }

    #[test]
    fn should_convert_to_json() {
        let username = String::from("test");
        let password = String::from("test");

        let user = create_user(username, password);

        serde_json::to_string(&user).unwrap();

    }

}