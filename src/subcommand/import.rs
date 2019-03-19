use clap::ArgMatches;
use msbt::Msbt;
use rayon::prelude::*;

use std::{
  fs::File,
  io::{BufReader, BufWriter},
  path::PathBuf,
};

use crate::{
  Result,
  model::{Msyt, Content},
  subcommand::find_files,
};

pub fn import(matches: &ArgMatches) -> Result<()> {
  let paths: Vec<PathBuf> = if matches.is_present("dir_mode") {
    find_files(matches.values_of("paths").expect("required argument"), "msyt")?
  } else {
    matches.values_of("paths").expect("required argument").map(PathBuf::from).collect()
  };

  let extension = matches.value_of("extension").expect("clap arg with default");
  let backup = !matches.is_present("no-backup");

  paths
    .into_par_iter()
    .map(|path| {
      let msyt_file = File::open(&path)?;
      let msyt: Msyt = serde_yaml::from_reader(BufReader::new(msyt_file))?;

      let msbt_path = path.with_extension("msbt");
      let msbt_file = File::open(&msbt_path)?;

      let mut msbt = Msbt::from_reader(BufReader::new(msbt_file))?;

      for (key, contents) in msyt.entries {
        let new_val = Content::write_all(msbt.header(), &contents)?;
        if let Some(ref mut lbl1) = msbt.lbl1_mut() {
          if let Some(label) = lbl1.labels_mut().iter_mut().find(|x| x.name() == key) {
            if let Err(()) = label.set_value_raw(new_val) {
              failure::bail!("could not set raw string at index {}", label.index());
            }
          }
        }
      }

      let dest_path = path.with_extension(extension);

      if backup && dest_path.exists() {
        let backup_path = dest_path.with_extension(format!("{}.bak", extension));
        std::fs::rename(&dest_path, backup_path)?;
      }

      let new_msbt = File::create(&dest_path)?;
      msbt.write_to(BufWriter::new(new_msbt))?;

      Ok(())
    })
    .collect::<Result<_>>()
}
