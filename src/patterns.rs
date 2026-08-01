use std::sync::LazyLock;

use regex::Regex;

pub const FILE_EXTENSION: [&str; 3] = ["ts", "cjs", "mjs"];
pub static ENV_VAR_TS_DEFAULT_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"process\.env\.(\w+)").expect("Regex need to be valid."));
