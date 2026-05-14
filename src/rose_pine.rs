use crate::Mode;
use typed_colours::SRgb;

/// A [Rosé Pine colour palette](https://rosepinetheme.com/palette).
pub struct RosePine {
    pub base: SRgb<u8>,
    pub surface: SRgb<u8>,
    pub overlay: SRgb<u8>,

    pub muted: SRgb<u8>,
    pub subtle: SRgb<u8>,
    pub text: SRgb<u8>,

    pub love: SRgb<u8>,
    pub gold: SRgb<u8>,
    pub rose: SRgb<u8>,
    pub pine: SRgb<u8>,
    pub foam: SRgb<u8>,
    pub iris: SRgb<u8>,

    pub highlight_low: SRgb<u8>,
    pub highlight_med: SRgb<u8>,
    pub highlight_high: SRgb<u8>,
}

include!(concat!(env!("OUT_DIR"), "/rose_pine.rs"));

impl From<Mode> for RosePine {
    /// Returns the Rosé Pine colour palette used for a certain mode.
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Dark => RosePine::MAIN,
            Mode::Light => RosePine::DAWN,
        }
    }
}
