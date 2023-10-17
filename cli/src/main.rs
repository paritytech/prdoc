//! Entry point of the cli. The cli itself does not contain much, it is mostly a shell around the [prdoclib].
mod opts;

use clap::{crate_name, crate_version, Parser};
use env_logger::Env;
use opts::*;
use prdoclib::{
	commands::{
		check::{CheckCmd, CheckResult},
		generate::GenerateCmd,
		load::LoadCmd,
		scan::ScanCmd,
		version::VersionCmd,
	},
	common::PRNumber,
	config::Config,
};
use std::{collections::HashSet, env, path::PathBuf};

/// Main entry point of the cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	color_eyre::install()?;

	let opts: Opts = Opts::parse();
	log::debug!("opts: {opts:#?}");

	// let config = Config::get_config_file(opts.config).unwrap();
	let config = match Config::load(opts.config) {
		Ok(c) => {
			log::debug!("Config found: {:#?}", c);
			c
		}
		Err(_e) => {
			log::warn!("No config could be found, using default");
			Config::get_default_config()
		}
	};

	let prdoc_dir: Vec<PathBuf> = match (config.prdoc_folder, opts.prdoc_folder) {
		(config_list, None) => config_list,
		(_config_list, Some(args)) => vec![args],
	};

	log::debug!("prdoc_dir: {:#?}", prdoc_dir);
	match opts.subcmd {
		Some(SubCommand::Generate(cmd_opts)) => {
			log::debug!("cmd_opts: {cmd_opts:#?}");
			let dir = match cmd_opts.output_dir {
				Some(d) => d,
				None => prdoclib::utils::get_pr_doc_folder().expect("Always have a default"),
			};
			log::debug!("PRDoc folder: {dir:?}");
			match GenerateCmd::run(!cmd_opts.dry_run, cmd_opts.number, cmd_opts.title, Some(dir)) {
				Ok(_) => Ok(()),
				Err(e) => {
					eprint!("{e:?}");
					std::process::exit(exitcode::IOERR);
				}
			}
		}

		Some(SubCommand::Check(cmd_opts)) => {
			log::debug!("cmd_opts: {cmd_opts:#?}");
			log::debug!("prdoc_dir: {prdoc_dir:?}");
			let results: HashSet<CheckResult> = prdoc_dir
				.iter()
				.flat_map(|d| {
					log::debug!("Checking files in {:#?}", d);
					CheckCmd::run(d, cmd_opts.file.clone(), cmd_opts.number.clone(), cmd_opts.list.clone()).unwrap()
				})
				.collect();

			if !opts.json {
				for (src, result) in &results {
					let pr_number: PRNumber = src.into();
					println!("PR #{pr_number: <4} -> {}", if *result { "OK " } else { "ERR" });
				}
			} else {
				let json = serde_json::to_string_pretty(&results).expect("We can serialize the result");
				println!("{json}");
			}

			let all_good = results.iter().map(|(_number, res)| res).all(|&res| res);
			if all_good {
				std::process::exit(exitcode::OK)
			} else {
				std::process::exit(exitcode::DATAERR)
			}
		}

		Some(SubCommand::Scan(cmd_opts)) => {
			log::debug!("cmd_opts: {cmd_opts:#?}");
			let files = ScanCmd::run(prdoc_dir, cmd_opts.all);
			let mut res: Vec<(Option<PRNumber>, PathBuf)> = files
				.iter()
				.map(|f| {
					let prdoc = LoadCmd::load_file(f);
					let n = match prdoc {
						Ok(p) => Some(p.filename.number),
						Err(_) => None,
					};

					(n, f.clone())
				})
				.collect();

			if cmd_opts.sort {
				res.sort_by(|a, b| a.1.cmp(&b.1));
			}

			if opts.json {
				println!("{}", serde_json::to_string_pretty(&res).unwrap());
			} else {
				res.iter().for_each(|(n, f)| {
					println!(
						"{number}\t{file}",
						file = f.display(),
						number = if let Some(number) = n { number.to_string() } else { "n/a".to_string() }
					);
				});
			}
			Ok(())
		}

		Some(SubCommand::Load(cmd_opts)) => {
			log::debug!("cmd_opts: {cmd_opts:#?}");

			let result = prdoc_dir
				.iter()
				.map(|dir| {
					LoadCmd::run(dir, cmd_opts.file.clone(), cmd_opts.number.clone(), cmd_opts.list.clone()).unwrap()
				})
				.fold((true, HashSet::new()), |(acc_status, acc_wrapper), (status, wrapper)| {
					let mut new_wrapper = acc_wrapper;
					new_wrapper.extend(wrapper);
					(acc_status && status, new_wrapper)
				});

			if opts.json {
				println!("{}", serde_json::to_string_pretty(&result.1).unwrap());
			} else {
				println!("{}", serde_yaml::to_string(&result.1).unwrap());
			}
			if result.0 {
				std::process::exit(exitcode::OK);
			} else {
				std::process::exit(exitcode::DATAERR)
			}
		}

		None => {
			if opts.version {
				let name = crate_name!();
				let version = crate_version!();
				VersionCmd::run(name, version, opts.json);
				Ok(())
			} else {
				unreachable!("We show help if there is no arg");
			}
		}
	}
}
