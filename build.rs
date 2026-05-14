use std::env::var_os;
use std::fs::write;
use std::path::Path;
use typed_colours::SRgb;
use typed_colours::format::HashPrefixed;
use typed_colours::format::Hexadecimal;

/// Parses a Rosé Pine colour as it is given on the
/// [pallete section on thir website](https://rosepinetheme.com/palette).
fn parse(string: &str) -> SRgb<u8> {
    let HashPrefixed(Hexadecimal(colour)) = string.parse().unwrap();

    colour
}

/// A mirror of the same struct in the crate.
#[allow(dead_code)]
#[derive(Debug)]
struct RosePine {
    base: SRgb<u8>,
    surface: SRgb<u8>,
    overlay: SRgb<u8>,

    muted: SRgb<u8>,
    subtle: SRgb<u8>,
    text: SRgb<u8>,

    love: SRgb<u8>,
    gold: SRgb<u8>,
    rose: SRgb<u8>,
    pine: SRgb<u8>,
    foam: SRgb<u8>,
    iris: SRgb<u8>,

    highlight_low: SRgb<u8>,
    highlight_med: SRgb<u8>,
    highlight_high: SRgb<u8>,
}

fn main() {
    let output_directory = var_os("OUT_DIR").unwrap();
    let path = Path::new(&output_directory).join("rose_pine.rs");

    generate(&path);

    println!("cargo::rerun-if-changed=build.rs");
}

fn generate(path: &Path) {
    fn make_constant(name: &str, value: RosePine) -> String {
        format!("const {name}: RosePine = {value:?};")
    }

    let main: RosePine = RosePine {
        base: parse("#191724"),
        surface: parse("#1f1d2e"),
        overlay: parse("#26233a"),
        muted: parse("#6e6a86"),
        subtle: parse("#908caa"),
        text: parse("#e0def4"),
        love: parse("#eb6f92"),
        gold: parse("#f6c177"),
        rose: parse("#ebbcba"),
        pine: parse("#31748f"),
        foam: parse("#9ccfd8"),
        iris: parse("#c4a7e7"),
        highlight_low: parse("#21202e"),
        highlight_med: parse("#403d52"),
        highlight_high: parse("#524f67"),
    };

    let dawn: RosePine = RosePine {
        base: parse("#faf4ed"),
        surface: parse("#fffaf3"),
        overlay: parse("#f2e9e1"),
        muted: parse("#9893a5"),
        subtle: parse("#797593"),
        text: parse("#575279"),
        love: parse("#b4637a"),
        gold: parse("#ea9d34"),
        rose: parse("#d7827e"),
        pine: parse("#286983"),
        foam: parse("#56949f"),
        iris: parse("#907aa9"),
        highlight_low: parse("#f4ede8"),
        highlight_med: parse("#dfdad9"),
        highlight_high: parse("#cecacd"),
    };

    let main = make_constant("MAIN", main);
    let dawn = make_constant("DAWN", dawn);

    let string = format!("impl RosePine {{ {main} {dawn} }}");

    write(path, string).unwrap();
}
