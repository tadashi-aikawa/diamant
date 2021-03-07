use std::io;
use strum_macros::EnumString;

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum IOType {
    CSV,
    TSV,
    JSON,
    PJSON,
    YAML,
}

pub fn read<T>(path: &PathBuf) -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let r: Result<Vec<_>, _> = csv::Reader::from_path(path)
        .with_context(|| format!("Could not read file {:?}", &path.to_str()))?
        .deserialize()
        .collect();
    r.with_context(|| "エラー")
}

pub fn write<T>(records: &[T], output_type: &IOType) -> Result<()>
where
    T: Serialize,
{
    match output_type {
        IOType::CSV => write_csv(records, b','),
        IOType::TSV => write_csv(records, b'\t'),
        IOType::JSON => write_json(records),
        IOType::PJSON => write_pretty_json(records),
        IOType::YAML => write_yaml(records),
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
