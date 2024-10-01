use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    // 显示reader的信息
    // println!("{:?}", reader);
    // println!("===============================");

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
        // 显示record的信息
        // println!("{:?}", record);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
