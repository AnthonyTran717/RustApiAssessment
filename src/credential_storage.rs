use std::collections::HashMap;

pub struct CredentialStorage {
    credentials: HashMap<String, Credential>, //<email, Credential>
}

pub struct Credential {
    pub user: String,
    pub password: String
}

impl CredentialStorage {
    pub fn new() -> CredentialStorage {
        CredentialStorage {
            credentials: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, email: String, user: String, password: String) {
        self.credentials.insert(email, Credential {
            user, password
        });
    }

    pub fn get_user(&self, email: String) -> Option<&Credential> {
        self.credentials.get(&email)
    }
}