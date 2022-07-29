use color_eyre::owo_colors::OwoColorize;
use tabled::settings::object::Columns;
use tabled::settings::object::Rows;
use tabled::settings::Disable;
use tabled::settings::Format;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::Table;

pub fn error(errors: &[&str]) {
    let data: Vec<_> = errors.iter().map(|e| ("error", *e)).collect();

    eprintln!(
        "{}",
        Table::new(data)
            .with(Style::blank())
            .with(Disable::row(Rows::new(..1)))
            .with(Modify::new(Columns::single(0)).with(Format::content(|s| s.red().to_string())),)
            .with(Modify::new(Columns::single(1)).with(Format::content(|s| s.red().to_string())),)
    );

    std::process::exit(1)
}

#[derive(Copy, Clone)]
pub enum Outcome {
    Positive,
    Negative,
    Neutral,
}

pub fn outcome(outcome: Outcome, message: &[(&str, &str)]) {
    println!(
        "{}",
        Table::new(message)
            .with(Style::blank())
            .with(Disable::row(Rows::new(..1)))
            .with(Modify::new(Columns::single(0)).with(Format::content(|s| {
                match outcome {
                    Outcome::Positive => s.green().to_string(),
                    Outcome::Negative => s.red().to_string(),
                    Outcome::Neutral => s.bright_blue().to_string(),
                }
            })))
            .with(Modify::new(Columns::single(1)).with(Format::content(|s| {
                match outcome {
                    Outcome::Positive => s.green().to_string(),
                    Outcome::Negative => s.red().to_string(),
                    Outcome::Neutral => s.bright_blue().to_string(),
                }
            })))
    );
}
