use crate::Colour;
use crate::Format;
use crate::Mode;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Print a colour to stdout.
    #[command(long_about = None)]
    GetColour {
        /// The name of the colour to print.
        #[arg()]
        colour_name: Colour,
        /// How to format the colours.
        #[arg(default_value = "hex")]
        format: Format,
        /// The mode of tilia to use.
        #[arg(short, long)]
        mode: Option<Mode>,
        /// The mode of tilia to use if the system one cannot be detected.
        #[arg(long)]
        fallback_mode: Option<Mode>,
    },
    /// Print a table of colour to stdout.
    #[cfg(feature = "tabulate")]
    #[command(long_about = None)]
    TabulateColours {
        /// Only print colours for this mode.
        #[arg(short, long)]
        mode: Option<Mode>,
        /// How to format the colours.
        #[arg(short, long, default_value = "hex")]
        format: Format,
        /// Print the Rosé Pine origin colours.
        #[arg(short, long)]
        derivations: bool,
        /// Print the table in markdown syntax.
        #[arg(short = 'M', long)]
        markdown: bool,
    },
}
