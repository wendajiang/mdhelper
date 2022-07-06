use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"
# usually you don't wanna change those
[gen]
path = "/Users/david/md/wendajiang.github.io/content/"

[record]
path = "/Users/david/md/daily/src/"
"#;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub gen: Gen,
    pub record: Record,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Record {
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gen {
    pub path: String,
}

pub fn config() -> anyhow::Result<Config> {
    let conf = config_path()?.join("mdhelper.toml");
    if !conf.is_file() {
        fs::write(&conf, &DEFAULT_CONFIG[1..])?;
    }

    let s = fs::read_to_string(&conf)?;
    Ok(toml::from_str::<Config>(&s)?)
}

fn config_path() -> anyhow::Result<PathBuf> {
    let dir = dirs::home_dir().unwrap().join(".config");
    if !dir.is_dir() {
        println!("Generate config dir at {:?}.", &dir);
        fs::DirBuilder::new().recursive(true).create(&dir)?;
    }
    Ok(dir)
}
