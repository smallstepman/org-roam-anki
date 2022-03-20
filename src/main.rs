#![allow(warnings)]
use clap::Parser;
use indextree::{Arena, NodeId};
use org_roam_anki::parse_org;
use orgize::{elements::*, Document, Headline, Org};
use std::{collections::HashMap, fs, num::NonZeroUsize, str::FromStr};
// use tokio

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let contents = fs::read_to_string(args.path).expect("Something went wrong reading the file");
    let parsed = Org::parse(&contents);
    // println!("{:?}", &parsed);

    for event in Org::parse(&contents).iter() {
        // handling the event
        println!("{:?}", event);
    }
    Ok(())
}

// #[test]
// fn test_parse() {
//     let input = parse_org("** TODO Title :tag:");
//     let mut output = Org::new();
//     let mut input_writer = Vec::new();
//     let mut output_writer = Vec::new();

//     let output_doc = output.document();
//     let output_props = PropertiesMap::new();

//     let output_title = Title {
//         raw: "Title".into(),
//         // level: 2,
//         ..Default::default()
//     };

//     dbg!(&output_title);

//     let mut output_headline = Headline::new(output_title, &mut output);

//     dbg!(output_headline.headline_node());
//     dbg!(output_headline.level());
//     dbg!(output_headline.title_node());

//     output_headline.set_level(2, &mut output).unwrap();
//     output_doc.append(output_headline, &mut output).ok();

//     output.write_html(&mut output_writer).unwrap();
//     input.write_html(&mut input_writer).unwrap();

//     assert_eq!(
//         String::from_utf8(input_writer).unwrap(),
//         String::from_utf8(output_writer).unwrap()
//     );

//     // let src = SourceBlock {
//     //     contents: "fufu".into(),
//     //     language: "ru".into(),
//     //     arguments: "".into(),
//     //     post_blank: 0,
//     // };

//     // let mut output = Org::new();
//     // let mut input_writer = Vec::new();
//     // let mut output_writer = Vec::new();

//     // dbg!(output_headline.headline_node());
//     // dbg!(output_headline.level());
//     // dbg!(output_headline.title_node());

//     // output_headline.set_level(2, &mut output).unwrap();
//     // output_doc.append(output_headline, &mut output).ok();

//     // output.write_html(&mut output_writer).unwrap();
//     // input.write_html(&mut input_writer).unwrap();

//     // assert_eq!(
//     //     String::from_utf8(input_writer).unwrap(),
//     //     String::from_utf8(output_writer).unwrap()
//     // );

//     // let src = SourceBlock {
//     //     contents: "fufu".into(),
//     //     language: "ru".into(),
//     //     arguments: "".into(),
//     //     post_blank: 0,
//     // };
// }

// #[test]
// fn test_nodes() {
//     // Create a new arena
//     let arena = &mut Arena::new();

//     // Add some new nodes to the arena
//     let a = arena.new_node(1);
//     let b = arena.new_node(2);
//     let c = arena.new_node(3);

//     // Append b to a
//     a.append(b, arena);
//     assert_eq!(b.ancestors(arena).into_iter().count(), 2);
//     assert_eq!(c.ancestors(arena).into_iter().count(), 1);
//     // let mut a = b.ancestors(arena);
//     // dbg!(a.next().unwrap());
//     // dbg!(a.next().unwrap());
// }

// #[test]
// fn test_add_anki_id() {
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

//     let mut input = parse_org(input);

//     let mut input_writer = Vec::new();

//     let doc = input.document();
//     let x = doc.first_child(&input).unwrap().title_mut(&mut input);
//     let x = x.properties.pairs.push(("anki_card_id".into(), "1".into()));

//     input.write_org(&mut input_writer).unwrap();

//     assert_eq!(String::from_utf8(input_writer).unwrap(), output.to_string());
// }

// // #[test]
// // fn test_add_anki_ids() {
// //     let input = r#"
// // :PROPERTIES:
// // :ID:  FileTitle
// // :END:
// // * Title
// // SomeContent
// // "#;

// //     let output = r#"
// // :PROPERTIES:
// // :ID:  FileTitle
// // :END:
// // * Title
// // :PROPERTIES:
// // :anki_card_id: 1
// // :END:
// // SomeContent
// // "#;

// //     let mut input = parse_org(input);

// //     let mut input_writer = Vec::new();

// //     for a in input.arena_mut().iter() {
// //         let x = a.first_child().unwrap();
// //     }

// //     let doc = input.document();
// //     let x = doc.first_child(&input).unwrap().title_mut(&mut input);
// //     let x = x.properties.pairs.push(("anki_card_id".into(), "1".into()));

// //     input.write_org(&mut input_writer).unwrap();

// //     assert_eq!(String::from_utf8(input_writer).unwrap(), output.to_string());
// // }

// // V1. parse file
// //  2. split per heading
// // V3. heading.tohtml
// // (4.) html to http json req -> ID
// // V5. heading write ID
// //  |
// //  6. save to file

// #[test]
// fn test_add_anki_ids() {
//     let input = r#"
// :PROPERTIES:
// :ID:  FileTitle
// :END:

// efef
// * Title
// :PROPERTIES:
// :ID:  FileTitle
// :END:
// SomeContent
// ** Faaa
// *** rfrfrf
// :PROPERTIES:
// :ID:  FileTitle
// :END:
// eefef

// efefef

// efef
// efef

// - lista
// - lista
// - lista

// #+begin_src rust
// let a = p;
// #+end_src
// ** rfrf
// * Title
// :PROPERTIES:
// :ID:  FileTitle
// :END:
// SomeContent
// "#;

//     // let mut map = HashMap::new();
//     // base case: end of headline
//     //

//     let input = parse_org("* ASD\ntext");
//     let arena = input.arena();

//     // let f = |n| {NodeId(NonZeroUsize::from_str(n.to_string().as_str()).unwrap()) }
//     for node in arena.iter() {
//         let id: NodeId = arena.get_node_id(&node).unwrap();
//         println!("{}", id);
//         // map.insert(node, v)
//         // Traverse::new(arena, current: node.to_string())
//     }

//     // for a in input.iter() {
//     //     match a {
//     //         orgize::Event::Start(s) => {
//     //             if let Document = s {
//     //                 println!("s:{:?}", s);
//     //             } else if let Headline = s {
//     //                 println!("s:{:?}", s);
//     //             }
//     //         }
//     //         orgize::Event::End(e) => {
//     //             if let Document = e {
//     //                 println!("e:{:?}", e);
//     //             } else if let Headline = e {
//     //                 println!("e:{:?}", e);
//     //             }
//     //         }
//     //     }
//     // }
//     // for x in input.document().children(&input) {
//     //     dbg!("{:?}", x);
//     //     let q = x;
//     // }

//     panic!()
// }

// // #[test]
// // fn test_rayon() {
// //     use rayon::iter::ParallelIterator;
// //     let mut a = indextree::Arena::new();
// //     let n1 = a.new_node(3);
// //     let n2 = a.new_node(4);
// //     let n3 = a.new_node(5);
// //     let n4 = a.new_node(6);
// //     let n5 = a.new_node(7);
// //     let n6 = a.new_node(8);
// //     let n7 = a.new_node(9);
// //     let n8 = a.new_node(10);
// //     let n9 = a.new_node(11);
// //     let n10 = a.new_node(14);
// //     n1.append(n2, &mut a);
// //     n2.append(n3, &mut a);
// //     n2.append(n4, &mut a);
// //     n2.append(n5, &mut a);
// //     n2.append(n6, &mut a);
// //     n3.append(n7, &mut a);
// //     n7.append(n8, &mut a);
// //     n7.append(n9, &mut a);
// //     n7.append(n10, &mut a);

// //     a.par_iter().map(|f| println!("{:?}", f));
// //     panic!()
// // }

// // fn get_content(h: Headline)

// /*

// */
// //     let output = r#"
// // :PROPERTIES:
// // :ID:  FileTitle
// // :anki_card_id: 1
// // :END:
// // efef
// // * Title
// // :PROPERTIES:
// // :anki_card_id: 1
// // :END:
// // SomeContent
// // ** Faaa
// // :PROPERTIES:
// // :anki_card_id: 1
// // :END:
// // *** rfrfrf
// // :PROPERTIES:
// // :anki_card_id: 1
// // :END:
// // ** rfrf
// // :PROPERTIES:
// // :anki_card_id: 1
// // :END:
// // "#;
// // dbg!(&input.arena());
// // let mut s: Vec<orgize::elements::Element> = vec![];
// // // Start Documment -*> Start Headling
// // // Strat Heading -*> End Heading
// // let mut pause = false;
// // for a in input.iter() {
// //     match a {
// //         orgize::Event::Start(s) => {
// //             if let Document = s {
// //                 println!("s:{:?}", s);
// //             } else if let Headline = s {
// //                 println!("s:{:?}", s);
// //             }
// //         }
// //         orgize::Event::End(e) => {
// //             if let Document = e {
// //                 println!("e:{:?}", e);
// //             } else if let Headline = e {
// //                 println!("e:{:?}", e);
// //             }
// //         }
// //     }
// // a.append(hdl, org)
// //         dbg!(a.title(&input));
// //         for c in a.children(&input) {
// //             dbg!(c.next(&input).map(|w| w.section_node()));
// //         }
// // match a {
// //     orgize::Event::Start(s) => {
// //         match s {}
// //         println!("{:?}", s.to_owned())
// //     }
// //     orgize::Event::End(e) => {
// //         println!("end:{:?}", e)
// //     }
// // }
// // ccc
// // for a in input.arena().iter() {
// //     let x: &Element = a.get();
// //     match a.first_child() {
// //         Some(v) => {
// //             dbg!(v
// //                 .ancestors(ca)
// //                 .next()
// //                 .unwrap()
// //                 .children(ca)
// //                 .next()
// //                 .map(|f| println!("{:?}", f.)));
// //         }
// //         None => {}
// //     }
// //     // dbg!(x);

// //     // match &x {

// //     //     // Element::Title(t) => {
// //     //     //     dbg!(t);
// //     //     // }
// //     // }
// //     //     Element::Section => todo!(),
// //     //     Element::Document { pre_blank } => todo!(),
// //     //     Element::Text { value } => todo!(),
// //     //     Element::Paragraph { post_blank } => todo!(),
// // }

// // let mut input_writer = Vec::new();

// // for a in input.arena_mut().iter() {
// //     let x = a.first_child().unwrap();
// // }

// // input
// //     .document()
// //     .first_child(&input)
// //     .unwrap()
// //     .title_mut(&mut input)
// //     .properties
// //     .pairs
// //     .push(("anki_card_id".into(), "1".into()));

// // input.write_org(&mut input_writer).unwrap();

// // assert_eq!(String::from_utf8(input_writer).unwrap(), output.to_string());
