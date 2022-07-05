mod cfg;
mod cmd;

use clap::{crate_description, crate_name, crate_version, Command};

const VERSION: &str = concat!("v", crate_version!());

fn main() {
    let command = create_clap_command();

    let res = match command.get_matches().subcommand() {
        Some(("gen", sub_matches)) => cmd::gen::execute(sub_matches),
        Some(("record", sub_matches)) => cmd::gen::execute(sub_matches),
        _ => unreachable!(),
    };

    if let Err(e) = res {
        eprintln!("{e}");
        std::process::exit(101);
    }
}

fn create_clap_command() -> Command<'static> {
    Command::new(crate_name!())
        .about(crate_description!())
        .author("David <wendajiang93@163.com>")
        .version(VERSION)
        .arg_required_else_help(true)
        .propagate_version(true)
        .after_help(
            "For more information about specific command, try `mdhelper <command> --help`\n
         ",
        )
        .subcommand(cmd::gen::make_subcommand())
        .subcommand(cmd::record::make_subcommand())
}
