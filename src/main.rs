#![allow(warnings)]
use clap::Parser;
use indextree::Arena;
use org_roam_anki::parse_org;
use orgize::{elements::*, Headline, Org};
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.path).expect("Something went wrong reading the file");
    let parsed = Org::parse(&contents);
    // println!("{:?}", &parsed);

    for event in Org::parse(&contents).iter() {
        // handling the event
        println!("{:?}", event);
    }
}
