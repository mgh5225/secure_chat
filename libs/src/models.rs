mod base {
    use time::PrimitiveDateTime;
    use uuid::Uuid;

    pub struct User {
        id: Uuid,
        name: String,
        username: String,
        password: String,
    }

    pub struct Message {
        id: Uuid,
        member: Member,
        body: String,
        created_at: PrimitiveDateTime,
    }

    pub struct Group {
        id: Uuid,
        name: String,
    }

    pub struct Member {
        group: Group,
        user: User,
    }
}

mod packet {
    use crate::models::base::*;
    use uuid::Uuid;

    pub struct E2E {
        public_key: String,
        user: User,
    }

    pub struct Chats {
        is_new: bool,
        groups: Vec<Group>,
    }

    pub struct Messages {
        group: Group,
        messages: Vec<Message>,
    }

    pub struct Refresh {
        id: Uuid,
        is_group: bool,
        is_message: bool,
    }
}
