extern crate clap;

use std::env;
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
             .value_name("PROFILE")
             .help("The AWS profile to use. This will override the AWS_PROFILE environment variable."))
        .arg(Arg::with_name("command")
             .required(true)
             .multiple(true))
        .get_matches();

    let environment_profile = match env::var("AWS_PROFILE") {
        Ok(value) => Some(value),
        Err(_) => None
    };

    execute(
        &mut BashRunner {},
        UserRequest {
            parameter_profile: matches.value_of("profile").map(str::to_string),
            parameter_command: matches.values_of_lossy("command"),
            environment_profile: environment_profile,
            ..Default::default()
        }
    )
}

trait Runner {
  fn run_command(&mut self, command: String);
}

#[derive(Default)]
struct UserRequest {
    parameter_profile: Option<String>,
    parameter_command: Option<Vec<String>>,
    environment_profile: Option<String>
}

fn execute(runner: &mut Runner, request: UserRequest) {
    let environment_profile = request.environment_profile;
    runner.run_command(
        format!(
            "{}{}{}{}", 
            "aws-vault exec ", 
            request.parameter_profile.unwrap_or_else(|| environment_profile.expect("No AWS_PROFILE set.")), 
            " -- ", 
            request.parameter_command.unwrap().join(" ")
        )
    );
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
    fn can_set_profile() {
        let mut spy = RunnerSpy {
            command: None
        };
        execute(
            &mut spy,
            UserRequest {
                parameter_profile: Some("sandbox").map(String::from),
                parameter_command: Some(vec!("aws", "s3", "ls").into_iter().map(String::from).collect()),
                ..Default::default()
            }
        );

        assert_eq!(spy.command, Some(String::from("aws-vault exec sandbox -- aws s3 ls")));
    }

    #[test]
    fn can_override_environment_variable() {
        let mut spy = RunnerSpy {
            command: None
        };
        execute(
            &mut spy,
            UserRequest {
                parameter_profile: Some("production").map(String::from),
                parameter_command: Some(vec!("env").into_iter().map(String::from).collect()),
                environment_profile: Some(String::from("live")),
                ..Default::default()
            }
        );

        assert_eq!(spy.command, Some(String::from("aws-vault exec production -- env")));
    }

    #[test]
    fn can_use_environment_variable() {
        let mut spy = RunnerSpy {
            command: None
        };
        execute(
            &mut spy,
            UserRequest {
                parameter_command: Some(vec!("env").into_iter().map(String::from).collect()),
                environment_profile: Some(String::from("live")),
                ..Default::default()
            }
        );

        assert_eq!(spy.command, Some(String::from("aws-vault exec live -- env")));
    }

    #[test]
    #[should_panic(expected = "No AWS_PROFILE set.")]
    fn can_panic_if_no_aws_profile_available() {
        let mut spy = RunnerSpy {
            command: None
        };
        execute(
            &mut spy,
            UserRequest {
                parameter_command: Some(vec!("env").into_iter().map(String::from).collect()),
                ..Default::default()
            }
        );
    }
}
