//! This module contains the definition of all commands, sub-commands and arguments
//! supported by the cli.

use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};
use prdoclib::{common::PRNumber, config};
use std::path::PathBuf;

/// prdoc is a cli utility to generate, check and load prdoc files.
///
/// More at <https://github.com/paritytech/prdoc>
#[derive(Parser, Debug)]
#[clap(color=ColorChoice::Auto, disable_version_flag = true, arg_required_else_help = true )]
pub struct Opts {
	/// Output as json
	#[clap(short, long, global = true, display_order = 99)]
	pub json: bool,

	#[clap(short, long, global = true, env = config::env::PRDOC_CONFIG)]
	pub config: Option<PathBuf>,

	#[clap(short = 'd', alias = "dir", long, global = true, env = config::env::PRDOC_FOLDER)]
	pub prdoc_folders: Option<PathBuf>,

	// /// Do not write color information to the output. This is recommended for scripts.
	// #[clap(short, long, global = true, env = "NO_COLOR", display_order = 99)]
	// pub no_color: bool,
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
	Generate(GenOpts),

	#[allow(missing_docs)]
	#[clap(alias = "validate", version = crate_version!(), author = crate_authors!())]
	Check(CheckOpts),

	#[allow(missing_docs)]
	#[clap(version = crate_version!(), author = crate_authors!())]
	Scan(ScanOpts),

	#[allow(missing_docs)]
	#[clap(version = crate_version!(), author = crate_authors!())]
	Load(LoadOpts),
}
/// Generate a new file. It will be saved by default unless you provide --dry-run.
#[derive(Parser, Debug)]
pub struct GenOpts {
	/// Change number
	#[clap(index = 1)]
	pub number: PRNumber,

	// Removed for now as it brings confusion between title in the filename
	// and the actual title in the prdoc file itself.
	// /// Change title
	// #[clap(short, long)]
	// pub title: Option<Title>,
	/// Do not save the generated document to file with the proper naming, show the content
	/// instead
	#[clap(long)]
	pub dry_run: bool,

	/// Optional output directory. It not passed, the default `PRDOC_DIR` will be used
	/// under the root of the current project.
	#[clap(short, long)]
	pub output_dir: Option<PathBuf>,
}

/// Check one or MORE `prdoc` files for validity.
#[derive(Parser, Debug)]
pub struct CheckOpts {
	/// Directly specify the file to be checked. It can be relative to the base directory.
	#[clap(short, long, conflicts_with = "number")]
	pub file: Option<PathBuf>,

	/// number
	#[clap(short, long)]
	pub number: Option<Vec<PRNumber>>,

	/// Get the list of PR numbers from a file
	#[clap(short, long, conflicts_with_all = ["file", "number"])]
	pub list: Option<PathBuf>,
}

/// Scan a directory for prdoc files based on their name
#[derive(Parser, Debug)]
pub struct ScanOpts {
	// /// directory path
	// #[clap(index = 1, default_value = ".")]
	// pub directory: PathBuf,
	/// Also return invalid files
	#[clap(short, long)]
	pub all: bool,

	/// Sort the output
	#[clap(short, long)]
	pub sort: bool,
}

/// Load one or more prdoc
#[derive(Parser, Debug)]
pub struct LoadOpts {
	// /// directory path
	// #[clap(short, long, default_value = ".")]
	// pub directory: PathBuf,
	/// file path
	#[clap(short, long, conflicts_with = "number")]
	pub file: Option<PathBuf>,

	/// One or more PR numbers.
	/// Depending on the host OS, the max length of a command may differ. If you run into issues, make sure to check the
	/// `--list` option instead.
	#[clap(short, long)]
	pub number: Option<Vec<PRNumber>>,

	/// Get the list of PR numbers from a file
	#[clap(short, long, conflicts_with_all = ["file", "number"])]
	pub list: Option<PathBuf>,
}
