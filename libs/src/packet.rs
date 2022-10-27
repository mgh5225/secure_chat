use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct DataPacket {
    p_type: PacketType,
    data: String,
}

impl DataPacket {
    pub fn error_message(message: String) -> Self {
        Self {
            p_type: PacketType::Error,
            data: message,
        }
    }

    pub fn ok_message(message: String) -> Self {
        Self {
            p_type: PacketType::Ok,
            data: message,
        }
    }

    pub fn new(buf: String) -> Result<Self, serde_json::Error> {
        let packet: Self = serde_json::from_str(&buf)?;
        Ok(packet)
    }

    pub fn buf(&self) -> Result<String, serde_json::Error> {
        let buf = serde_json::to_string(self)?;

        Ok(buf)
    }

    pub fn get_type(&self) -> PacketType {
        self.p_type
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

    pub fn get(self) -> (PacketType, T) {
        (self.p_type, self.body)
    }

    pub fn parse(packet: &'a DataPacket, error_message: &str) -> Result<Self, DataPacket> {
        let packet = match Self::from(packet) {
            Ok(packet) => packet,
            _ => return Err(DataPacket::error_message(String::from(error_message))),
        };

        Ok(packet)
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
    pub message: String,
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
