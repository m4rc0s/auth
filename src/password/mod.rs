use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Scrypt
};

pub fn hash(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Scrypt.hash_password(password.as_bytes(), &salt).ok().unwrap().to_string();

    return hash
}

pub fn verify(password: String, password_hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();

    return Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok();
}
