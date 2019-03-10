use crate::Result;

use byteordered::Endian;

use msbt::{Encoding, Header};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control2Variable {
  field_1: u16,
  string: String,
  field_3: u16,
}

impl Control2Variable {
  pub(crate) fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let field_1 = header.endianness().read_u16(&mut reader)?;
    let str_len = header.endianness().read_u16(&mut reader)?;

    let mut str_bytes = vec![0; str_len as usize];
    reader.read_exact(&mut str_bytes)?;

    let string = match header.encoding() {
      Encoding::Utf16 => {
        let utf16_str: Vec<u16> = str_bytes.chunks(2)
          .map(|bs| header.endianness().read_u16(bs).map_err(Into::into))
          .collect::<Result<_>>()?;
        String::from_utf16(&utf16_str)?
      },
      Encoding::Utf8 => String::from_utf8(str_bytes)?,
    };

    let field_3 = header.endianness().read_u16(&mut reader)?;

    Ok(Control2Variable {
      field_1,
      string,
      field_3,
    })
  }

  pub(crate) fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1)?;

    let str_bytes = match header.encoding() {
      Encoding::Utf16 => {
        let mut buf = [0; 2];
        self.string.encode_utf16()
          .flat_map(|x| {
            header.endianness().write_u16(&mut buf[..], x).expect("failed to write to array");
            buf.to_vec()
          })
          .collect()
      },
      Encoding::Utf8 => self.string.as_bytes().to_vec(),
    };

    header.endianness().write_u16(&mut writer, str_bytes.len() as u16)?;
    writer.write_all(&str_bytes)?;

    header.endianness().write_u16(&mut writer, self.field_3)?;

    Ok(())
  }
}
