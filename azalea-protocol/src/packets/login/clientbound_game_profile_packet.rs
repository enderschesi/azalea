use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundLoginPacket;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundLoginPacket)]
pub struct ClientboundGameProfilePacket {
    pub game_profile: GameProfile,
    pub strict_error_handling: bool,
}

#[derive(McBuf, Debug, Clone, Default, Eq, PartialEq)]
pub struct GameProfile {
    /// The UUID of the player.
    pub uuid: Uuid,
    /// The username of the player.
    pub name: String,
    pub properties: HashMap<String, ProfilePropertyValue>,
}

impl GameProfile {
    pub fn new(uuid: Uuid, name: String) -> Self {
        GameProfile {
            uuid,
            name,
            properties: HashMap::new(),
        }
    }
}

#[derive(McBuf, Debug, Clone, Eq, PartialEq)]
pub struct ProfilePropertyValue {
    pub value: String,
    pub signature: Option<String>,
}
