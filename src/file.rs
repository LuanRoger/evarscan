use crate::sniffer::ScanResult;

pub struct WriteResultError(String);

pub fn write_result_env_metadata(values: &ScanResult) -> Result<(), WriteResultError> {
    let json = serde_json::to_string(values)
        .map_err(|_| WriteResultError(String::from("Error when serializing result")))?;
    std::fs::write("result.json", json)
        .map_err(|_| WriteResultError(String::from("Error when writing result")))?;

    Ok(())
}
