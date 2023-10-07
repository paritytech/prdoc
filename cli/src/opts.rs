//! This module contains the definition of all commands, sub-commands and arguments
//! supported by the cli.

use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};
use prdoclib::{common::PRNumber, title::Title};
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

	// /// Less output
	// #[clap(short, long, global = true, display_order = 99)]
	// pub quiet: bool,

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

	#[allow(missing_docs)]
	#[clap(version = crate_version!(), author = crate_authors!())]
	Schema(SchemaOpts),
}
/// Generate a new file. It will be printed to stdout by default unless you provide
/// the `--save` flag.
#[derive(Parser, Debug)]
pub struct GenOpts {
	/// Change number
	#[clap(index = 1)]
	pub number: PRNumber,

	/// Change title
	#[clap(short, long)]
	pub title: Option<Title>,

	/// Save the generated document to file with the proper naming
	#[clap(short, long)]
	pub save: bool,

	/// Optional output directory. It not passed, the default `PRDOC_DIR` will be used
	/// under the root of the current project.
	#[clap(short, long)]
	pub output_dir: Option<PathBuf>,
}

/// Check one or MORE `prdoc` files for validity.
#[derive(Parser, Debug)]
pub struct CheckOpts {
	/// Base directory for the files. Default to folder `PRDOC_DIR` under the root of your project.
	#[clap(short, long)]
	pub directory: Option<PathBuf>,

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
	/// Base directory for the files. Default to folder `PRDOC_DIR` under the root of your project.
	#[clap(index = 1)]
	pub directory: Option<PathBuf>,

	/// Also return invalid files
	#[clap(short, long)]
	pub all: bool,
}

/// Load one or more prdoc
#[derive(Parser, Debug)]
pub struct LoadOpts {
	/// Base directory for the files. Default to folder `PRDOC_DIR` under the root of your project.
	#[clap(short, long)]
	pub directory: Option<PathBuf>,

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

/// Retrieve the JSON schema that is used internally
#[derive(Parser, Debug)]
pub struct SchemaOpts {}
