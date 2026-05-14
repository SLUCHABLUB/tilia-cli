use crate::Colour;
use crate::RosePine;
use clap::ValueEnum;
use typed_colours::Palette;
use typed_colours::SRgb;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Mode {
    Dark,
    Light,
}

impl Mode {
    pub fn get(specified: Option<Mode>) -> Mode {
        if let Some(mode) = specified {
            return mode;
        }

        todo!("determine the system mode")
    }
}

impl Palette<Colour, SRgb<u8>> for Mode {
    fn resolve(&self, entry: &Colour) -> SRgb<u8> {
        let rose_pine = RosePine::from(*self);

        match *entry {
            Colour::PrimaryBackground => rose_pine.base,
            Colour::SecondaryBackground => rose_pine.surface,
            Colour::TertiaryBackground => rose_pine.overlay,
            Colour::Hover => rose_pine.highlight_low,
            Colour::Selection => rose_pine.highlight_med,
            Colour::Border => rose_pine.highlight_high,
            Colour::Disabled => rose_pine.muted,
            Colour::Unimportant => rose_pine.subtle,
            Colour::Text => rose_pine.text,
            Colour::Red => rose_pine.love,
            Colour::Orange => rose_pine.rose,
            Colour::Yellow => rose_pine.gold,
            Colour::Green => rose_pine.iris, // FIXME
            Colour::Cyan => rose_pine.foam,
            Colour::Blue => rose_pine.pine,
            Colour::Purple => rose_pine.iris,
        }
    }
}
