use byteordered::Endianness;
use clap::ArgMatches;
use msbt::{
  Encoding,
  builder::MsbtBuilder,
};
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

pub fn create(matches: &ArgMatches) -> Result<()> {
  let paths: Vec<PathBuf> = if matches.is_present("dir_mode") {
    find_files(matches.values_of("paths").expect("required argument"), "msyt")?
  } else {
    matches.values_of("paths").expect("required argument").map(PathBuf::from).collect()
  };

  let endianness = match matches.value_of("platform").expect("required clap arg") {
    "switch" => Endianness::Little,
    "wiiu" => Endianness::Big,
    _ => unreachable!("clap arg with possible values"),
  };
  let encoding = match matches.value_of("encoding").expect("clap arg with default") {
    "utf16" => Encoding::Utf16,
    "utf8" => Encoding::Utf8,
    _ => unreachable!("clap arg with possible values"),
  };
  let extension = matches.value_of("extension").expect("clap arg with default");
  let backup = !matches.is_present("no-backup");

  paths
    .into_par_iter()
    .map(|path| {
      let msyt_file = File::open(&path)?;
      let msyt: Msyt = serde_yaml::from_reader(BufReader::new(msyt_file))?;

      let mut builder = MsbtBuilder::new(endianness, encoding, Some(msyt.group_count));
      if let Some(unknown_bytes) = msyt.ato1 {
        builder = builder.ato1(msbt::section::Ato1::new_unlinked(unknown_bytes));
      }
      if let Some(atr1) = msyt.atr1 {
        builder = builder.atr1(msbt::section::Atr1::new_unlinked(atr1.string_count, atr1._unknown_1, atr1.strings));
      }
      if let Some(unknown_bytes) = msyt.tsy1 {
        builder = builder.tsy1(msbt::section::Tsy1::new_unlinked(unknown_bytes));
      }
      if let Some(nli1) = msyt.nli1 {
        builder = builder.nli1(msbt::section::Nli1::new_unlinked(nli1.id_count, nli1.global_ids));
      }
      for (label, contents) in msyt.entries.into_iter() {
        let new_val = Content::write_all(builder.header(), &contents)?;
        builder = builder.add_label(label, new_val);
      }
      let msbt = builder.build();

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
