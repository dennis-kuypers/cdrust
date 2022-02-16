use crate::prelude::*;
use figment::Figment;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(StructOpt, Debug)]
pub struct OutputOpts {
    #[structopt(long)]
    /// Turns off colored output.
    pub no_color: bool,

    #[structopt(long)]
    /// Turns off pretty formatting.
    pub no_pretty: bool,

    #[structopt(short, long)]
    /// Auto-respond confirmations with 'yes'
    pub yes: bool,

    #[structopt(short, long, conflicts_with = "one")]
    /// Automatically select all possible items
    pub all: bool,

    #[structopt(short = "1", long)]
    /// Automatically select the first item
    pub one: bool,

    #[structopt(short, long)]
    /// Controls output formatting
    pub output: Option<OutputFormat>,
}

#[derive(Deserialize)]
pub struct Config {
    pub format: Option<OutputFormat>,
}

pub struct _Output {
    _colorize: bool,
    pretty: bool,
    _confirm_all: bool,
    output: OutputFormat,
}

#[derive(StructOpt, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Json,
    Yaml,
    Csv,
    Table,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = serde_variant::to_variant_name(&self).map_err(|_| std::fmt::Error)?;
        f.write_str(name)
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "csv" => Ok(OutputFormat::Csv),
            "table" => Ok(OutputFormat::Table),
            _ => Err(String::from("Unknown output type")),
        }
    }
}

/// The output / printer
pub struct Output(_Output);
impl Output {
    pub fn new(opts: OutputOpts, config: Config) -> Self {
        Self(_Output {
            _colorize: !opts.no_color,
            pretty: !opts.no_pretty,
            _confirm_all: opts.yes,
            output: opts.output.or(config.format).unwrap_or(OutputFormat::Json),
        })
    }
    pub fn plaintext(self) -> anyhow::Result<PlainTextOutput> {
        Ok(PlainTextOutput(self.0))
    }
    pub fn structured(self) -> anyhow::Result<StructuredOutput> {
        Ok(StructuredOutput(self.0))
    }
}

#[derive(new)]
pub struct PlainTextOutput(_Output);
impl PlainTextOutput {
    pub fn println(&self, s: &str) {
        eprintln!("{}", s)
    }
}

#[derive(new)]
pub struct StructuredOutput(_Output);
impl StructuredOutput {
    /// Consumes output and prints object
    pub fn print(self, s: impl serde::Serialize) {
        match self.0.output {
            OutputFormat::Json => self.print_json(s),
            // OutputFormat::Table => self.print_table(s),
            _ => todo!(),
        }
    }

    // fn print_table(self, s: impl serde::Serialize) {
    //     let mut column_collector = table::TableSerializer::new();
    //     s.serialize(column_collector.borrow_mut()).unwrap();
    // }
    fn print_json(self, s: impl serde::Serialize) {
        let w = std::io::stdout();
        if self.0.pretty {
            serde_json::to_writer_pretty(w, &s).expect("write json to stdout");
        } else {
            serde_json::to_writer(w, &s).expect("write json to stdout");
        }
    }
}

/// Prepares output singleton
pub fn build_output(opts: OutputOpts, x: &Figment) -> anyhow::Result<Output> {
    let ceres_config = x.focus("ceres").extract().expect("ceres configuration is missing");
    Ok(Output::new(opts, ceres_config))
}
