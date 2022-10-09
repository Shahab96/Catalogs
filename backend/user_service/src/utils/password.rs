use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .hash
        .unwrap()
        .to_string()
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

    argon2
        .verify_password(hashed_password.as_bytes(), &hash)
        .is_ok()
}
