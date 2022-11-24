use std::collections::HashMap;

pub struct CredentialStorage {
    credentials: HashMap<String, Credential>, //<email, Credential>
}

struct Credential {
    user: String,
    password: String
}

impl CredentialStorage {
    pub fn new() -> CredentialStorage {
        CredentialStorage {
            credentials: HashMap::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.credentials.keys().len()
    }

    pub fn add_user(&mut self, email: String, user: String, password: String) {
        self.credentials.insert(email, Credential {
            user, password
        });
    }
}