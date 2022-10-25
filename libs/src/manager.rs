pub mod packet_manager {
    use std::{
        io::{Read, Write},
        net::TcpStream,
    };

    use crate::packet::{DataPacket, PacketError};

    pub fn sendPacket(stream: &mut TcpStream, packet: DataPacket) -> Result<(), PacketError> {
        let buf = match packet.buf() {
            Ok(buf) => buf,
            Err(err) => {
                return Err(PacketError {
                    message: err.to_string(),
                })
            }
        };

        if let Err(err) = stream.write(buf.as_bytes()) {
            return Err(PacketError {
                message: err.to_string(),
            });
        }

        if let Err(err) = stream.flush() {
            return Err(PacketError {
                message: err.to_string(),
            });
        }

        Ok(())
    }

    pub fn recvPacket(stream: &mut TcpStream) -> Result<DataPacket, PacketError> {
        let mut buf = String::new();
        match stream.read_to_string(&mut buf) {
            Ok(_) => match DataPacket::new(buf) {
                Ok(packet) => Ok(packet),
                Err(err) => Err(PacketError {
                    message: err.to_string(),
                }),
            },
            Err(err) => Err(PacketError {
                message: err.to_string(),
            }),
        }
    }

    pub fn signPacket(packet: DataPacket) -> Result<DataPacket, PacketError> {
        todo!()
    }

    pub fn checkSignedPacket(packet: DataPacket) -> Result<(), PacketError> {
        todo!()
    }

    pub fn encryptPacket(packet: DataPacket) -> Result<DataPacket, PacketError> {
        todo!()
    }

    pub fn decryptPacket(packet: DataPacket) -> Result<DataPacket, PacketError> {
        todo!()
    }
}
