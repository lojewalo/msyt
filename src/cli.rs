use clap::{App, AppSettings, Arg, SubCommand};

pub fn app<'a, 'b: 'a>() -> App<'a, 'b> {
  App::new(clap::crate_name!())
    .version(clap::crate_version!())
    .author(clap::crate_authors!())
    .about(clap::crate_description!())

    .settings(&[
      AppSettings::SubcommandRequiredElseHelp,
      AppSettings::DeriveDisplayOrder,
      AppSettings::VersionlessSubcommands,
    ])

    .subcommand(SubCommand::with_name("import")
      .about("Import from MSYT files to MSBT files")

      .arg(Arg::with_name("paths")
        .help("MSYT paths to import (MSBT files should be adjacent)")
        .required(true)
        .multiple(true)))
    .subcommand(SubCommand::with_name("export")
      .about("Export from MSBT files to MSYT files")

      .arg(Arg::with_name("paths")
        .help("MSBT paths to export")
        .required(true)
        .multiple(true)))
}
