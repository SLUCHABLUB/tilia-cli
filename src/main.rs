use crate::arguments::Arguments;
use crate::arguments::Command;
use crate::colour::Colour;
use crate::format::Format;
use crate::mode::Mode;
use crate::rose_pine::RosePine;
use clap::Parser;
use std::io;
use std::io::IsTerminal;
use std::io::Write;
use std::io::stdout;

mod arguments;
mod colour;
mod format;
mod mode;
mod rose_pine;
#[cfg(feature = "tabulate")]
mod table;

fn main() -> anyhow::Result<()> {
    let Arguments { command } = Arguments::parse();

    match command {
        Command::GetColour {
            colour_name,
            format,
            mode,
            fallback_mode,
        } => {
            print(&colour_name.to_string_with(format, Mode::get(mode, fallback_mode)?))?;
        }
        #[cfg(feature = "tabulate")]
        Command::TabulateColours {
            mode,
            format,
            derivations,
            markdown,
        } => {
            print(&table::generate(mode, format, derivations, markdown))?;
        }
    }

    Ok(())
}

fn print(string: &str) -> io::Result<()> {
    let mut stdout = stdout();

    stdout.write_all(string.as_bytes())?;

    if stdout.is_terminal() {
        stdout.write_all("\n".as_bytes())?;
    }

    stdout.flush()?;

    Ok(())
}
