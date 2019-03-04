use itertools::Itertools;
use msbt::Msbt;
use serde_derive::{Deserialize, Serialize};
use indexmap::IndexMap;

use std::{
  fs::File,
  io::{BufReader, BufWriter},
};

fn main() {
  for path in std::env::args().skip(1) {
    let msbt_file = File::open(&path).unwrap();
    let msbt = Msbt::from_reader(BufReader::new(msbt_file)).unwrap();

    let lbl1 = msbt.lbl1.unwrap();

    let mut entries = IndexMap::with_capacity(lbl1.labels.len());

    for label in lbl1.labels {
      let mut all_content = Vec::new();

      let grouped = label.value
        .as_bytes()
        .iter()
        .group_by(|x| x.is_ascii_alphanumeric() || x.is_ascii_punctuation() || x.is_ascii_whitespace());
      for (is_ascii, part) in &grouped {
        let bytes: Vec<u8> = part.cloned().collect();
        let content = if is_ascii {
          Content::Ascii(unsafe { String::from_utf8_unchecked(bytes) })
        } else {
          Content::Bytes(bytes)
        };
        all_content.push(content);
      }

      entries.insert(label.name, all_content);
    }

    let msxt = Msxt { entries };

    let base = path.rsplitn(2, '.').nth(1).unwrap();
    serde_yaml::to_writer(
      BufWriter::new(File::create(format!("{}.msyt", base)).unwrap()),
      &msxt,
    ).unwrap();
  }
}

#[derive(Debug, Deserialize, Serialize)]
struct Msxt {
  entries: IndexMap<String, Vec<Content>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Content {
  Ascii(String),
  Bytes(Vec<u8>),
}
