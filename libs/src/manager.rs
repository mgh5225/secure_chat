pub mod PacketManager {
    use crate::packet::{DataPacket, PacketError};

    pub fn sendPacket(packet: DataPacket) -> Result<(), PacketError> {
        todo!()
    }

    pub fn recvPacket() -> Result<DataPacket, PacketError> {
        todo!()
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
