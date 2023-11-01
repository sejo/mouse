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
use yaml_rust::{YamlEmitter, YamlLoader};

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
                    serde_json::from_str::<Value>(&IPData {}.gather())
                        .unwrap()
                        .get("ipaddr")
                        .unwrap()
                        .clone(),
                );
            }
            "iproute" => {
                outmap.insert(
                    "iproute".to_string(),
                    serde_json::from_str::<Value>(&IPRouteData {}.gather())
                        .unwrap()
                        .get("iproute")
                        .unwrap()
                        .clone(),
                );
            }
            x => {
                println!("unknown gatherer: {x}");
            }
        };
    }
    match output {
        "json" => serde_json::to_string_pretty(&outmap).unwrap(),
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
    match output_format {
        "json" => println!("{}", data_output),
        "yaml" => {
            // We will run the string back through yaml_rust, for better output compatibility.
            // We could choose to swwitch serde_yaml out completely but at this time I don't see
            // a huge value, as once serde_yaml fixes their implementation,...
            let tmp = format!("---\n{}", data_output);
            let yr = YamlLoader::load_from_str(tmp.as_str()).unwrap();
            let mut out = String::new();
            {
                let mut emitter = YamlEmitter::new(&mut out);
                emitter.dump(&yr[0]).unwrap();
            };
            println!("{}", out)
        }
        x => println!("This should never print,.. but it did. Maybe this explains why: {x}"),
    }
}
