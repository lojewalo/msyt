use crate::{
  Result,
  botw::Control,
};

use byteordered::Endian;
use indexmap::IndexMap;
use msbt::{Encoding, Header};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Msyt {
  pub entries: IndexMap<String, Vec<Content>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Content {
  Text(String),
  Control(Control),
}

impl Content {
  pub fn write_all(header: &Header, contents: &[Content]) -> Result<Vec<u8>> {
    let mut buf = Vec::new();

    for content in contents {
      match *content {
        Content::Text(ref s) => match header.encoding() {
          Encoding::Utf16 => {
            let mut inner_buf = [0; 2];
            let mut bytes: Vec<u8> = s.encode_utf16()
              .flat_map(|x| {
                header.endianness().write_u16(&mut inner_buf[..], x).expect("failed writing to array");
                inner_buf.to_vec()
              })
              .collect();
            buf.append(&mut bytes);
          }
          Encoding::Utf8 => buf.append(&mut s.as_bytes().to_vec()),
        },
        Content::Control(ref c) => c.write(header, &mut buf)?,
      }
    }

    // add \u0000
    buf.push(0);
    buf.push(0);

    Ok(buf)
  }
}
