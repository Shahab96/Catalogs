use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(e) => panic!(
            "If you're seeing this message, you fucked up. The password provided by a user could not be hashed on registration. {}",
            e
        ),
    }
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hashed_password) {
        Ok(p) => p,
        Err(e) => panic!(
            "If you're seeing this message, you fucked up. The PasswordHash constructor failed. {}",
            e
        ),
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
