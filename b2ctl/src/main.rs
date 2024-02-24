use crate::args::Cli;
use argon2::{password_hash::{rand_core::{CryptoRng, CryptoRngCore}, SaltString}, Argon2, PasswordHasher};
use constant_time_eq::constant_time_eq;
use rand::rngs::OsRng;
use clap::Parser;
use config::Config;
pub mod args;
pub mod conf;

fn main() {
    let args = Cli::parse();

    match args.command {
        //args::Commands::Wizard { output } => {}
        args::Commands::Check { config } => {
            let config = std::fs::read_to_string(config).expect("this file should only contain ASCII characters");
            let config: Result<Config,_> = serde_json::from_str(&config);
            match config {
                Ok(_) => {
                    eprintln!("This file is valid");
                    
                }
                Err(e) => {
                    eprintln!("Invalid file: {}",e);
                    std::process::exit(1)
                }
            }
        }
        /*
        args::Commands::Convert { input, output } => {

        }
        */
        args::Commands::Sample => {
            let mut conf = conf::fallback_menu();
            conf.items.push(config::BootItem {
                name: "Linux".to_owned(),
                target: config::BootTarget::EFI {
                    path: "/linux".to_owned(),
                    cmdline: Some("initrd=/initrd".to_owned()),
                },
            });
            let s = serde_json::to_string_pretty(&conf);
            println!("{}", s.unwrap());
        }
        args::Commands::Password => {
            let password = rpassword::prompt_password("Password: ").unwrap();
            let password_repeat = rpassword::prompt_password("Repeat Password: ").unwrap();
            if ! constant_time_eq(password.as_bytes(), password_repeat.as_bytes()) {
                eprintln!("Password mismatch.");
                std::process::exit(1);
            }
            let argon2 = Argon2::default();
            let salt = SaltString::generate(&mut OsRng);
            let pwhash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
            println!("\n{}",pwhash);
        }
    }
}
