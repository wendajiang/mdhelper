use crate::cfg::config;
use anyhow::bail;
use chrono::prelude::*;
use chrono::DateTime;
use clap::{Arg, ArgMatches, Command};
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub fn make_subcommand<'help>() -> Command<'help> {
    Command::new("gen").about("generate md file").arg(
        Arg::new("file_path")
            .short('p')
            .long("file_path")
            .value_name("FILE_PATH")
            .help("get the file path, example ./help")
            .takes_value(true),
    )
}

pub fn execute(arg: &ArgMatches) -> anyhow::Result<()> {
    let config_file_path = config()?.gen.path;
    let file_path = arg
        .value_of("file_path")
        .unwrap_or(config_file_path.as_str());

    let title = if let Ok(title) = request_blog_title() {
        title
    } else {
        bail!("must input the blog title")
    };
    let file_name = if let Ok(file_name) = request_blog_file_name() {
        file_name
    } else {
        bail!("must input the file name")
    };

    println!("\nDo you want support mermaid? (y/n)");
    let (mermaid_template, mermaid_flag) = if confirm() {
        (
            "<!--\nmermaid example:\n<div class=\"mermaid\">\n    mermaid program\n</div>\n-->",
            "true",
        )
    } else {
        ("", "false")
    };

    let pre_content = "+++\ntemplate = \"page.html\"\n";

    // let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = Local::now();

    // println!("{}", utc_time.format("%Y-%m-%d %T"));
    // println!("{}", local_time.format("%Y-%m-%d %T"));
    let str_datetime = local_time.format("%Y-%m-%d %T");

    let complete_file_name = PathBuf::from(file_path).join(file_name);

    // let complete_file_name = std::format!("{}{}.md", file_path, file_name);
    println!("file_name is {:?}", complete_file_name);

    let content = std::format!("{pre_content}date = \"{str_datetime}\"\ntitle = \"{title}\"\n[taxonomies]\ntags = []\n\n[extra]\nmermaid = {mermaid_flag}\nusemathjax = true\n+++\n{mermaid_template}");
    println!("{}", content);

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(complete_file_name)
        .unwrap();

    file.write_fmt(format_args!("{}", content)).unwrap();

    Ok(())
}

fn request_blog_file_name() -> anyhow::Result<String> {
    println!("What file name would you like to give the blog md file? ");
    io::stdout().flush().unwrap();
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();
    let resp = resp.trim();
    if resp.is_empty() {
        bail!("must input the file name")
    } else {
        Ok(resp.to_string())
    }
}

fn request_blog_title() -> anyhow::Result<String> {
    println!("What title would you like to give the blog? ");
    io::stdout().flush().unwrap();
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();
    let resp = resp.trim();
    if resp.is_empty() {
        bail!("must input the title")
    } else {
        Ok(resp.to_string())
    }
}

fn confirm() -> bool {
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    matches!(s.trim(), "Y" | "y" | "yes" | "Yes")
}
