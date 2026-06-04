use clap::ValueEnum;
use typed_colours::SRgb;
use typed_colours::format::AlphaSuffixed;
use typed_colours::format::HashPrefixed;
use typed_colours::format::Hexadecimal;

#[derive(Copy, Clone, Default, ValueEnum)]
pub enum Format {
    /// Hash sign prefixed sRGB with hexadecimal 8-bit integral channels.
    ///
    /// From my experience, this is the most common.
    /// Thus, I made it the default.
    #[default]
    #[value(name = "#rrggbb")]
    HashRrggbb,
    /// sRGBA with hexadecimal 8-bit integral channels.
    /// Note that the alpha channel is always maxed out.
    Rrggbbaa,
}

impl Format {
    pub fn format(self, colour: SRgb<u8>) -> String {
        match self {
            Format::HashRrggbb => HashPrefixed(Hexadecimal(colour)).to_string(),
            Format::Rrggbbaa => Hexadecimal(AlphaSuffixed(colour)).to_string(),
        }
    }
}
