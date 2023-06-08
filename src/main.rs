#[derive(Debug)]
struct User {
    username: String,
    password: String
}

fn create_user(username: String, password: String) -> User {
    let user = User { username, password };
    return user;
}

fn main() {
    let username = String::from("test@gmail.com");
    let password = String::from("123456");

    let user = create_user(username, password);

    println!("New user created: {}", user.username);
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

}