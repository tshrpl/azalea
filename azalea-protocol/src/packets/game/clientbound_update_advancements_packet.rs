use azalea_buf::McBuf;
use azalea_chat::component::Component;
use azalea_core::{ResourceLocation, Slot};
use azalea_protocol_macros::ClientboundGamePacket;
use std::collections::HashMap;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateAdvancementsPacket {
    pub reset: bool,
    pub added: HashMap<ResourceLocation, Advancement>,
    pub removed: Vec<ResourceLocation>,
    pub progress: HashMap<ResourceLocation, AdvancementProgress>,
}

#[derive(Clone, Debug, McBuf)]
pub struct Advancement {
    parent_id: Option<ResourceLocation>,
    display: Option<DisplayInfo>,
    // rewards: AdvancementRewards.EMPTY,
    criteria: HashMap<ResourceLocation, Criterion>,
    requirements: Vec<Vec<String>>,
    // requirements_strategy: RequirementsStrategy.AND
}

#[derive(Clone, Debug)]
pub struct DisplayInfo {
    pub title: Component,
    pub description: Component,
    pub icon: Slot,
    pub frame: FrameType,
    pub show_toast: bool,
    pub hidden: bool,
    pub background: Option<ResourceLocation>,
    pub x: f32,
    pub y: f32,
}

impl azalea_buf::McBufWritable for DisplayInfo {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.title.write_into(buf)?;
        self.description.write_into(buf)?;
        self.icon.write_into(buf)?;
        self.frame.write_into(buf)?;

        let mut data: u32 = 0;
        if self.background.is_some() {
            data |= 0b1;
        }
        if self.show_toast {
            data |= 0b10;
        }
        if self.hidden {
            data |= 0b100;
        }
        data.write_into(buf)?;

        if let Some(background) = &self.background {
            background.write_into(buf)?;
        }
        self.x.write_into(buf)?;
        self.y.write_into(buf)?;
        Ok(())
    }
}
impl azalea_buf::McBufReadable for DisplayInfo {
    fn read_from(buf: &mut impl std::io::Read) -> Result<Self, azalea_buf::BufReadError> {
        let title = azalea_buf::McBufReadable::read_from(buf)?;
        let description = azalea_buf::McBufReadable::read_from(buf)?;
        let icon = azalea_buf::McBufReadable::read_from(buf)?;
        let frame = azalea_buf::McBufReadable::read_from(buf)?;

        let data = u32::read_from(buf)?;
        let has_background = (data & 0b1) != 0;
        let show_toast = (data & 0b10) != 0;
        let hidden = (data & 0b100) != 0;

        let background = if has_background {
            Some(ResourceLocation::read_from(buf)?)
        } else {
            None
        };
        let x = azalea_buf::McBufReadable::read_from(buf)?;
        let y = azalea_buf::McBufReadable::read_from(buf)?;
        Ok(DisplayInfo {
            title,
            description,
            icon,
            frame,
            background,
            x,
            y,
            hidden,
            show_toast,
        })
    }
}

#[derive(Clone, Debug, Copy, McBuf)]
pub enum FrameType {
    Task = 0,
    Challenge = 1,
    Goal = 2,
}

// nothing is written here
#[derive(Clone, Debug, McBuf)]
pub struct Criterion {}

pub type AdvancementProgress = HashMap<ResourceLocation, CriterionProgress>;

#[derive(Clone, Debug, McBuf)]
pub struct CriterionProgress {
    date: Option<u64>,
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use azalea_buf::{McBufReadable, McBufWritable};
//     use azalea_core::ResourceLocation;
//     use azalea_protocol_macros::ClientboundGamePacket;
//     use std::io::Cursor;

//     #[test]
//     fn test() {
//         let mut buf = Cursor::new(Vec::new());
//         let packet = ClientboundUpdateAdvancementsPacket {
//             reset: true,
//             added: [(
//                 ResourceLocation::new("minecraft:test").unwrap(),
//                 Advancement {
//                     parent_id: None,
//                     display: Some(DisplayInfo {
//                         title: Component::from("title".to_string()),
//                         description: Component::from("description".to_string()),
//                         icon: Slot::Empty,
//                         frame: FrameType::Task,
//                         show_toast: true,
//                         hidden: false,
//                         background: None,
//                         x: 0.0,
//                         y: 0.0,
//                     }),
//                     criteria: HashMap::new(),
//                     requirements: Vec::new(),
//                 },
//             )]
//             .into_iter()
//             .collect(),
//             removed: vec![ResourceLocation::new("minecraft:test2").unwrap()],
//             progress: [(
//                 ResourceLocation::new("minecraft:test3").unwrap(),
//                 [(
//                     ResourceLocation::new("minecraft:test4").unwrap(),
//                     CriterionProgress {
//                         date: Some(123456789),
//                     },
//                 )]
//                 .into_iter()
//                 .collect(),
//             )]
//             .into_iter()
//             .collect(),
//         };
//         packet.write_into(&mut buf).unwrap();
//         let packet = ClientboundUpdateAdvancementsPacket::read_from(&mut buf).unwrap();
//         assert_eq!(packet.reset, true);
//     }
// }
