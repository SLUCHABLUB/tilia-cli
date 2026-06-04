use crate::colour::Colour;
use crate::mode::Mode;
use clap::ValueEnum;
use typed_colours::Lerp as _;
use typed_colours::Oklab;
use typed_colours::Palette;
use typed_colours::SRgb;
use typed_colours::UnitInterval;
use typed_colours::unit_interval;

#[derive(Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum ForegroundColour {
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
}

#[derive(Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum BackgroundLevel {
    #[value(alias = "1")]
    Primary,
    #[value(alias = "2")]
    Secondary,
    #[value(alias = "3")]
    Tertiary,
}

impl BackgroundLevel {
    fn to_colour(self) -> Colour {
        match self {
            BackgroundLevel::Primary => Colour::PrimaryBackground,
            BackgroundLevel::Secondary => Colour::SecondaryBackground,
            BackgroundLevel::Tertiary => Colour::TertiaryBackground,
        }
    }
}

impl ForegroundColour {
    fn is_achromatic(self) -> bool {
        matches!(
            self,
            ForegroundColour::Disabled | ForegroundColour::Unimportant | ForegroundColour::Text
        )
    }

    fn to_colour(self) -> Colour {
        match self {
            ForegroundColour::Disabled => Colour::Disabled,
            ForegroundColour::Unimportant => Colour::Unimportant,
            ForegroundColour::Text => Colour::Text,
            ForegroundColour::Red => Colour::Red,
            ForegroundColour::Orange => Colour::Orange,
            ForegroundColour::Yellow => Colour::Yellow,
            ForegroundColour::Green => Colour::Green,
            ForegroundColour::Cyan => Colour::Cyan,
            ForegroundColour::Blue => Colour::Blue,
            ForegroundColour::Purple => Colour::Purple,
        }
    }

    pub fn background(self, level: BackgroundLevel, mode: Mode) -> SRgb<u8> {
        let background = mode.resolve(&level.to_colour());

        if self.is_achromatic() || mode == Mode::Dark {
            background
        } else {
            let colour = mode.resolve(&self.to_colour());

            SRgb::<u8>::from(SRgb::<UnitInterval<f64>>::from(Oklab::lerp(
                Oklab::<f64, f64>::from(SRgb::<UnitInterval<f64>>::from(colour)),
                Oklab::<f64, f64>::from(SRgb::<UnitInterval<f64>>::from(background)),
                unit_interval!(0.9),
            )))
        }
    }
}
