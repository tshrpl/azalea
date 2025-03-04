use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundLoginPacket;
use uuid::Uuid;

#[derive(Clone, Debug, ServerboundLoginPacket, McBuf, PartialEq)]
pub struct ServerboundHelloPacket {
    pub username: String,
    pub public_key: Option<ProfilePublicKeyData>,
    pub profile_id: Option<Uuid>,
}

#[derive(Clone, Debug, McBuf, PartialEq)]
pub struct ProfilePublicKeyData {
    pub expires_at: u64,
    pub key: Vec<u8>,
    pub key_signature: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use azalea_buf::{McBufReadable, McBufWritable};

    #[test]
    fn test_read_write() {
        let packet = ServerboundHelloPacket {
            username: "test".to_string(),
            public_key: None,
            profile_id: Some(Uuid::from_u128(0)),
        };
        let mut buf = Vec::new();
        packet.write_into(&mut buf).unwrap();
        let packet2 = ServerboundHelloPacket::read_from(&mut buf.as_slice()).unwrap();
        assert_eq!(packet, packet2);
    }
}
