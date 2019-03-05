use msbt::Msbt;

use std::{
  collections::{HashMap, HashSet},
  fs::File,
  io::BufReader,
};

fn main() {
  let mut all_types = HashSet::new();
  let mut examples: HashMap<u16, Vec<Vec<u16>>> = HashMap::new();

  for path in std::env::args().skip(1) {
    let f = File::open(path).unwrap();
    let msbt = Msbt::from_reader(BufReader::new(f)).unwrap();

    let lbl1 = msbt.lbl1.unwrap();
    for label in lbl1.labels {
      let u16s: Vec<u16> = label.value.encode_utf16().collect();

      let mut last_was_marker = false;
      let mut had_marker = false;

      let mut buf = Vec::new();

      for u in u16s {
        if u == 0x0e {
          buf.clear();
          last_was_marker = true;
          had_marker = true;
        } else if last_was_marker {
          all_types.insert(u);
          last_was_marker = false;
        } else {
          last_was_marker = false;
        }
        buf.push(u);
      }

      if had_marker && buf.len() > 1 {
        let entry = examples.entry(buf[1]).or_default();
        if entry.len() < 5 {
          entry.push(buf.clone());
        }
      }
    }
  }

  let mut all_types: Vec<u16> = all_types.into_iter().collect();
  all_types.sort_unstable();
  println!("{:#?}", all_types);
  for t in all_types {
    if examples.contains_key(&t) {
      println!("{}:", t);
      for example in &examples[&t] {
        println!("  {:?}", example);
      }
      println!();
    }
  }
}
