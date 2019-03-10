use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Seek, SeekFrom};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_9 {
  unknown_1: Option<[u8; 12]>,
  strings: [Control1_9String; 4],
  field_4: u16,
  field_5: u16,
  unknown_2: Option<[u8; 12]>,
  field_6: [u8; 2],
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Control1_9String {
  field_1: u16,
  string: String,
}

const UNKNOWN: [u8; 12] = [255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0];

impl Control1_9 {
  pub(crate) fn parse(endianness: Endianness, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let mut unknown_buf = [0; 12];

    let payload_length = endianness.read_u16(&mut reader)?;

    reader.read_exact(&mut unknown_buf[..])?;
    let unknown_1 = if unknown_buf == UNKNOWN {
      Some(unknown_buf)
    } else {
      reader.seek(SeekFrom::Current(-12))?;
      None
    };

    let mut strings = [
      Control1_9String::default(),
      Control1_9String::default(),
      Control1_9String::default(),
      Control1_9String::default(),
    ];
    for cstring in strings.iter_mut() {
      let field_1 = endianness.read_u16(&mut reader)?;
      let str_len = endianness.read_u16(&mut reader)?;

      let mut str_bytes = vec![0; str_len as usize];
      reader.read_exact(&mut str_bytes)?;

      // FIXME: check if file is utf-8 or utf-16
      let utf16_str: Vec<u16> = str_bytes.chunks(2)
        .map(|bs| endianness.read_u16(bs).map_err(Into::into))
        .collect::<Result<_>>()?;

      let string = String::from_utf16(&utf16_str)?;

      *cstring = Control1_9String {
        field_1,
        string,
      };
    }

    let field_4 = endianness.read_u16(&mut reader)?;
    let field_5 = endianness.read_u16(&mut reader)?;

    let unknown_2 = if reader.get_ref().len() > reader.position() as usize + 12 {
      reader.read_exact(&mut unknown_buf[..])?;
      if unknown_buf == UNKNOWN {
        Some(unknown_buf)
      } else {
        reader.seek(SeekFrom::Current(-12))?;
        None
      }
    } else {
      None
    };

    let mut field_6 = [0; 2];
    reader.read_exact(&mut field_6)?;

    debug_assert_eq!(u64::from(payload_length), reader.position() - 4);

    Ok(Control1_9 {
      unknown_1,
      strings,
      field_4,
      field_5,
      unknown_2,
      field_6,
    })
  }
}
