use crate::cfg::config;
use anyhow::bail;
use chrono::prelude::*;
use chrono::DateTime;
use clap::{Arg, ArgMatches, Command};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Extra {
    mermaid: bool,
    usemathjax: bool,
    lead: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Taxonomies {
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ZolaAndTyporaYamlFrontMatter {
    template: String,
    date: String,
    updated: String,
    title: String,
    #[serde(rename = "typora-copy-images-to")]
    typora_copy_images_to: String,
    taxonomies: Taxonomies,
    extra: Extra,
}

impl ZolaAndTyporaYamlFrontMatter {
    pub fn new() -> ZolaAndTyporaYamlFrontMatter {
        Self {
            template: "blog/page.html".to_string(),
            date: "".to_string(),
            updated: "".to_string(),
            title: "".to_string(),
            typora_copy_images_to: "../static/pics/${filename}".to_string(),
            taxonomies: Taxonomies { tags: vec![] },
            extra: Extra {
                mermaid: false,
                usemathjax: true,
                lead: "".to_string(),
            },
        }
    }
}

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

    let mut yml_front_matter = ZolaAndTyporaYamlFrontMatter::new();

    if let Ok(title) = request_blog_title() {
        yml_front_matter.title = title;
    } else {
        bail!("must input the blog title")
    };
    let file_name = if let Ok(file_name) = request_blog_file_name() {
        file_name + ".md"
    } else {
        bail!("must input the file name")
    };

    println!("\nDo you want support mermaid? (y/n)");
    let mermaid_template = if confirm() {
        yml_front_matter.extra.mermaid = true;
        r#"
# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
"#
    } else {
        // do nothing
        ""
    };

    let delimiter = "---\n";

    // let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = Local::now();

    // println!("{}", utc_time.format("%Y-%m-%d %T"));
    // println!("{}", local_time.format("%Y-%m-%d %T"));
    yml_front_matter.date = local_time.format("%Y-%m-%d %T").to_string();
    yml_front_matter.updated = local_time.format("%Y-%m-%d %T").to_string();

    let complete_file_name = PathBuf::from(file_path).join(file_name);

    // let complete_file_name = std::format!("{}{}.md", file_path, file_name);
    println!("file_name is {:?}", complete_file_name);

    let content = format!(
        "{}{}{}{}",
        delimiter,
        serde_yaml::to_string(&yml_front_matter).unwrap(),
        mermaid_template,
        delimiter
    );

    println!("{content}");

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
