#![allow(warnings)]

mod anki;
mod cli;
mod parse_org;
mod table;

use anki::AnkiCreateNoteRequest;
use cli::Args;
use parse_org::OrgFile;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::get_args();
    let mut org = OrgFile::parse(args.path);
    org.scrape_headings();
    org.scrape_top();
    org.filter_tags(args.ignore_tags);
    for s in org.sections.iter_mut() {
        AnkiCreateNoteRequest::create_card(s);
    }
    // let parsed = Org::parse(&contents);
    // // println!("{:?}", &parsed);

    // for event in Org::parse(&contents).iter() {
    //     // handling the event
    //     println!("{:?}", event);
    // }
    Ok(())
}
