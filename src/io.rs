use std::io;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use strum_macros::{EnumString, EnumVariantNames};

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Format {
    Csv,
    Tsv,
    Json,
    PJson,
    Yaml,
}

pub fn read<T>(path: &PathBuf) -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let r: Result<Vec<_>, _> = csv::Reader::from_path(path)
        .with_context(|| format!("{:?} が読み込めませんでした", &path.to_str()))?
        .deserialize()
        .collect();
    r.with_context(|| format!("{:?} のパースに問題が発生しました", &path.to_str()))
}

pub fn write<T>(records: &[T], format: &Format) -> Result<()>
where
    T: Serialize,
{
    match format {
        Format::Csv => write_csv(records, b','),
        Format::Tsv => write_csv(records, b'\t'),
        Format::Json => write_json(records),
        Format::PJson => write_pretty_json(records),
        Format::Yaml => write_yaml(records),
    }?;
    Ok(())
}

fn write_csv<T>(records: &[T], delimiter: u8) -> Result<()>
where
    T: Serialize,
{
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .from_writer(io::stdout());
    for r in records {
        wtr.serialize(r)?;
        wtr.flush()?;
    }
    Ok(())
}

fn write_json<T>(records: &[T]) -> Result<()>
where
    T: Serialize,
{
    serde_json::to_writer(io::stdout(), records)?;
    Ok(())
}

fn write_pretty_json<T>(records: &[T]) -> Result<()>
where
    T: Serialize,
{
    serde_json::to_writer_pretty(io::stdout(), records)?;
    Ok(())
}

fn write_yaml<T>(records: &[T]) -> Result<()>
where
    T: Serialize,
{
    serde_yaml::to_writer(io::stdout(), records)?;
    Ok(())
}
