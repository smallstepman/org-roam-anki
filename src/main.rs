#![allow(warnings)]

mod anki;
mod cli;
mod parse_org;

use cli::Args;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::get_args();
    let contents = fs::read_to_string(args.path).expect("Something went wrong reading the file");
    // let parsed = Org::parse(&contents);
    // // println!("{:?}", &parsed);

    // for event in Org::parse(&contents).iter() {
    //     // handling the event
    //     println!("{:?}", event);
    // }
    Ok(())
}
