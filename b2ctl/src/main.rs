use crate::args::Cli;
use clap::Parser;
use config::Config;
pub mod args;
pub mod conf;

fn main() {
    let args = Cli::parse();
    let mut conf = conf::fallback_menu();
    conf.items.push(config::BootItem {
        name: "Linux".to_owned(),
        target: config::BootTarget::EFI {
            path: "/linux.efi".to_owned(),
            cmdline: Some("initrd=/initrd.gz noacpi".to_owned()),
        },
    });
    let s = toml::to_string_pretty(&conf);
    println!("{}", s.unwrap());
    let s = serde_json::to_string_pretty(&conf);
    println!("{}", s.unwrap());
}
