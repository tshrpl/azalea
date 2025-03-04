use crate::Error;
use crate::Tag;
use ahash::AHashMap;
use azalea_buf::{BufReadError, McBufReadable};
use byteorder::{ReadBytesExt, BE};
use flate2::read::{GzDecoder, ZlibDecoder};
use std::io::{BufRead, Read};

#[inline]
fn read_string(stream: &mut impl Read) -> Result<String, Error> {
    let length = stream.read_u16::<BE>()?;

    let mut buf = vec![0; length as usize];
    stream.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

impl Tag {
    #[inline]
    fn read_known(stream: &mut impl Read, id: u8) -> Result<Tag, Error> {
        Ok(match id {
            // Signifies the end of a TAG_Compound. It is only ever used inside
            // a TAG_Compound, and is not named despite being in a TAG_Compound
            0 => Tag::End,
            // A single signed byte
            1 => Tag::Byte(stream.read_i8()?),
            // A single signed, big endian 16 bit integer
            2 => Tag::Short(stream.read_i16::<BE>()?),
            // A single signed, big endian 32 bit integer
            3 => Tag::Int(stream.read_i32::<BE>()?),
            // A single signed, big endian 64 bit integer
            4 => Tag::Long(stream.read_i64::<BE>()?),
            // A single, big endian IEEE-754 single-precision floating point
            // number (NaN possible)
            5 => Tag::Float(stream.read_f32::<BE>()?),
            // A single, big endian IEEE-754 double-precision floating point
            // number (NaN possible)
            6 => Tag::Double(stream.read_f64::<BE>()?),
            // A length-prefixed array of signed bytes. The prefix is a signed
            // integer (thus 4 bytes)
            7 => {
                let length = stream.read_u32::<BE>()?;
                let mut bytes = vec![0; length as usize];
                stream.read_exact(&mut bytes)?;
                Tag::ByteArray(bytes)
            }
            // A length-prefixed modified UTF-8 string. The prefix is an
            // unsigned short (thus 2 bytes) signifying the length of the
            // string in bytes
            8 => Tag::String(read_string(stream)?),
            // A list of nameless tags, all of the same type. The list is
            // prefixed with the Type ID of the items it contains (thus 1
            // byte), and the length of the list as a signed integer (a further
            // 4 bytes). If the length of the list is 0 or negative, the type
            // may be 0 (TAG_End) but otherwise it must be any other type. (The
            // notchian implementation uses TAG_End in that situation, but
            // another reference implementation by Mojang uses 1 instead;
            // parsers should accept any type if the length is <= 0).
            9 => {
                let type_id = stream.read_u8()?;
                let length = stream.read_i32::<BE>()?;
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    list.push(Tag::read_known(stream, type_id)?);
                }
                Tag::List(list)
            }
            // Effectively a list of a named tags. Order is not guaranteed.
            10 => {
                // we default to capacity 4 because it'll probably not be empty
                let mut map = AHashMap::with_capacity(4);
                loop {
                    let tag_id = stream.read_u8().unwrap_or(0);
                    if tag_id == 0 {
                        break;
                    }
                    let name = read_string(stream)?;
                    let tag = Tag::read_known(stream, tag_id)?;
                    map.insert(name, tag);
                }
                Tag::Compound(map)
            }
            // A length-prefixed array of signed integers. The prefix is a
            // signed integer (thus 4 bytes) and indicates the number of 4 byte
            // integers.
            11 => {
                let length = stream.read_u32::<BE>()?;
                let mut ints = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    ints.push(stream.read_i32::<BE>()?);
                }
                Tag::IntArray(ints)
            }
            // A length-prefixed array of signed longs. The prefix is a signed
            // integer (thus 4 bytes) and indicates the number of 8 byte longs.
            12 => {
                let length = stream.read_u32::<BE>()?;
                let mut longs = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    longs.push(stream.read_i64::<BE>()?);
                }
                Tag::LongArray(longs)
            }
            _ => return Err(Error::InvalidTagType(id)),
        })
    }

    pub fn read(stream: &mut impl Read) -> Result<Tag, Error> {
        // default to compound tag

        // the parent compound only ever has one item
        let tag_id = stream.read_u8().unwrap_or(0);
        if tag_id == 0 {
            return Ok(Tag::End);
        }
        let name = read_string(stream)?;
        let tag = Tag::read_known(stream, tag_id)?;
        let mut map = AHashMap::with_capacity(1);
        map.insert(name, tag);

        Ok(Tag::Compound(map))
    }

    pub fn read_zlib(stream: &mut impl BufRead) -> Result<Tag, Error> {
        let mut gz = ZlibDecoder::new(stream);
        Tag::read(&mut gz)
    }

    pub fn read_gzip(stream: &mut impl Read) -> Result<Tag, Error> {
        let mut gz = GzDecoder::new(stream);
        Tag::read(&mut gz)
    }
}

impl McBufReadable for Tag {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        Ok(Tag::read(buf)?)
    }
}
impl From<Error> for BufReadError {
    fn from(e: Error) -> Self {
        BufReadError::Custom(e.to_string())
    }
}
