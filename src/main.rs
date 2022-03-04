use clap::Parser;
use indextree::Arena;
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

fn parse_org(content: &'static str) -> Org<'static> {
    Org::parse(content)
}

#[test]
fn test_parse() {
    let input = parse_org("** TODO Title :tag:");
    let mut output = Org::new();
    let mut input_writer = Vec::new();
    let mut output_writer = Vec::new();

    let output_doc = output.document();
    let output_props = PropertiesMap::new();

    let output_title = Title {
        raw: "Title".into(),
        // level: 2,
        ..Default::default()
    };

    dbg!(&output_title);

    let mut output_headline = Headline::new(output_title, &mut output);

    dbg!(output_headline.headline_node());
    dbg!(output_headline.level());
    dbg!(output_headline.title_node());

    output_headline.set_level(2, &mut output).unwrap();
    output_doc.append(output_headline, &mut output).ok();

    output.write_html(&mut output_writer).unwrap();
    input.write_html(&mut input_writer).unwrap();

    assert_eq!(
        String::from_utf8(input_writer).unwrap(),
        String::from_utf8(output_writer).unwrap()
    );
}

#[test]
fn test_nodes() {
    // Create a new arena
    let arena = &mut Arena::new();

    // Add some new nodes to the arena
    let a = arena.new_node(1);
    let b = arena.new_node(2);
    let c = arena.new_node(3);

    // Append b to a
    a.append(b, arena);
    assert_eq!(b.ancestors(arena).into_iter().count(), 2);
    assert_eq!(c.ancestors(arena).into_iter().count(), 1);
    // let mut a = b.ancestors(arena);
    // dbg!(a.next().unwrap());
    // dbg!(a.next().unwrap());
}

#[test]
fn test_add_anki_id() {
    let input = r#"** Title
:PROPERTIES:
:Title: Goldberg Variations
:END:
"#;

    let output = r#"** Title
:PROPERTIES:
:Title: Goldberg Variations
:anki_card_id: 1
:END:
"#;

    let mut input = parse_org(input);

    let mut input_writer = Vec::new();

    let doc = input.document();
    let x = doc.first_child(&input).unwrap().title_mut(&mut input);
    let x = x.properties.pairs.push(("anki_card_id".into(), "1".into()));

    input.write_org(&mut input_writer).unwrap();

    assert_eq!(String::from_utf8(input_writer).unwrap(), output.to_string());
}

#[test]
fn test_add_anki_ids() {
    let input = r#"
:PROPERTIES:
:ID:  FileTitle
:END:
* Title
SomeContent
"#;

    let output = r#"
:PROPERTIES:
:ID:  FileTitle
:END:
* Title
:PROPERTIES:
:anki_card_id: 1
:END:
SomeContent
"#;

    let mut input = parse_org(input);

    let mut input_writer = Vec::new();

    let doc = input.document();
    let x = doc.first_child(&input).unwrap().title_mut(&mut input);
    let x = x.properties.pairs.push(("anki_card_id".into(), "1".into()));

    input.write_org(&mut input_writer).unwrap();

    assert_eq!(String::from_utf8(input_writer).unwrap(), output.to_string());
}
