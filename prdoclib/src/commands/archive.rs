use std::path::{Path, PathBuf};
use log::debug;
use crate::common::PRNumber;
use crate::doc_filename::DocFileName;
use crate::docfile::DocFile;
use crate::error;
use crate::utils::{get_numbers_from_file, get_pr_doc_folder};

pub struct ArchiveCmd;

impl ArchiveCmd {
    /*pub fn run() {
        let res = numbers
            .iter()
            .filter_map(|&number| {
                log::debug!("Archiving PR #{}", number);

                let file_maybe = DocFileName::find(number, None, dir);

                match file_maybe {
                    Ok(file) => {
                        let filename = DocFileName::try_from(&file)
                            .expect("If we found a file, it should be valid");

                        todo!();
                    },
                    Err(e) => {
                        global_result &= false;
                        log::warn!("{e:?}");
                        None
                    },
                }
            })
            .collect();
    }*/

    pub(crate) fn archive_numbers(
        numbers: Vec<PRNumber>,
        dir: &PathBuf,
        output: &PathBuf,
        dry_run: bool,
    ) -> error::Result<()> {
        numbers
            .iter()
            .for_each(|&number| {
                log::debug!("Archive PR #{} to {output:?}", number);

                let file_maybe = DocFileName::find(number, None, dir);

                match file_maybe {
                    Ok(file) => {
                        let filename = DocFileName::try_from(&file)
                            .expect("If we found a file, it should be valid");

                        let file_path = PathBuf::from(filename.clone());
                        let _ = Self::archive_file(&file_path, output, dry_run);
                    }
                    Err(e) => {
                        log::warn!("{e:?}");
                    }
                }
            });

        Ok(())
    }

    /// Archive a file given its path and a destination folder
    pub(crate) fn archive_file(file: &PathBuf, folder: &PathBuf, dry_run: bool) -> crate::error::Result<()> {
        log::debug!("Archiving '{}' to '{}'", file.display(), folder.display());

        let target_folder = if folder.is_absolute() {
            get_pr_doc_folder().unwrap().join(folder.clone())
        } else {
            get_pr_doc_folder().unwrap().join(folder)
        };

        let source_file = if file.is_absolute() {
            file.clone()
        } else {
            get_pr_doc_folder().unwrap().join(file)
        };
        //let target = target_folder.join(file);

        let target_file = target_folder.join(file);
        if dry_run {
            println!("Moving '{}' to '{}'", source_file.display(), target_file.display());
        } else {
            let _ = std::fs::create_dir(&target_folder);

            let move_res = std::fs::rename(&source_file, &target_file);
            debug!("move_res: {move_res:?}");
        }
        Ok(())
    }

    pub(crate) fn archive_list(
        file: &PathBuf,
        dir: &PathBuf,
        folder: &PathBuf,
        dry_run: bool,
    ) -> crate::error::Result<()> {
        let extract_numbers = get_numbers_from_file(file)?;
        let numbers: Vec<PRNumber> =
            extract_numbers.iter().filter_map(|(_, _, n)| n.to_owned()).collect();

        //let mut global_result = extract_numbers.iter().map(|(_, status, _)| status).all(|&x| x);

        let _ = Self::archive_numbers(numbers, dir, folder, dry_run).unwrap();
        Ok(())
    }

    pub(crate) fn archive_from_folder(dir: &PathBuf) -> crate::error::Result<()> {
        let files = DocFile::find(dir, false);

        let _ = files
            .for_each(|file| {
                let filename_maybe = DocFileName::try_from(&file);
                println!("{filename_maybe:?}");
                /*	if let Ok(filename) = filename_maybe {
                        let yaml = Schema::load(&file);
                        if let Ok(value) = yaml {
                            let wrapper = DocFileWrapper::new(filename, value);

                            log::info!("OK  {}", file.display());
                            Some(wrapper)
                        } else {
                            log::warn!("ERR {}", file.display());
                            None
                        }
                    } else {
                        log::warn!("Invalid file {:?}", file.display());
                        None
                    }*/
            });

        Ok(())
    }


    pub fn run(
        dir: &PathBuf,
        file: Option<PathBuf>,
        numbers: Option<Vec<PRNumber>>,
        list: Option<PathBuf>,
        output: PathBuf,
        dry_run: bool,
    ) -> crate::error::Result<()> {
        let output_abs = if output.is_relative() { get_pr_doc_folder().unwrap().join(&output) } else { output.clone() };
        log::debug!("Archive from: {}", dir.display());
        log::debug!("Archive to  : {}", output_abs.display());

        let _ = match (file, numbers, list) {
            (Some(f), None, None) => {
                let file_abs = if f.is_relative() { Path::new(&dir).join(&f) } else { f.clone() };
                Self::archive_file(&file_abs, &output_abs, dry_run)?;
                ()
            }

            (None, Some(numbers), None) => {
                log::debug!("Archive numbers {:?}", numbers);
                let _ = Self::archive_numbers(numbers, dir, &output, dry_run)?;
                ()
            }

            (None, None, Some(list)) => {
                log::debug!("Archive list from {:?}", list);
                let _ = Self::archive_list(&list, dir, &output, dry_run)?;
                ()
            }

            /*(None, None, None) => {
                log::debug!("Archive all files in folder {}", dir.display());
                let (global_result, wrapper) = Self::archive_folder(dir)?;
                (Some(global_result), wrapper)
            },*/

            _ => unreachable!(),
        };
        Ok(())
    }
}
