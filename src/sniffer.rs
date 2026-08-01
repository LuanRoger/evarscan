use std::sync::atomic::AtomicUsize;

use dashmap::DashMap;
use rayon::prelude::*;
use serde::Serialize;
use walkdir::WalkDir;

use crate::patterns::{ENV_VAR_TS_DEFAULT_PATTERN, FILE_EXTENSION};

pub struct ScanError(pub String);

#[derive(Debug, Serialize, Clone)]
pub struct EnvMetadata {
    key: String,
    appearances: usize,
    places: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ScanResult {
    envs: Vec<EnvMetadata>,
    processed_files: usize,
}

impl EnvMetadata {
    pub fn new(key: String) -> Self {
        Self {
            key,
            appearances: 0,
            places: Vec::new(),
        }
    }

    pub fn register_place(&mut self, place: String) {
        self.places.push(place);
        self.appearances += 1;
    }
}

impl ScanResult {
    fn new(envs: Vec<EnvMetadata>, processed_files: usize) -> Self {
        Self {
            envs,
            processed_files,
        }
    }

    pub fn processed_files(&self) -> usize {
        self.processed_files
    }

    pub fn envs(&self) -> &[EnvMetadata] {
        &self.envs
    }
}

pub fn scan_folder(folder: &str) -> Result<ScanResult, ScanError> {
    let result: DashMap<String, EnvMetadata> = DashMap::new();
    let processed_files = AtomicUsize::new(0);

    WalkDir::new(folder)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            #[allow(unnecessary_map_or)]
            entry.path().extension().map_or(false, |extension| {
                let ext_str = extension.to_str();
                if let Some(ext_str) = ext_str {
                    FILE_EXTENSION.contains(&ext_str)
                } else {
                    false
                }
            })
        })
        .for_each(|entry| {
            let content = std::fs::read_to_string(entry.path());
            if let Err(err) = content {
                eprintln!("Error reading file: {}", err);
                return;
            }

            processed_files.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let content = content.unwrap();
            for cap in ENV_VAR_TS_DEFAULT_PATTERN.captures_iter(&content) {
                let var_name = String::from(&cap[1]);
                let current_var_ammount = result.get_mut(&var_name);
                let place = entry.path().to_string_lossy().into_owned();

                match current_var_ammount {
                    Some(mut value) => {
                        value.register_place(place);
                    }
                    None => {
                        let mut new_env = EnvMetadata::new(var_name.clone());
                        new_env.register_place(place);
                        result.insert(var_name, new_env);
                    }
                }
            }
        });

    let final_result = result
        .iter()
        .map(|entry| (*entry.value()).clone())
        .collect::<Vec<_>>();
    let processed_files = processed_files.load(std::sync::atomic::Ordering::Relaxed);

    Ok(ScanResult::new(final_result, processed_files))
}
