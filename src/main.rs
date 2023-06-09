use anyhow::bail;
use clap::{Parser, Subcommand};
use std::fs;
use thiserror::Error;

const CALL_FILE: &str = "/proc/acpi/call";
const ON_MESSAGE: &str = "\\_SB.PCI0.PEG0.PEGP._ON";
const OFF_MESSAGE: &str = "\\_SB.PCI0.PEG0.PEGP._OFF";

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Just read the acpi call result whithout envoking the call
    #[clap(short, long)]
    read: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Turns on the Nvidia GPU
    On,

    /// Turns off the Nvidia GPU
    Off,
}

impl Command {
    fn send_call(&self) -> Result<(), std::io::Error> {
        let call_message = match self {
            Command::On => ON_MESSAGE,
            Command::Off => OFF_MESSAGE,
        };
        fs::write(CALL_FILE, call_message)?;
        Ok(())
    }

    fn after_call_result(&self) -> anyhow::Result<()> {
        let file_content = fs::read_to_string(CALL_FILE)?;

        let acpi_str = match self {
            Command::On => "0x1 ",
            Command::Off => "0x0 ",
        };
        if file_content.trim() != acpi_str {
            bail!(Box::new(CallError(file_content)));
        };

        Ok(())
    }

    fn without_call_result(&self) -> anyhow::Result<()> {
        let file_content = fs::read_to_string(CALL_FILE)?;
        println!("{}", file_content);
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.read {
        cli.command.without_call_result()?;
        return Ok(());
    };

    cli.command.send_call()?;
    cli.command.after_call_result()?;

    println!("Done");

    Ok(())
}

#[derive(Error, Debug)]
#[error("acpi call error: {0}")]
struct CallError(String);
