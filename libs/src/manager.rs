pub mod packet_manager {
    use std::{
        io::{Read, Write},
        net::TcpStream,
    };

    use crate::packet::{DataPacket, PacketError};

    pub fn send_packet(stream: &mut TcpStream, packet: DataPacket) -> Result<(), PacketError> {
        let buf = match packet.buf() {
            Ok(buf) => buf,
            Err(err) => {
                return Err(PacketError {
                    message: err.to_string(),
                })
            }
        };

        let buf = buf.as_bytes();

        let start: [u8; 1] = [2];
        let end: [u8; 1] = [3];

        if let Err(err) = stream.write(&start) {
            return Err(PacketError {
                message: err.to_string(),
            });
        }

        if let Err(err) = stream.write(buf) {
            return Err(PacketError {
                message: err.to_string(),
            });
        }

        if let Err(err) = stream.write(&end) {
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

    pub fn recv_packet(stream: &mut TcpStream) -> Result<DataPacket, PacketError> {
        let mut start: [u8; 1] = [0];
        if let Err(err) = stream.read_exact(&mut start) {
            return Err(PacketError {
                message: err.to_string(),
            });
        }

        if start[0] != 2 {
            return Err(PacketError {
                message: String::from("Bad Packet"),
            });
        }

        let mut buf = Vec::new();

        while let Some(data) = stream.bytes().next() {
            match data {
                Ok(byte) => {
                    if byte == 3 {
                        break;
                    }

                    buf.push(byte);
                }
                Err(err) => {
                    return Err(PacketError {
                        message: err.to_string(),
                    });
                }
            }
        }

        let buf = match String::from_utf8(buf) {
            Ok(buf) => buf,
            Err(err) => {
                return Err(PacketError {
                    message: err.to_string(),
                });
            }
        };

        match DataPacket::new(buf) {
            Ok(packet) => Ok(packet),
            Err(err) => Err(PacketError {
                message: err.to_string(),
            }),
        }
    }

    pub fn sign_packet(packet: DataPacket) -> Result<DataPacket, PacketError> {
        todo!()
    }

    pub fn check_signed_packet(packet: DataPacket) -> Result<(), PacketError> {
        todo!()
    }

    pub fn encrypt_packet(packet: DataPacket) -> Result<DataPacket, PacketError> {
        todo!()
    }

    pub fn decrypt_packet(packet: DataPacket) -> Result<DataPacket, PacketError> {
        todo!()
    }
}
