use crate::botw::Control;

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
  Control(Control),
}

impl Content {
  pub fn combine_utf8(contents: &[Content]) -> Vec<u8> {
    let mut buf = Vec::new();

    for content in contents {
      match *content {
        Content::Text(ref s) => buf.append(&mut s.as_bytes().to_vec()),
        Content::Control(_) => unimplemented!("exporting with controls not implemented"),
      }
    }

    buf
  }

  pub fn combine_utf16(contents: &[Content]) -> Vec<u16> {
    let mut buf = Vec::new();

    for content in contents {
      match *content {
        Content::Text(ref s) => {
          let mut utf16_bytes: Vec<u16> = s.encode_utf16().collect();
          buf.append(&mut utf16_bytes);
        },
        Content::Control(_) => unimplemented!("exporting with controls not implemented"),
      }
    }

    buf
  }
}

