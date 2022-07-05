use chrono::prelude::*;
use chrono::DateTime;
use clap::{Arg, ArgMatches, Command};
use std::io::Write;

pub fn make_subcommand<'help>() -> Command<'help> {
    Command::new("gen")
        .about("generate md file")
        .arg(
            Arg::new("file_path")
                .short('p')
                .long("file_path")
                .value_name("FILE_PATH")
                .help("get the file path, example ./help")
                .takes_value(true),
        )
        .arg(
            Arg::new("title")
                .short('t')
                .long("title")
                .value_name("TITLE")
                .takes_value(true)
                .help("get the blog title"),
        )
        .arg(
            Arg::new("mermaid")
                .short('m')
                .long("mermaid")
                .value_name("mermaid true/false")
                .help("-m/--mermaid true to turn on mermaid"),
        )
        .arg(Arg::new("file_name").required(true).index(1))
}

pub fn execute(arg: &ArgMatches) -> anyhow::Result<()> {
    let file_path = arg.value_of("file_path").unwrap_or("./content/");
    let title = arg.value_of("title").unwrap_or("");
    let file_name_in = arg.value_of("file_name").unwrap();
    let mermaid_flag = arg.value_of("mermaid").unwrap_or("false");
    let mut mermaid_template =
        "<!--\nmermaid example:\n<div class=\"mermaid\">\n    mermaid program\n</div>\n-->";
    if mermaid_flag == "false" {
        mermaid_template = "";
    }

    let pre_content = "+++\ntemplate = \"page.html\"\n";

    // let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = Local::now();

    // println!("{}", utc_time.format("%Y-%m-%d %T"));
    // println!("{}", local_time.format("%Y-%m-%d %T"));
    let str_datetime = local_time.format("%Y-%m-%d %T");

    let file_name = std::format!("{}{}.md", file_path, file_name_in);
    println!("file_name is {}", file_name);

    let content = std::format!("{}date = \"{}\"\ntitle = \"{}\"\n[taxonomies]\ntags = []\n\n[extra]\nmermaid = {}\nusemathjax = true\n+++\n{}", pre_content, str_datetime, title, mermaid_flag, mermaid_template);
    println!("{}", content);

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();

    file.write_fmt(format_args!("{}", content)).unwrap();

    Ok(())
}
