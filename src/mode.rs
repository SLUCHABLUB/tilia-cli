use crate::Colour;
use crate::RosePine;
use anyhow::bail;
use clap::ValueEnum;
use typed_colours::Lerp as _;
use typed_colours::Oklab;
use typed_colours::Palette;
use typed_colours::SRgb;
use typed_colours::UnitInterval;
use typed_colours::unit_interval;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Mode {
    Dark,
    Light,
}

impl Mode {
    pub fn get(specified: Option<Mode>, fallback: Option<Mode>) -> anyhow::Result<Mode> {
        if let Some(mode) = specified {
            return Ok(mode);
        }

        #[cfg(feature = "mode-detection")]
        {
            use anyhow::Context as _;

            let mode = dark_light::detect().context("detecting system theme mode")?;

            match mode {
                dark_light::Mode::Dark => return Ok(Mode::Dark),
                dark_light::Mode::Light => return Ok(Mode::Light),
                dark_light::Mode::Unspecified => (),
            }
        }

        if let Some(mode) = fallback {
            return Ok(mode);
        }

        bail!("cannot detect system theme mode");
    }
}

impl Palette<Colour, SRgb<u8>> for Mode {
    fn resolve(&self, entry: &Colour) -> SRgb<u8> {
        let rose_pine = RosePine::from(*self);

        let purple = Oklab::<f64, f64>::from(SRgb::<UnitInterval<f64>>::from(rose_pine.iris));
        let background = Oklab::<f64, f64>::from(SRgb::<UnitInterval<f64>>::from(rose_pine.base));

        let green = Oklab {
            lightness: purple.lightness,
            a: background.a - purple.a,
            b: background.b - purple.b,
        };
        let green = SRgb::<u8>::from(SRgb::<UnitInterval<f64>>::from(green));

        let background = |colour| {
            SRgb::<u8>::from(SRgb::<UnitInterval<f64>>::from(Oklab::lerp(
                Oklab::<f64, f64>::from(SRgb::<UnitInterval<f64>>::from(colour)),
                Oklab::<f64, f64>::from(SRgb::<UnitInterval<f64>>::from(rose_pine.base)),
                unit_interval!(0.9),
            )))
        };

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
            Colour::Green => green,
            Colour::Cyan => rose_pine.foam,
            Colour::Blue => rose_pine.pine,
            Colour::Purple => rose_pine.iris,

            Colour::BackgroundRed => background(rose_pine.love),
            Colour::BackgroundOrange => background(rose_pine.rose),
            Colour::BackgroundYellow => background(rose_pine.gold),
            Colour::BackgroundGreen => background(green),
            Colour::BackgroundCyan => background(rose_pine.foam),
            Colour::BackgroundBlue => background(rose_pine.pine),
            Colour::BackgroundPurple => background(rose_pine.iris),
        }
    }
}
