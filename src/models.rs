use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuth {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(email: &str, password: &str) -> Self {
        let hashed_password = bcrypt::hash(&password, 4).unwrap();
        User {
            email: email.to_string(),
            password_hash: hashed_password.to_string(),
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password_hash).unwrap()
    }
}
