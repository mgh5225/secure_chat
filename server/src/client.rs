use std::{net::TcpStream, sync::Arc};

use libs::{
    packet::{DataPacket, Packet, PacketType},
    packet_manager, BaseModels, PacketModels,
};

use crate::Database;

pub struct Client {
    stream: TcpStream,
    me: Option<BaseModels::User>,
    db: Arc<Database>,
}

impl Client {
    pub fn new(stream: TcpStream, db: Arc<Database>) -> Self {
        Client {
            stream,
            db,
            me: None,
        }
    }

    pub fn run(&mut self) {
        let packet = packet_manager::recvPacket(&mut self.stream).unwrap();

        let packet: DataPacket = match packet.get_type() {
            PacketType::PubKey => todo!(),
            PacketType::Error => todo!(),
            PacketType::Empty => todo!(),
            PacketType::Ok => todo!(),
            PacketType::E2E => match Packet::parse(&packet, "Packet Type Error E2E") {
                Ok(packet) => self.start_e2e(packet),
                Err(packet) => packet,
            },
            PacketType::Login => match Packet::parse(&packet, "Packet Type Error Login") {
                Ok(packet) => self.login_user(packet),
                Err(packet) => packet,
            },
            PacketType::Logout => {
                self.me = None;
                DataPacket::OkMessage(String::from("Logout Successfully"))
            }
            PacketType::Register => match Packet::parse(&packet, "Packet Type Error Register") {
                Ok(packet) => self.register_user(packet),
                Err(packet) => packet,
            },
            PacketType::CreateGroup => {
                match Packet::parse(&packet, "Packet Type Error CreateGroup") {
                    Ok(packet) => self.create_group(packet),
                    Err(packet) => packet,
                }
            }
            PacketType::AddUser => match Packet::parse(&packet, "Packet Type Error AddUser") {
                Ok(packet) => self.add_user(packet),
                Err(packet) => packet,
            },

            PacketType::CreateMessage => {
                match Packet::parse(&packet, "Packet Type Error CreateMessage") {
                    Ok(packet) => self.create_message(packet),
                    Err(packet) => packet,
                }
            }
            PacketType::GetMessages => {
                match Packet::<PacketModels::Empty>::parse(&packet, "Packet Type Error GetMessages")
                {
                    Ok(_) => self.get_messages(),
                    Err(packet) => packet,
                }
            }
            PacketType::GetChats => {
                match Packet::<PacketModels::Empty>::parse(&packet, "Packet Type Error GetChats") {
                    Ok(_) => self.get_chats(),
                    Err(packet) => packet,
                }
            }
            PacketType::Listen => todo!(),
            _ => DataPacket::ErrorMessage(String::from("Packet Type Error")),
        };

        packet_manager::sendPacket(&mut self.stream, packet).unwrap();
    }

    fn exchange_keys(&self) -> DataPacket {
        todo!()
    }
    fn start_e2e(&self, packet: Packet<PacketModels::E2E>) -> DataPacket {
        todo!()
    }
    fn register_user(&self, packet: Packet<BaseModels::User>) -> DataPacket {
        let res_packet = match self.db.create_user(packet.get().1) {
            Ok(()) => DataPacket::OkMessage(String::from("User Created Successfully")),
            Err(err) => DataPacket::ErrorMessage(err.to_string()),
        };

        res_packet
    }
    fn login_user(&self, packet: Packet<BaseModels::User>) -> DataPacket {
        let user = match self.db.get_user(packet.get().1.get_key()) {
            Ok(user) => user,
            Err(err) => return DataPacket::ErrorMessage(err.to_string()),
        };

        DataPacket::OkMessage(String::from("Login Successfully"))
    }
    fn create_group(&self, packet: Packet<BaseModels::Group>) -> DataPacket {
        todo!()
    }
    fn add_user(&self, packet: Packet<BaseModels::Member>) -> DataPacket {
        todo!()
    }
    fn create_message(&self, packet: Packet<BaseModels::Message>) -> DataPacket {
        todo!()
    }
    fn get_messages(&self) -> DataPacket {
        todo!()
    }
    fn get_chats(&self) -> DataPacket {
        todo!()
    }
}
