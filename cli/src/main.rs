//! todo
mod opts;

use clap::{crate_name, crate_version, Parser};
use color_eyre::eyre::{bail, Context};
use env_logger::Env;
use log::*;
use opts::*;
use serde_json::json;
use std::{
	env,
	path::{Path, PathBuf},
};

use prdoclib::{docfile::*, docfilename::DocFileName, schema::Schema};

/// Main entry point of the cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	color_eyre::install()?;

	let opts: Opts = Opts::parse();
	// debug!("opts: {opts:#?}");

	match opts.subcmd {
		Some(SubCommand::Generate(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");

			// generate doc
			let template = DocFile::generate();

			// print to stdout or save to file
			if !cmd_opts.save {
				debug!("Printing to stdout only, use --save to save to a file");
				println!("{}", &template);
				Ok(())
			} else {
				// generate filename based on number and title
				let filename: PathBuf = DocFileName::new(cmd_opts.number, cmd_opts.title).into();
				let output_file = Path::new(&cmd_opts.output_dir).join(filename);
				debug!("template = {:?}", &template);
				debug!("output_file = {:?}", &output_file);
				std::fs::write(output_file, template).with_context(|| "Unable to write template to {output_file:?}")
			}
		}

		Some(SubCommand::Check(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			let dir = cmd_opts.directory;
			debug!("Checking directory {}", dir.display());

			if let Some(file) = cmd_opts.file {
				let file = if file.is_relative() { Path::new(&dir).join(&file) } else { file.clone() };

				debug!("Checking file {}", file.display());

				// todo: DEDUP that
				let result = Schema::check(&file);
				if result {
					println!("OK  {}", file.display());
					std::process::exit(exitcode::OK);
				} else {
					eprintln!("ERR {}", file.display());
					std::process::exit(exitcode::DATAERR);
				}
			}

			if let Some(number) = cmd_opts.number {
				debug!("Checking PR #{}", number);

				let search = DocFileName::find(number, None, &dir);

				let file_maybe = match search {
					Ok(f) => f,
					Err(e) => {
						eprintln!("e = {:?}", e);
						std::process::exit(exitcode::DATAERR)
					}
				};

				if let Some(file) = file_maybe {
					debug!("Checking file {}", file.display());

					// todo: DEDUP that
					let result = Schema::check(&file);
					if result {
						println!("OK  {}", file.display());
						std::process::exit(exitcode::OK);
					} else {
						eprintln!("ERR {}", file.display());
						std::process::exit(exitcode::DATAERR);
					}
				} else {
					bail!("No file found");
				}
			}

			if cmd_opts.number.is_none() && cmd_opts.file.is_none() {
				debug!("Checking all files in folder {}", dir.display());
				let res = DocFile::find(&dir, false);
				let mut global_result = true;

				res.for_each(|file| {
					// todo: DEDUP that
					let result = Schema::check(&file);
					if result {
						global_result &= true;
						println!("OK  {}", file.display());
					} else {
						global_result &= false;
						eprintln!("ERR {}", file.display());
					}
				});

				if global_result {
					std::process::exit(exitcode::OK);
				} else {
					std::process::exit(exitcode::DATAERR);
				}
			}

			Ok(())
		}

		Some(SubCommand::Scan(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			let res = DocFile::find(&cmd_opts.directory, !cmd_opts.all);
			res.for_each(|hit| {
				println!("{}", hit.display());
			});
			Ok(())
		}

		Some(SubCommand::Load(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			todo!();
			Ok(())
		}

		Some(SubCommand::Schema(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			let schema = Schema::get(true);
			println!("{schema}");
			Ok(())
		}

		None => {
			if opts.version {
				let name = crate_name!();
				let version = crate_version!();
				let commit_hash = env::var("PRDOC_CLI_GIT_COMMIT_HASH");
				let build_date = env::var("PRDOC_CLI_BUILD_DATE");

				if !opts.json {
					let commit_hash_str = if let Ok(s) = commit_hash { format!("-{s}") } else { String::from("") };
					let build_date_str = if let Ok(s) = build_date { format!(" built {s}") } else { String::from("") };
					println!("{name} v{version}{commit_hash_str}{build_date_str}");
				} else {
					let version_data = json!({
						"name": name,
						"version": version,
						"commit": commit_hash.unwrap_or_default(),
						"build_date": build_date.unwrap_or_default(),
					});
					let s = serde_json::to_string_pretty(&version_data).expect("serde_json ran into issues");
					println!("{s}");
				}
				Ok(())
			} else {
				unreachable!("We show help if there is no arg");
			}
		}
	}
}
