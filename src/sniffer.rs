use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, atomic::AtomicUsize},
};

use dashmap::DashMap;
use rayon::prelude::*;
use serde::Serialize;
use walkdir::WalkDir;

use crate::patterns::{ENV_VAR_TS_DEFAULT_PATTERN, FILE_EXTENSION};

pub struct ScanError(pub String);

#[derive(Debug, Serialize, Clone)]
pub struct EnvMetadata {
    key: Arc<str>,
    appearances: usize,
    places: Vec<Arc<str>>,
}

#[derive(Debug, Serialize)]
pub struct ScanResult {
    envs: Vec<Arc<EnvMetadata>>,
    processed_files: usize,
}

impl EnvMetadata {
    pub fn new(key: Arc<str>) -> Self {
        Self {
            key,
            appearances: 0,
            places: Vec::new(),
        }
    }

    pub fn register_place(&mut self, place: Arc<str>) {
        self.places.push(place);
        self.appearances += 1;
    }
}

impl ScanResult {
    fn new(envs: Vec<Arc<EnvMetadata>>, processed_files: usize) -> Self {
        Self {
            envs,
            processed_files,
        }
    }

    pub fn processed_files(&self) -> usize {
        self.processed_files
    }
}

fn filter_dir(entry: &walkdir::DirEntry) -> bool {
    entry.path().extension().is_some_and(|extension| {
        let ext_str = extension.to_str();
        if let Some(ext_str) = ext_str {
            FILE_EXTENSION.contains(&ext_str)
        } else {
            false
        }
    })
}

fn process_line(
    entry: &walkdir::DirEntry,
    line: &str,
    result: &DashMap<Arc<str>, Arc<EnvMetadata>>,
) {
    ENV_VAR_TS_DEFAULT_PATTERN
        .captures_iter(line)
        .for_each(|cap| {
            let var_name = &cap[1];
            let current_var_ammount = result.get_mut(var_name);
            let place = entry.path().to_str();

            match current_var_ammount {
                Some(mut value) => {
                    if let Some(place) = place {
                        Arc::make_mut(&mut value).register_place(Arc::from(place));
                    }
                }
                None => {
                    if let Some(place) = place {
                        let new_var: Arc<str> = Arc::from(var_name);
                        let mut new_env = EnvMetadata::new(new_var.clone());
                        new_env.register_place(Arc::from(place));
                        result.insert(new_var.clone(), Arc::from(new_env));
                    }
                }
            }
        })
}

fn for_each_file(
    entry: walkdir::DirEntry,
    result: &DashMap<Arc<str>, Arc<EnvMetadata>>,
    processed_files: &AtomicUsize,
    parallel: bool,
) {
    let file = File::open(entry.path());
    let reader = match file {
        Ok(file) => BufReader::new(file),
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };

    processed_files.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if parallel {
        reader.lines().par_bridge().for_each(|line| {
            if let Ok(line) = line {
                process_line(&entry, &line, result);
            }
        });
    } else {
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                process_line(&entry, &line, result);
            }
        });
    }
}

pub fn scan_folder(folder: &str, parallel: bool) -> Result<ScanResult, ScanError> {
    let result: DashMap<Arc<str>, Arc<EnvMetadata>> = DashMap::new();
    let processed_files = AtomicUsize::new(0);

    if parallel {
        WalkDir::new(folder)
            .into_iter()
            .par_bridge()
            .filter_map(|entry| entry.ok())
            .filter(filter_dir)
            .for_each(|entry| for_each_file(entry, &result, &processed_files, true));
    } else {
        WalkDir::new(folder)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(filter_dir)
            .for_each(|entry| for_each_file(entry, &result, &processed_files, false));
    }

    let final_result = result
        .iter()
        .map(|entry| entry.value().clone())
        .collect::<Vec<_>>();
    let processed_files = processed_files.load(std::sync::atomic::Ordering::Relaxed);
    drop(result);

    Ok(ScanResult::new(final_result, processed_files))
}
