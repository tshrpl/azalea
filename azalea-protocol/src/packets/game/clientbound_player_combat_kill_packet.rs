use azalea_buf::McBuf;
use azalea_chat::component::Component;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerCombatKillPacket {
    #[var]
    pub player_id: u32,
    pub killer_id: u32,
    pub message: Component,
}
