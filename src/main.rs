use crate::arguments::Arguments;
use crate::arguments::Command;
use crate::colour::Colour;
use crate::format::Format;
use crate::mode::Mode;
use crate::rose_pine::RosePine;
use clap::Parser;

mod arguments;
mod colour;
mod foreground;
mod format;
mod mode;
mod rose_pine;
#[cfg(feature = "tabulate")]
mod table;

fn main() -> anyhow::Result<()> {
    let Arguments { command } = Arguments::parse();

    let output = match command {
        Command::GetColour {
            colour_name,
            format,
            mode,
            fallback_mode,
        } => colour_name.to_string_with(format, Mode::get(mode, fallback_mode)?),
        Command::GetBackground {
            colour_name,
            format,
            level,
            mode,
            fallback_mode,
        } => format.format(colour_name.background(level, Mode::get(mode, fallback_mode)?)),
        #[cfg(feature = "tabulate")]
        Command::TabulateColours {
            mode,
            format,
            derivations,
            markdown,
        } => table::generate(mode, format, derivations, markdown),
    };

    println!("{output}");

    Ok(())
}
