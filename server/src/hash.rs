use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

/// Hashes a password using Argon2id and a random salt.
/// Returns the full PHC string (e.g., "$argon2id$v=19$m=4096...")
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    // Hash the password and convert to string format for DB storage
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    
    Ok(password_hash.to_string())
}