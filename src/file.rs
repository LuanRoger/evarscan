use crate::{patterns::RESULT_FILE, sniffer::ScanResult};

pub struct WriteResultError(pub String);

pub fn write_result_env_metadata(values: &ScanResult) -> Result<(), WriteResultError> {
    let json = serde_json::to_string(values)
        .map_err(|_| WriteResultError(String::from("Error when serializing result")))?;
    std::fs::write(RESULT_FILE, json)
        .map_err(|_| WriteResultError(String::from("Error when writing result")))?;

    Ok(())
}
