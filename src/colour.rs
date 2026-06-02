use crate::Format;
use crate::Mode;
use clap::ValueEnum;
use typed_colours::Palette as _;
use typed_colours::SRgb;
use typed_colours::format::HashPrefixed;
use typed_colours::format::Hexadecimal;

#[derive(Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum Colour {
    PrimaryBackground,
    SecondaryBackground,
    TertiaryBackground,

    Hover,
    Selection,
    Border,

    Disabled,
    Unimportant,
    Text,

    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,

    BackgroundRed,
    BackgroundOrange,
    BackgroundYellow,
    BackgroundGreen,
    BackgroundCyan,
    BackgroundBlue,
    BackgroundPurple,
}

impl Colour {
    pub fn srgb(self, mode: Mode) -> SRgb<u8> {
        mode.resolve(&self)
    }

    #[cfg(feature = "tabulate")]
    pub fn name(self) -> &'static str {
        match self {
            Colour::PrimaryBackground => "Primary Background",
            Colour::SecondaryBackground => "Secondary Background",
            Colour::TertiaryBackground => "Tertiary Background",
            Colour::Hover => "Hover",
            Colour::Selection => "Selection",
            Colour::Border => "Border",
            Colour::Disabled => "Disabled",
            Colour::Unimportant => "Unimportant",
            Colour::Text => "Text",
            Colour::Red => "Red",
            Colour::Orange => "Orange",
            Colour::Yellow => "Yellow",
            Colour::Green => "Green",
            Colour::Cyan => "Cyan",
            Colour::Blue => "Blue",
            Colour::Purple => "Purple",
            Colour::BackgroundRed => "Background Red",
            Colour::BackgroundOrange => "Background Orange",
            Colour::BackgroundYellow => "Background Yellow",
            Colour::BackgroundGreen => "Background Green",
            Colour::BackgroundCyan => "Background Cyan",
            Colour::BackgroundBlue => "Background Blue",
            Colour::BackgroundPurple => "Background Purple",
        }
    }

    #[cfg(feature = "tabulate")]
    pub fn origin(self) -> &'static str {
        match self {
            Colour::PrimaryBackground => "Base",
            Colour::SecondaryBackground => "Surface",
            Colour::TertiaryBackground => "Overlay",
            Colour::Hover => "Highlight Low",
            Colour::Selection => "Highlight Med",
            Colour::Border => "Highlight High",
            Colour::Disabled => "Muted",
            Colour::Unimportant => "Subtle",
            Colour::Text => "Text",
            Colour::Red => "Love",
            Colour::Orange => "Rose",
            Colour::Yellow => "Gold",
            Colour::Green => "Iris*",
            Colour::Cyan => "Foam",
            Colour::Blue => "Pine",
            Colour::Purple => "Iris",
            Colour::BackgroundRed => "Base & Love",
            Colour::BackgroundOrange => "Base & Rose",
            Colour::BackgroundYellow => "Base & Gold",
            Colour::BackgroundGreen => "Base & Iris*",
            Colour::BackgroundCyan => "Base & Foam",
            Colour::BackgroundBlue => "Base & Pine",
            Colour::BackgroundPurple => "Base & Iris",
        }
    }

    pub fn to_string_with(self, format: Format, mode: Mode) -> String {
        match format {
            Format::HashHex24BitSRgb => HashPrefixed(Hexadecimal(self.srgb(mode))).to_string(),
        }
    }
}

#[cfg(feature = "tabulate")]
impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
