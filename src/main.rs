// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod gatherers;
mod types;
mod util;

#[macro_use]
extern crate serde_json;

use crate::gatherers::environment::EnvironmentData;
use crate::gatherers::ip::{IPData, IPRouteData};
use crate::types::fact::Fact;
use clap::Parser;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Specific gatherer you wish to run, defaults to all
    #[arg(short, long)]
    gatherer: Option<Vec<String>>,
    /// Output format, yaml, json are supported
    #[arg(short, long, default_value = "yaml")]
    output: String,
}

fn gather_list(gatherers: HashSet<String>, output: &str) -> String {
    let mut outmap: HashMap<String, Value> = HashMap::new();
    for g in gatherers {
        match g.as_str() {
            "env" => {
                outmap.insert(
                    "environment".to_string(),
                    serde_json::from_str(&EnvironmentData {}.gather()).unwrap(),
                );
            }
            "ipaddr" => {
                outmap.insert(
                    "ipaddr".to_string(),
                    serde_json::from_str(&IPData {}.gather()).unwrap(),
                );
            }
            "iproute" => {
                outmap.insert(
                    "iproute".to_string(),
                    serde_json::from_str(&IPRouteData {}.gather()).unwrap(),
                );
            }
            x => {
                println!("unknown gatherer: {x}");
            }
        };
    }
    match output {
        "json" => serde_json::to_string(&outmap).unwrap(),
        "yaml" => serde_yaml::to_string(&outmap).unwrap(),
        x => format!("Unknown output format {x}"),
    }
}
fn main() {
    let args = Args::parse();
    let output_format = match args.output.as_str() {
        "json" => "json",
        "yaml" => "yaml",
        _ => "yaml",
    };
    let all: HashSet<String> = HashSet::from([
        "env".to_string(),
        "ipaddr".to_string(),
        "iproute".to_string(),
    ]);
    let data_output: String = match args.gatherer {
        None => gather_list(all, output_format),
        Some(x) => {
            let gatherers: HashSet<String> = x.into_iter().collect::<HashSet<String>>();
            gather_list(gatherers, output_format)
        }
    };
    println!("{}", data_output);
}
