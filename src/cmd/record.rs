use crate::cfg::config;
use anyhow::bail;
use chrono::{DateTime, Local};
use clap::{Arg, ArgMatches, Command};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn make_subcommand<'help>() -> Command<'help> {
    Command::new("record")
        .about("add some title in the head of file")
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .takes_value(true)
                .help("record path")
                .required(false),
        )
        .arg(
            Arg::new("daily")
                .long("daily")
                .short('d')
                .help("add daily second level title")
                .required(false),
        )
        .arg(
            Arg::new("weekly")
                .long("weekly")
                .short('w')
                .help("add weekly second level title")
                .required(false),
        )
}

pub fn execute(arg: &ArgMatches) -> anyhow::Result<()> {
    let config_path = config()?.record.path;
    let path = arg.value_of("path").unwrap_or(config_path.as_str());
    let daily_flag = arg.contains_id("daily");
    let weekly_flag = arg.contains_id("weekly");
    if daily_flag && weekly_flag {
        bail!("Can not both daily and weekly");
    }

    if daily_flag {
        daily(path)?;
    } else if weekly_flag {
        weekly(path)?;
    } else {
        unreachable!();
    };

    Ok(())
}

fn daily(path: &str) -> anyhow::Result<()> {
    let local_time: DateTime<Local> = Local::now();
    let time_format = local_time.format("%Y%m%d");
    let time_format2 = local_time.format("%Y-%m-%d");
    let file_name = PathBuf::from(path).join(format!("{}.md", time_format));

    let summary = PathBuf::from(path).join("SUMMARY.md");

    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_name)
        .unwrap();

    let mut summary_fd = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open(summary)
        .unwrap();

    summary_fd.write_all(format!("- [{}]({}.md)", time_format2, time_format).as_bytes())?;

    Ok(())
}

fn weekly(_path: &str) -> anyhow::Result<()> {
    todo!()
}
