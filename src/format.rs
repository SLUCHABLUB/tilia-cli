use clap::ValueEnum;

#[derive(Copy, Clone, Default, ValueEnum)]
pub enum Format {
    /// Hash sign prefixed sRGB with hexadecimal 8-bit integral channels.
    ///
    /// From my experience, this is the most common.
    /// Thus, I made it the default.
    #[default]
    #[value(alias("hex"))]
    HashHex24BitSRgb,
}
