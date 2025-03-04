use azalea_buf::McBuf;
use azalea_chat::component::Component;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundResourcePackPacket {
    pub url: String,
    pub hash: String,
    pub required: bool,
    pub prompt: Option<Component>,
}
