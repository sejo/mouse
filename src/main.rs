mod types;
mod gatherers;
mod util;

use std::collections::HashSet; 
use crate::types::fact::{Fact, FactData};
use crate::gatherers::environment::EnvironmentData;
use crate::gatherers::ip::IPData;
use clap::Parser;
use csv::Writer;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Specific gatherer you wish to run, defaults to all
    #[arg(short, long)]
    gatherer: Option<Vec<String>>,
    /// Output format, yaml, json are supported
    #[arg(short, long, default_value="yaml")]
    output: String,
}

fn gather_all() -> Vec<FactData>{
    let mut data: Vec<FactData> = vec![];
    data.append(&mut gather(&EnvironmentData{}));
    data.append(&mut gather(&IPData{}));
    return data;
}

fn gather_list(gatherers: HashSet<String>) -> Vec<FactData> {
    let mut data: Vec<FactData> = vec![];
    for g in gatherers {
        match g.as_str() {
            "all" => for d in gather_all(){
                data.push(d);
            },
            "env" => for d in gather(&EnvironmentData{}) {
                data.push(d); 
            },
            "ip" => for d in gather(&IPData{}) {
                data.push(d);
            }
            _ => for d in gather_all() {
                data.push(d);
            }      
        };
    }
    return data;
}
fn gather (t: &dyn Fact) -> Vec<FactData> {
    return t.gather();
}
fn main() {
    let args = Args::parse();
    let mut data:Vec<FactData> = vec![];

    if args.gatherer.is_none() {
        data.append(&mut gather_all());
    } else {
        // sanitize the gatherer list
        let ugatherers: HashSet<String> = match &args.gatherer {
            Some(x) => x.into_iter().map(|name| name).cloned().collect::<HashSet<String>>(),
            None => HashSet::<String>::new(),
        };
        data.append(&mut gather_list(ugatherers));
       }
    match args.output.as_str() {
        "json" => {
            let serialized_json = serde_json::to_string(&data).unwrap();
            println!("{}", serialized_json);
        },
        "csv" => {
            let mut writer = Writer::from_writer(vec![]);
            for fd in data {
                writer.serialize(fd).unwrap();
            }
            println!("{}", String::from_utf8(writer.into_inner().unwrap()).unwrap());
        }
        &_ =>  {
            let serialized_yaml = serde_yaml::to_string(&data).unwrap();
            println!("{}", serialized_yaml);
        },
    };
}
