#![allow(warnings)]
use indextree::{Arena, Node, NodeEdge, NodeId};
use org_roam_anki::parse_org;
use orgize::{elements::*, Headline, Org};
use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    anki_id: Option<u64>,
    path: Vec<String>,
    content: String, // front: String,
                     // back: String,
                     // tags: Vec<String>
}

impl Card {
    fn new(path: Vec<String>, content: String) -> Self {
        Card {
            anki_id: None,
            path,
            content,
        }
    }
}

type Cards = HashMap<NodeId, Card>;

#[test]
fn test_parse_org_str() {
    let input = parse_org("** TODO Title :tag:");
    let mut output = Org::new();
    let output_doc = output.document();
    let mut input_writer = Vec::new();
    let mut output_writer = Vec::new();

    let mut output_headline = Headline::new(
        Title {
            raw: "Title".into(),
            ..Default::default()
        },
        &mut output,
    );

    output_headline.set_level(2, &mut output).unwrap();
    output_doc.append(output_headline, &mut output).ok();

    output.write_html(&mut output_writer).unwrap();
    input.write_html(&mut input_writer).unwrap();

    assert_eq!(
        String::from_utf8(input_writer).unwrap(),
        String::from_utf8(output_writer).unwrap()
    );
}

fn get_section_content(s: &Node<Element>, a: &Arena<Element>) -> Option<String> {
    let section_p = s.first_child().unwrap();
    let section_p_node = a.get(section_p);
    if let Some(c) = section_p_node {
        let t = a.get(c.first_child().unwrap()).unwrap().get();
        match t {
            Element::Text { value } => Some(value.to_string()),
            _ => None,
        }
    } else {
        None
    }
}

fn create_anki_card(h: Headline, o: &Org, mut cards: Cards, mut path: Vec<String>) -> Cards {
    // add current headline to path
    path.push(h.title(&o).raw.to_string());
    // if the current headline has some content, create a new anki card
    if let Some(section_id) = h.section_node() {
        let section_node = o.arena().get(section_id).unwrap();
        let section_content = get_section_content(section_node, &o.arena()).unwrap();

        cards.insert(
            h.title_node(),
            Card::new(path.clone(), section_content.to_string()),
        );
    }

    // if the headline has some children, create anki cards for them as well
    for c in h.children(o).into_iter() {
        cards = create_anki_card(c, o, cards, path.clone());
    }
    cards
}

#[test]
fn test_add_anki_id() {
    //     let input = r#"** Title
    // :PROPERTIES:
    // :Title: Goldberg Variations
    // :END:
    // "#;

    //     let output = r#"** Title
    // :PROPERTIES:
    // :Title: Goldberg Variations
    // :anki_card_id: 1
    // :END:
    // "#;

    let input = r#"** Title"#;
    let output = r#"** Title
:PROPERTIES:
:anki_card_id: 1
:END:
"#;

    let mut input = parse_org(input);
    let input_doc = input.document();
    let mut input_writer = Vec::new();

    let title = input_doc.first_child(&input).unwrap().title_mut(&mut input);
    title
        .properties
        .pairs
        .push(("anki_card_id".into(), "1".into()));

    input.write_org(&mut input_writer).unwrap();

    assert_eq!(String::from_utf8(input_writer).unwrap(), output.to_string());
}

#[test]
fn test_create_cards_map() {
    //     let input = r#"
    // :PROPERTIES:
    // :ID: 123
    // :Title: somefile
    // :END:
    // * h1
    // h1 content
    // ** h2a
    // h1h2a content
    // *** h3
    // h1h2ah3 content
    // ** h2b
    // h1h2b content
    // "#;

    let input = r#"
:PROPERTIES:
:ID: 123
:END:
#+Title: somefile
somefile content
#+begin_src rust
#+end_src

* h1
h1 content
** h2
h1h2 content
"#;

    let mut cards: Cards = HashMap::new();
    let input = parse_org(input);
    let doc = input.document();

    let headlines = doc.children(&input);

    for h in headlines {
        cards = create_anki_card(h, &input, cards, vec![]);
    }

    // dbg!(&cards);
    let file_section = doc.section_node().unwrap();
    let file_section_node = input.arena().get(file_section).unwrap();

    let t = file_section.traverse(input.arena());
    for x in t {
        match x {
            NodeEdge::Start(i) => {
                let node = input.arena().get(i).unwrap();
                match node.get() {
                    Element::Drawer(_) => {
                        dbg!(get_section_content(&node, input.arena()));
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    for x in input.iter() {
        match x {
            orgize::Event::Start(v) => println!("start: {:?}", v),
            orgize::Event::End(v) => println!("end: {:?}", v),
        }
    }

    // check number of cards in the hashmap
    assert_eq!(*&cards.len(), input.headlines().count());
    // check that each card has a matching path and content
    for (titleId, card) in &cards {
        assert_eq!(format!("{} content", card.path.join("")), card.content);
    }

    panic!();
}

//     let output = r#"
// :PROPERTIES:
// :ID:  FileTitle
// :END:
// * H1
// :PROPERTIES:
// :org_anki_id: 1
// :END:
// H1Content
// ** H2
// :PROPERTIES:
// :org_anki_id: 2
// :END:
// H2Content
// *** H3
// :PROPERTIES:
// :org_anki_id: 3
// :END:
// H3Content
// ** H2a
// :PROPERTIES:
// :org_anki_id: 4
// :END:
// H2aContent
// "#;
// start: Document { pre_blank: 1 }
// start: Section
// start: Drawer(Drawer { name: "PROPERTIES", pre_blank: 0, post_blank: 0 })
// start: Paragraph { post_blank: 0 }
// start: Text { value: ":ID: 123\n:Title: somefile" }
