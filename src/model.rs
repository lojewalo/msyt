use crate::Result;

use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Msyt {
  pub entries: IndexMap<String, Vec<Content>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Content {
  Text(String),
  Utf16Bytes(Vec<u16>),
  Utf8Bytes(Vec<u8>),
}

impl Content {
  pub fn combine_utf8(contents: &[Content]) -> Result<Vec<u8>> {
    let mut buf = Vec::new();

    for content in contents {
      match *content {
        Content::Text(ref s) => buf.append(&mut s.as_bytes().to_vec()),
        Content::Utf8Bytes(ref b) => buf.append(&mut b.to_vec()),
        _ => failure::bail!("utf16 bytes in utf8 file"),
      }
    }

    Ok(buf)
  }

  pub fn combine_utf16(contents: &[Content]) -> Result<Vec<u16>> {
    let mut buf = Vec::new();

    for content in contents {
      match *content {
        Content::Text(ref s) => {
          let mut utf16_bytes: Vec<u16> = s.encode_utf16().collect();
          buf.append(&mut utf16_bytes);
        },
        Content::Utf16Bytes(ref b) => buf.append(&mut b.to_vec()),
        _ => failure::bail!("utf8 bytes in utf16 file"),
      }
    }

    Ok(buf)
  }
}

