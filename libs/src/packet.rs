use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct DataPacket {
    p_type: PacketType,
    data: String,
}

impl DataPacket {
    pub fn ErrorMessage(message: String) -> Self {
        Self {
            p_type: PacketType::Error,
            data: message,
        }
    }

    pub fn OkMessage(message: String) -> Self {
        Self {
            p_type: PacketType::Ok,
            data: message,
        }
    }
}

pub struct Packet<T> {
    p_type: PacketType,
    body: T,
}

impl<'a, T> Packet<T>
where
    T: Serialize + Deserialize<'a>,
{
    pub fn from(packet: &'a DataPacket) -> Result<Self, serde_json::Error> {
        let body: T = serde_json::from_str(packet.data.as_str())?;

        let new_packet = Self {
            p_type: packet.p_type,
            body,
        };

        Ok(new_packet)
    }

    pub fn to(&self) -> Result<DataPacket, serde_json::Error> {
        let data = serde_json::to_string(&self.body)?;

        let packet = DataPacket {
            p_type: self.p_type,
            data,
        };

        Ok(packet)
    }

    pub fn new(p_type: PacketType, body: T) -> Self {
        Self { p_type, body }
    }

    pub fn get(&self) -> (PacketType, &T) {
        (self.p_type, &self.body)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum PacketType {
    PubKey,
    Error,
    Empty,
    Ok,
    Refresh,
    E2E,
    Login,
    Logout,
    Register,
    CreateGroup,
    AddUser,
    CreateMessage,
    GetMessages,
    GetChats,
    Listen,
}

pub struct PacketError {
    message: String,
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ PackerError: {} }}", self.message)
    }
}

impl fmt::Debug for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PacketError")
            .field("message", &self.message)
            .finish()
    }
}
