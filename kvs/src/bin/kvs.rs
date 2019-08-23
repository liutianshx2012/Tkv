use clap::{App, AppSettings, Arg, SubCommand};
//use kvs::KvStore;
use std::process::exit;
fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            //            let key = matches.value_of("KEY").expect("KEY argument missing");
            //            let value = matches.value_of("VALUE").expect("VALUE argument missing");
            //            println!("{} ,{}", key, value);
            //            let mut kvs = KvStore::new();
            //            kvs.set(key.to_string(), value.to_string());
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            //            let key = matches.value_of("KEY").expect("KEY argument missing");
            //            let kvs = KvStore::new();
            //            let val = kvs.get(key.to_string());
            //            println!("{:?}", val);
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
