//! Entry point of the cli. The cli itself does not contain much, it is mostly a shell around the [prdoclib].
#![warn(missing_docs)]

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
	prdoc_source::PRDocSource,
	prdoc_source::PRDocSource::File,
	schema::Schema,
};
use std::cmp::Ordering;
use std::{collections::HashSet, env, path::PathBuf};

/// Main entry point of the cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
	color_eyre::install()?;

	let opts: Opts = Opts::parse();
	log::debug!("opts: {opts:#?}");

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

	let prdoc_dir: Vec<PathBuf> = match (&config.prdoc_folders, opts.prdoc_folders) {
		(config_list, None) => config_list.clone(),
		(_config_list, Some(args)) => vec![args],
	};

	log::debug!("prdoc_dir: {:#?}", prdoc_dir);

	match opts.subcmd {
		Some(SubCommand::Generate(cmd_opts)) => {
			log::debug!("cmd_opts: {cmd_opts:#?}");
			let dir = prdoclib::utils::get_pr_doc_folder(cmd_opts.output_dir, &config);
			let template_path = prdoclib::utils::get_template_path(&config);

			log::debug!("PRDoc folder: {dir:?}");
			match GenerateCmd::run(cmd_opts.dry_run, cmd_opts.number, None, Some(dir), template_path) {
				Ok(_) => Ok(()),
				Err(e) => {
					log::error!("{e}");
					std::process::exit(exitcode::IOERR);
				}
			}
		}

		Some(SubCommand::Check(cmd_opts)) => {
			log::debug!("cmd_opts: {cmd_opts:#?}");
			let mut results: Vec<CheckResult> = prdoc_dir
				.iter()
				.flat_map(|dir| {
					CheckCmd::run(
						&config,
						cmd_opts.schema.clone(),
						dir,
						cmd_opts.file.clone(),
						cmd_opts.number.clone(),
						cmd_opts.list.clone(),
					)
					.unwrap()
				})
				.collect();

			results.sort_by(|a, b| match (&a.0, &b.0) {
				(File(path_a), File(path_b)) => path_a.cmp(path_b),
				(PRDocSource::Number(num_a), PRDocSource::Number(num_b))
				| (PRDocSource::Both(_, num_a), PRDocSource::Both(_, num_b)) => num_a.cmp(num_b),
				_ => Ordering::Greater,
			});

			if !opts.json {
				for (src, result) in &results {
					if *result {
						continue;
					}

					let pr_number: PRNumber = src.into();
					println!("PR #{pr_number: <4} -> ERR");
				}

				let plural_s = if results.len() > 1 { "s" } else { "" };
				println!("Checked {} file{plural_s}.", results.len());
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
			let schema_path = config.schema_path();
			let schema = Schema::new(schema_path);

			log::debug!("cmd_opts: {cmd_opts:#?}");
			let files = ScanCmd::run(schema.clone(), prdoc_dir, cmd_opts.all);
			let load_cmd = LoadCmd::new(schema);

			let mut res: Vec<(Option<PRNumber>, PathBuf)> = files
				.iter()
				.map(|f| {
					let prdoc = load_cmd.load_file(f);
					let n = match prdoc {
						Ok(p) => Some(p.doc_filename.number),
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
					LoadCmd::run(
						&config,
						cmd_opts.schema.clone(),
						dir,
						cmd_opts.file.clone(),
						cmd_opts.number.clone(),
						cmd_opts.list.clone(),
					)
					.unwrap()
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
