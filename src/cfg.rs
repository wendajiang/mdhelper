use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG: &str = r#"
# gen default config
[gen]
path = "";

[record]
"#;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Record {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gen {}
