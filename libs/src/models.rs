pub mod base {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use time::PrimitiveDateTime;
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct User {
        name: String,
        username: String,
        password: String,
    }

    impl User {
        pub fn simple(user: String, pass: String) -> Self {
            Self {
                name: String::new(),
                username: user,
                password: pass,
            }
        }

        pub fn full(name: String, user: String, pass: String) -> Self {
            Self {
                name,
                username: user,
                password: pass,
            }
        }

        pub fn get_hash(self) -> (String, Vec<(u8, String)>) {
            let mut key_pairs = Vec::new();
            key_pairs.push((0, self.name));
            key_pairs.push((1, self.username.clone()));
            key_pairs.push((2, self.password));

            (self.username, key_pairs)
        }

        pub fn from_hash(hash: HashMap<u8, String>) -> Self {
            let name = hash.get(&0).unwrap();
            let username = hash.get(&1).unwrap();
            let password = hash.get(&2).unwrap();

            Self {
                name: name.to_string(),
                username: username.to_string(),
                password: password.to_string(),
            }
        }

        pub fn get_key(self) -> String {
            return self.username;
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct Message {
        id: Uuid,
        member: Member,
        body: String,
        created_at: PrimitiveDateTime,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Group {
        id: Uuid,
        name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Member {
        group: Group,
        user: User,
    }
}

pub mod packet {
    use crate::models::base::*;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct E2E {
        public_key: String,
        user: User,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Chats {
        is_new: bool,
        groups: Vec<Group>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Messages {
        group: Group,
        messages: Vec<Message>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Refresh {
        id: Uuid,
        is_group: bool,
        is_message: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Empty {}
}
