//! This module contains the definition of all commands, sub-commands and arguments
//! supported by the cli.

use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(color=ColorChoice::Auto, disable_version_flag = true, arg_required_else_help = true )]
pub struct Opts {
	/// Output as json
	#[clap(short, long, global = true, display_order = 99)]
	pub json: bool,

	/// Less output
	#[clap(short, long, global = true, display_order = 99)]
	pub quiet: bool,

	/// Do not write color information to the output. This is recommended for scripts.
	#[clap(short, long, global = true, env = "NO_COLOR", display_order = 99)]
	pub no_color: bool,

	#[allow(missing_docs)]
	#[clap(subcommand)]
	pub subcmd: Option<SubCommand>,

	/// Show the version
	#[clap(short, long, alias = "V")]
	pub version: bool,
}

/// Define the list of all sub-commands.
#[derive(Subcommand, Debug)]
pub enum SubCommand {
	#[allow(missing_docs)]
	#[clap(version = crate_version!(), author = crate_authors!())]
	Scan(ScanOpts),
}

/// Scan a directory
#[derive(Parser, Debug)]
pub struct ScanOpts {
    /// directory path
	#[clap(index = 1, default_value = ".")]
	pub directory: Option<PathBuf>,
}
