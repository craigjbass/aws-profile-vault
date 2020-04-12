extern crate clap;

use std::option::Option;
use std::process::{Command, Stdio};
use clap::{App, Arg};

struct BashRunner {}
impl Runner for BashRunner {
    fn run_command(&mut self, command: String) {
        let mut child = Command::new("bash")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("Could not find binary 'bash'.");
        
        child.wait().expect("Command failed.");
    }
}

fn main() {
    let matches = App::new("aws-profile-vault")
        .version("Cherrytree")
        .author("Craig J. Bass <craig@madetech.com>")
        .about("Let's you run scripts that use aws-profile when you only have aws-vault.")
        .arg(Arg::with_name("profile")
             .short("p")
             .long("profile")
             .takes_value(true)
             .required(true)
             .value_name("PROFILE")
             .help("The AWS profile to use."))
        .arg(Arg::with_name("command")
             .required(true)
             .multiple(true))
        .get_matches();

    execute(
        &mut BashRunner {},
        UserRequest {
            parameter_profile: matches.value_of("profile").map(str::to_string),
            parameter_command: matches.values_of_lossy("command")
        }
    )
}

trait Runner {
  fn run_command(&mut self, command: String);
}

struct UserRequest {
    parameter_profile: Option<String>,
    parameter_command: Option<Vec<String>>
}

fn execute(runner: &mut Runner, request: UserRequest) {
    runner.run_command(format!("{}{}{}{}", "aws-vault exec ", request.parameter_profile.unwrap(), " -- ", request.parameter_command.unwrap().join(" ")));
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RunnerSpy {
        command: Option<String>
    }
    impl Runner for RunnerSpy {
        fn run_command(&mut self, command: String) {
            self.command = Some(command);
        }
    }

    #[test]
    fn can_run_aws_vault_1() {
        let mut spy = RunnerSpy {
            command: None
        };
        execute(
            &mut spy,
            UserRequest {
                parameter_profile: Some("sandbox").map(String::from),
                parameter_command: Some(vec!("aws", "s3", "ls").into_iter().map(String::from).collect())
            }
        );

        assert_eq!(spy.command, Some(String::from("aws-vault exec sandbox -- aws s3 ls")));
    }

    #[test]
    fn can_run_aws_vault_2() {
        let mut spy = RunnerSpy {
            command: None
        };
        execute(
            &mut spy,
            UserRequest {
                parameter_profile: Some("production").map(String::from),
                parameter_command: Some(vec!("env").into_iter().map(String::from).collect())
            }
        );

        assert_eq!(spy.command, Some(String::from("aws-vault exec production -- env")));
    }
}
