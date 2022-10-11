extern crate clap;

use std::env;
use std::option::Option;
use std::process::{Command, Stdio};
use clap::{Command as App, Arg, arg};


struct BashRunner {
    shell: String
}
impl Runner for BashRunner {
    fn run_command(&mut self, command: String) {
        let mut child = Command::new(String::clone(&self.shell))
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
        .arg(Arg::new("profile")
             .short('p')
             .long("profile")
             .value_name("PROFILE")
             .help("The AWS profile to use. This will override the AWS_PROFILE environment variable."))
        .arg(arg!(<command> ... "commands to run")
             .trailing_var_arg(true))
        .get_matches();

    let environment_profile = match env::var("AWS_PROFILE") {
        Ok(value) => Some(value),
        Err(_) => None
    };

    let shell = match env::var("AWS_PROFILE_VAULT_SHELL") {
        Ok(value) => value,
        Err(_) => String::from("bash")
    };

    execute(
        &mut BashRunner { shell: shell },
        UserRequest {
            parameter_profile: matches.get_one::<String>("profile").cloned(),
            parameter_command: matches.get_many("command").expect("`command` is required").cloned().collect(),
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
    parameter_command: Vec<String>,
    environment_profile: Option<String>
}

fn execute(runner: &mut dyn Runner, request: UserRequest) {
    let environment_profile = request.environment_profile;
    runner.run_command(
        format!(
            "{}{}{}{}", 
            "aws-vault exec ", 
            request.parameter_profile.unwrap_or_else(|| environment_profile.expect("No AWS_PROFILE set.")), 
            " -- ", 
            request.parameter_command.join(" ")
        )
    );
}


#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, remove_file};
    use std::io::prelude::*;

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
                parameter_command: vec!("aws", "s3", "ls").into_iter().map(String::from).collect(),
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
                parameter_command: vec!("env").into_iter().map(String::from).collect(),
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
                parameter_command: vec!("env").into_iter().map(String::from).collect(),
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
                parameter_command: vec!("env").into_iter().map(String::from).collect(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn e2e_function_test() {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--profile")
            .arg("sandbox")
            .arg("--")
            .arg("env")
            .env("AWS_PROFILE_VAULT_SHELL", "./fake_shell")
            .spawn()
            .expect("Failed to run aws-profile-vault");
        
        let pid = child.id();
        child.wait().expect("Command failed.");

        let spy_file = String::from("./.integration_test#spy-data")+&pid.to_string();
        let mut f = File::open(
            spy_file.clone()
        ).expect(&spy_file.clone());
        let mut buffer = String::new();
        f.read_to_string(&mut buffer);
        
        assert_eq!(buffer, String::from("-c aws-vault exec sandbox -- env\n"));
        remove_file(spy_file);
    }

    #[test]
    fn e2e_function_test2() {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--profile")
            .arg("sandbox")
            .arg("env")
            .arg("--help")
            .env("AWS_PROFILE_VAULT_SHELL", "./fake_shell")
            .spawn()
            .expect("Failed to run aws-profile-vault");
        
        let pid = child.id();
        child.wait().expect("Command failed.");

        let spy_file = String::from("./.integration_test#spy-data")+&pid.to_string();
        let mut f = File::open(
            spy_file.clone()
        ).expect(&spy_file.clone());
        let mut buffer = String::new();
        f.read_to_string(&mut buffer);
        
        assert_eq!(buffer, String::from("-c aws-vault exec sandbox -- env --help\n"));
        remove_file(spy_file);
    }
}
