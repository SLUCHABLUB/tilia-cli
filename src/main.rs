use crate::arguments::Arguments;
use crate::arguments::Command;
use crate::colour::Colour;
use crate::format::Format;
use crate::mode::Mode;
use crate::rose_pine::RosePine;
use clap::Parser;

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
        } => {
            println!("{}", colour_name.to_string_with(format, Mode::get(mode)))
        }
        #[cfg(feature = "tabulate")]
        Command::TabulateColours {
            mode,
            format,
            derivations,
            markdown,
        } => {
            println!("{}", table::generate(mode, format, derivations, markdown))
        }
    }

    Ok(())
}
