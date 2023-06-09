use jwt_simple::prelude::*;

pub fn token(key: &HS256Key) -> Result<String, jwt_simple::Error> {
    let claims = Claims::create(Duration::from_mins(2));
    let token = key.authenticate(claims);

    return token
}

pub fn verify(key: &HS256Key, token: &str) -> bool {
    key.verify_token::<NoCustomClaims>(token, None).is_ok()
} 

#[cfg(test)]
mod tests {
    use jwt_simple::prelude::HS256Key;
    use crate::{jwt::token, jwt::verify};

    #[test]
    fn should_verify_jwt_token() {
        let key = HS256Key::generate();

        let token = token(&key).unwrap();

        let result = verify(&key, &token);

        assert!(result);
    }

    #[test]
    fn should_fail_if_token_is_invalid() {
        let key = HS256Key::generate();

        let result = verify(&key, "invalid token");

        assert!(!result);
    }
}