use std::io::{self, Write};
use std::process::{exit, Command};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            exit(0);
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command {
            "cd" => {
                if args.len() == 1 {
                    if let Err(err) = std::env::set_current_dir(args[0]) {
                        eprintln!("cd: {}", err);
                    }
                } else {
                    eprintln!("cd: too many arguments");
                }
            }
            _ => {
                let output = Command::new(command).args(args).output();

                match output {
                    Ok(output) => {
                        if !output.stdout.is_empty() {
                            io::stdout().write_all(&output.stdout).unwrap();
                        }
                        if !output.stderr.is_empty() {
                            io::stderr().write_all(&output.stderr).unwrap();
                        }
                    }
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
    }
}

