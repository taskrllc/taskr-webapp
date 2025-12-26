use argon2::{
    Argon2,
    password_hash::{
        self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};

#[derive(Debug, Clone)]
pub struct PasswordChecker<'a> {
    salt: SaltString,
    argon: Argon2<'a>,
    password: String,
}

impl<'a> PasswordChecker<'a> {
    pub fn new(password: &str) -> Result<Self, password_hash::Error> {
        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon.hash_password(password.as_bytes(), &salt)?.to_string();
        Ok(Self {
            salt: salt,
            argon: argon,
            password: hash,
        })
    }

    pub fn from_hash(hash: &str) -> Self {
        Self {
            salt: SaltString::generate(&mut OsRng),
            argon: Argon2::default(),
            password: String::from(hash),
        }
    }

    pub fn check(&self, plaintext: &str) -> bool {
        if let Ok(hash) = PasswordHash::new(&self.password) {
            return self
                .argon
                .verify_password(plaintext.as_bytes(), &hash)
                .is_ok();
        }
        false
    }
}
