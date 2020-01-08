extern crate clap;
extern crate csv;
extern crate env_logger;
extern crate geo;
extern crate geo_types; // the old version used by proj, not the new ones re-exported by geo
extern crate parcelscan;
#[macro_use]
extern crate log;
extern crate num_traits;
extern crate proj;
#[macro_use]
extern crate serde_derive;
extern crate wkt;

use chrono::Datelike;
use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::SubCommand;
use parcelscan::sfplanningacela::PPTSRecord;
use std::error::Error;
use std::fs::File;
use std::io::Read;


#[derive(Clone, Debug, Deserialize, Serialize)]
struct OutputRow {
    description: String,
    address: String,
    units: i32,
    net_units: i32,
    date_opened: String,
    date_closed: Option<String>,
    record_status: String,
    related_building_permit: String,
}

fn expansions(
    mut planning_rdr: csv::Reader<File>,
    mut output_write: Option<csv::Writer<File>>,
) -> Result<(), Box<Error>> {
    info!("Scanning PPTS records of applications");
    for result in planning_rdr.deserialize::<PPTSRecord>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        if record.record_status.contains("Withdrawn") {
            continue;
        }
        if ! (record.record_type_group == "Planning" &&
            record.record_type_type == "Project" &&
            record.record_type_subtype == "Project" &&
            record.record_type_category == "PRJ"
            ) {
            continue;
        }
        if record.related_building_permit == "" {
            continue;
        }
        if ! (record.date_opened.year() >= 2017) {
            continue;
        }
        let units = (record.prj_feature_market_rate_prop.unwrap_or(0f64) +
            record.prj_feature_affordable_prop.unwrap_or(0f64)) as i32;
        if ! (units >= 10) {
            continue;
        }
        let net_units = (record.prj_feature_affordable_net.unwrap_or(0f64) +
            record.prj_feature_market_rate_net.unwrap_or(0f64)) as i32;
        if net_units < 5 {
            continue;
        }
        let o = OutputRow {
            description: record.description,
            address: record.address,
            units,
            net_units,
            date_opened: record.date_opened.date().format("%Y-%m-%d").to_string(),
            date_closed: record.date_closed.map(|d| d.date().format("%Y-%m-%d").to_string()),
            record_status: record.record_status,
            related_building_permit: record.related_building_permit,
        };
        print_row(&o);
        if let Some(output_write) = output_write.as_mut() {
            output_write.serialize(o)?;
        }
    }
    Ok(())
}
fn print_row(o: &OutputRow) {
    println!(
        "opened: {date_opened}, closed: {date_closed}, address: {address}, units: {units}, net units: {net_units}, status: {record_status}, building: {related_building_permit}, description: {description}",
        date_opened = o.date_opened,
        date_closed = o.date_closed.as_ref().map(|x| x.as_str()).unwrap_or("none"),
        address = o.address,
        units = o.units,
        net_units = o.net_units,
        record_status = o.record_status,
        related_building_permit = o.related_building_permit,
        description = o.description,
    );
}

fn main() -> Result<(), Box<Error>> {
    env_logger::init();
    let matches = App::new("approvedapartments")
        .version("0.0")
        .about("Shows apartments that were approved")
        .author("Yonathan.")
        .after_help("Print stats on apartments that were approved since 2017-01-01. Note: everything is output using the logger, so you should set RUST_LOG=approvedapartments=info to see the output.")
        .subcommand(SubCommand::with_name("apartments")
            .arg(Arg::with_name("planning")
                .long("planning")
                .help("Planning CSV file named PPTS_Records_data.csv from https://data.sfgov.org/Housing-and-Buildings/PPTS-Records/7yuw-98m5")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("out-projects")
                .long("out-projects")
                .help("csv file output")
                .takes_value(true)
            )
            .about("Show information about expansions")
        )
        .subcommand(SubCommand::with_name("reprint")
            .arg(Arg::with_name("projects")
                .long("projects")
                .help("csv file input, which was output by apartments")
                .required(true)
                .takes_value(true)
            )
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("apartments") {
        let planning = matches
            .value_of_os("planning")
            .expect("Expected planning file");
        let out_projects_path = matches.value_of_os("out-projects");
        info!(
            "Opening acela: {acela}",
            acela = planning.to_string_lossy()
        );
        let planning_file = File::open(planning)?;
        let planning_rdr = csv::Reader::from_reader(planning_file);
        let out_projects_writer_opt = out_projects_path
            .map(
                |path| -> Result<Option<csv::Writer<File>>, std::io::Error> {
                    info!("Opening output file {}", path.to_string_lossy());
                    let writer: File = File::create(path)?;
                    Ok(Some(csv::Writer::from_writer(writer)))
                },
            )
            .unwrap_or(Ok(None))?;
        expansions(planning_rdr, out_projects_writer_opt)?;
    } else if let Some(matches) = matches.subcommand_matches("reprint") {
        let projects_path = matches.value_of_os("projects").expect("required arg should exist");
        let projects_file: Box<Read> = if projects_path == "-" {
            Box::new(std::io::stdin())
        } else {
            Box::new(File::open(projects_path)?)
        };
        let mut projects_rdr = csv::Reader::from_reader(projects_file);
        for result in projects_rdr.deserialize::<OutputRow>() {
            let o = result?;
            print_row(&o);
        }
    } else {
        panic!("Should not happen");
    }
    Ok(())
}
