extern crate clap;

use std::process::{Command, Stdio};
use clap::{App, Arg};

fn main() {
    let matches = App::new("aws-profile-vault")
        .arg(Arg::with_name("profile").short("p").takes_value(true).required(true).value_name("PROFILE").help("The AWS profile to use."))
        .arg(Arg::with_name("command").required(true).multiple(true))
        .get_matches();

    let mut child = Command::new("bash")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("-c")
        .arg(format!("{}{}{}{}", "aws-vault exec ", matches.value_of("profile").unwrap(), " -- ", matches.values_of("command").map(|vals| vals.collect::<Vec<_>>()).unwrap().join(" ")))
        .spawn()
        .expect("Could not find binary 'bash'.");
    

    child.wait().expect("Command failed.");
}
