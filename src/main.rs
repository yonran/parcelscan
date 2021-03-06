extern crate clap;
extern crate csv;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate parcelscan;

use clap::AppSettings;
use clap::SubCommand;
use clap::{App, Arg};
use csv::Reader;
use parcelscan::sflanduse::LandUseRecord;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;

fn houses_on_standard_lots(mut rdr: Reader<File>) -> Result<(), Box<Error>> {
    let mut num_normal_lots = 0;
    let mut num_lots = 0;
    let mut num_res_lots = 0;
    let mut map: BTreeMap<usize, Vec<LandUseRecord>> = BTreeMap::new();
    for result in rdr.deserialize::<LandUseRecord>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        //        info!("{:?}", record);
        if record.landuse == "RESIDENT" {
            num_res_lots += 1;
        }
        if 2100. < record.shape_area && record.shape_area < 2600. && record.landuse == "RESIDENT" {
            num_normal_lots += 1;
            map.entry(record.resunits).or_insert(vec![]).push(record);
        }
        num_lots += 1;
    }
    info!("{} total lots, {} residential", num_lots, num_res_lots);
    info!("Found {} average-sized lots", num_normal_lots);
    for (units, records) in map.iter() {
        let mut records_sorted_by_date = records.iter().collect::<Vec<_>>();
        records_sorted_by_date.sort_by_key(|record| record.yrbuilt);
        records_sorted_by_date.reverse();
        let records_fmt = records_sorted_by_date
            .iter()
            .take(5)
            .map(|record| {
                format!(
                    "{}-{} {} {} ({}, {})",
                    record
                        .from_st
                        .map(|x| format!("{}", x))
                        .unwrap_or("".to_string()),
                    record
                        .to_st
                        .map(|x| format!("{}", x))
                        .unwrap_or("".to_string()),
                    record.street,
                    record.st_type,
                    record.blklot,
                    record.yrbuilt,
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        info!(
            "{} units: {} records e.g. {}",
            units,
            records.len(),
            records_fmt
        )
    }
    Ok(())
}
fn density_historgram(mut rdr: Reader<File>) -> Result<(), Box<Error>> {
    let mut num_lots = 0;
    let mut num_res_lots = 0;
    let mut map: BTreeMap<i64, Vec<LandUseRecord>> = BTreeMap::new();
    for result in rdr.deserialize::<LandUseRecord>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        //        info!("{:?}", record);
        if record.landuse == "RESIDENT" {
            num_res_lots += 1;
            map.entry((record.resunits as f64 / record.shape_area * 2500. * 4.0).round() as i64)
                .or_insert(vec![])
                .push(record);
        }
        num_lots += 1;
    }
    info!("{} total lots, {} residential", num_lots, num_res_lots);
    for (units_times4, records) in map.iter() {
        let units = *units_times4 as f64 / 4.0;
        let mut records_sorted_by_date = records.iter().collect::<Vec<_>>();
        records_sorted_by_date.sort_by_key(|record| record.yrbuilt);
        records_sorted_by_date.reverse();
        let records_fmt = records_sorted_by_date
            .iter()
            .take(5)
            .map(|record| {
                format!(
                    "{}-{} {} {} ({}, {})",
                    record
                        .from_st
                        .map(|x| format!("{}", x))
                        .unwrap_or("".to_string()),
                    record
                        .to_st
                        .map(|x| format!("{}", x))
                        .unwrap_or("".to_string()),
                    record.street,
                    record.st_type,
                    record.blklot,
                    record.yrbuilt,
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        info!(
            "{} units: {} records e.g. {}",
            units,
            records.len(),
            records_fmt
        )
    }
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    env_logger::init();
    let matches = App::new("parcelscan")
        .version("0.0")
        .about("Scan parcels and print details")
        .author("Yonathan.")
        .after_help("Show stats on sf parcels csv file LandUse2016.csv https://data.sfgov.org/Housing-and-Buildings/Land-Use/us3s-fp9q")
        .arg(Arg::with_name("input")
            .long("input")
            .required(true)
            .takes_value(true)
        )
        .subcommand(SubCommand::with_name("houses-on-standard-lots")
            .about("Show statistics about standard-sized lots")
        )
        .subcommand(SubCommand::with_name("density-historgram")
            .about("Show statistics about all residences")
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    let input = matches.value_of_os("input").expect("Expected input");
    info!("Opening {}", input.to_string_lossy());
    let file = File::open(input)?;
    let rdr = csv::Reader::from_reader(file);
    if let Some(_matches) = matches.subcommand_matches("houses-on-standard-lots") {
        houses_on_standard_lots(rdr)
    } else if let Some(_matches) = matches.subcommand_matches("density-historgram") {
        density_historgram(rdr)
    } else {
        panic!("Should not happen");
    }
}
