use libs::{
    packet::{DataPacket, Packet, PacketType},
    BaseModels, PacketManager, PacketModels,
};
use std::fmt;

pub struct Session {
    me: Option<BaseModels::User>,
    groups: Vec<BaseModels::Group>,
    messages: Vec<BaseModels::Message>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            me: None,
            groups: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn login(&mut self, user: String, pass: String) -> Result<(), SessionError> {
        let body = BaseModels::User::login_body(user, pass);

        let packet = Packet::new(PacketType::Login, body);

        let data_packet = match packet.to() {
            Ok(data) => data,
            Err(err) => {
                return Err(SessionError {
                    message: err.to_string(),
                })
            }
        };

        let res = PacketManager::sendPacket(data_packet);

        if let Err(err) = res {
            return Err(SessionError {
                message: err.to_string(),
            });
        }

        let data_packet = match PacketManager::recvPacket() {
            Ok(packet) => packet,
            Err(err) => {
                return Err(SessionError {
                    message: err.to_string(),
                })
            }
        };

        let packet: Packet<BaseModels::User> =
            match Packet::parse(&data_packet, "Wrong Packet Received") {
                Ok(packet) => packet,
                Err(data_packet) => match PacketManager::sendPacket(data_packet) {
                    Ok(_) => {
                        return Err(SessionError {
                            message: String::from("Wrong Packet Received"),
                        })
                    }
                    Err(err) => {
                        return Err(SessionError {
                            message: err.to_string(),
                        })
                    }
                },
            };

        self.me = Some(packet.get().1);

        Ok(())
    }
}

pub struct SessionError {
    pub message: String,
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ PackerError: {} }}", self.message)
    }
}

impl fmt::Debug for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SessionError")
            .field("message", &self.message)
            .finish()
    }
}
