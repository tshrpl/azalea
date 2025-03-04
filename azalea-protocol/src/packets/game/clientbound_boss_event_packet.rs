use std::io::{Read, Write};

use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_chat::component::Component;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBossEventPacket {
    pub id: Uuid,
    pub operation: Operation,
}

#[derive(Clone, Debug)]
pub enum Operation {
    Add(AddOperation),
    Remove,
    UpdateProgress(f32),
    UpdateName(Component),
    UpdateStyle(Style),
    UpdateProperties(Properties),
}

impl McBufReadable for Operation {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        let operation_id = u32::var_read_from(buf)?;
        Ok(match operation_id {
            0 => Operation::Add(AddOperation::read_from(buf)?),
            1 => Operation::Remove,
            2 => Operation::UpdateProgress(f32::read_from(buf)?),
            3 => Operation::UpdateName(Component::read_from(buf)?),
            4 => Operation::UpdateStyle(Style::read_from(buf)?),
            5 => Operation::UpdateProperties(Properties::read_from(buf)?),
            _ => {
                return Err(BufReadError::UnexpectedEnumVariant {
                    id: operation_id as i32,
                })
            }
        })
    }
}

impl McBufWritable for Operation {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Operation::Add(add) => {
                0u32.var_write_into(buf)?;
                add.write_into(buf)?;
            }
            Operation::Remove => {
                1u32.var_write_into(buf)?;
            }
            Operation::UpdateProgress(progress) => {
                2u32.var_write_into(buf)?;
                progress.write_into(buf)?;
            }
            Operation::UpdateName(name) => {
                3u32.var_write_into(buf)?;
                name.write_into(buf)?;
            }
            Operation::UpdateStyle(style) => {
                4u32.var_write_into(buf)?;
                style.write_into(buf)?;
            }
            Operation::UpdateProperties(properties) => {
                5u32.var_write_into(buf)?;
                properties.write_into(buf)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, McBuf)]
pub struct AddOperation {
    name: Component,
    progress: f32,
    style: Style,
    properties: Properties,
}

#[derive(Clone, Debug, McBuf)]
pub struct Style {
    color: BossBarColor,
    overlay: BossBarOverlay,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum BossBarColor {
    Pink = 0,
    Blue = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Purple = 5,
    White = 6,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum BossBarOverlay {
    Progress = 0,
    Notched6 = 1,
    Notched10 = 2,
    Notched12 = 3,
    Notched20 = 4,
}

#[derive(Clone, Debug)]
pub struct Properties {
    pub darken_screen: bool,
    pub play_music: bool,
    pub create_world_fog: bool,
}

impl McBufReadable for Properties {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        let byte = u8::read_from(buf)?;
        Ok(Self {
            darken_screen: byte & 1 != 0,
            play_music: byte & 2 != 0,
            create_world_fog: byte & 4 != 0,
        })
    }
}

impl McBufWritable for Properties {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut byte = 0;
        if self.darken_screen {
            byte |= 1;
        }
        if self.play_music {
            byte |= 2;
        }
        if self.create_world_fog {
            byte |= 4;
        }
        u8::write_into(&byte, buf)?;
        Ok(())
    }
}
