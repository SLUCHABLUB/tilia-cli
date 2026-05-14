use std::slice::from_ref;

use crate::Mode;
use crate::colour::Colour;
use crate::format::Format;
use clap::ValueEnum as _;
use tabled::Table;
use tabled::builder::Builder;
use tabled::grid::config::Offset;
use tabled::settings::Alignment;
use tabled::settings::Style;
use tabled::settings::object::Rows;
use tabled::settings::style::LineChar;

fn header(modes: &[Mode], derivations: bool) -> Vec<String> {
    let mut header = vec!["Colour".to_owned()];

    header.extend(modes.iter().map(|mode| format!("{mode:?}")));

    if derivations {
        header.push("Derived from".to_owned());
    }

    header
}

/// The colours after which to insert a blank line in the table.
const INSERT_BLANK_AFTER: &[Colour] = &[Colour::TertiaryBackground, Colour::Border, Colour::Text];

pub fn generate(mode: Option<Mode>, format: Format, derivations: bool, markdown: bool) -> String {
    let modes = mode
        .as_ref()
        .map(from_ref)
        .unwrap_or(&[Mode::Dark, Mode::Light]);

    let empty_record = vec![""; 1 + modes.len() + derivations as usize];

    let mut builder = Builder::new();

    builder.push_record(header(modes, derivations));

    if !markdown {
        builder.push_record(empty_record.clone());
    }

    for colour in Colour::value_variants() {
        let mut record = vec![];

        let name = colour.to_string();

        record.push(if markdown { format!("_{name}_") } else { name });

        for mode in modes {
            let colour = colour.to_string_with(format, *mode);

            record.push(if markdown {
                format!("`{colour}`")
            } else {
                colour
            });
        }

        if derivations {
            record.push(colour.origin().to_owned());
        }

        builder.push_record(record);

        if INSERT_BLANK_AFTER.contains(colour) {
            builder.push_record(empty_record.clone());
        }
    }

    let mut table = builder.build();

    if markdown {
        table.with(Style::markdown());
        align_markdown_centre(&mut table);
    } else {
        table.with(Style::blank());
        table.with(Alignment::center());
    }

    table.to_string()
}

fn align_markdown_centre(table: &mut Table) {
    let separator = Rows::one(1);
    let insert_colons = (
        LineChar::horizontal(' ', Offset::Start(0)),
        LineChar::horizontal(':', Offset::Start(1)),
        LineChar::horizontal(':', Offset::End(1)),
        LineChar::horizontal(' ', Offset::End(0)),
    );

    table.modify(separator, insert_colons);
}
