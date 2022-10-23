pub mod base {
    use serde::{Deserialize, Serialize};
    use time::PrimitiveDateTime;
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct User {
        id: Uuid,
        name: String,
        username: String,
        password: String,
    }

    impl User {
        pub fn login_body(user: String, pass: String) -> Self {
            Self {
                id: Uuid::nil(),
                name: String::new(),
                username: user,
                password: pass,
            }
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
