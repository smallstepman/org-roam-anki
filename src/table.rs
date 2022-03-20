use orgize::Org;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
struct TabularizedCards(Vec<Card>);

impl TabularizedCards {
    fn parse(input: &str) -> Self {
        let x = Org::parse(input);
        for e in x.arena().iter() {
            dbg!(e);
        }

        TabularizedCards(vec![Card {
            front: "s".to_string(),
            back: "s".to_string(),
        }])
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
struct Card {
    front: String,
    back: String,
}

// #[test]
// fn basic() {
//     let t = r#"| a | b |
// | e | f |
// | z | x |
// | v | k |
// "#;

//     let o = TabularizedCards::parse(t);

//     let eo = TabularizedCards(vec![]);

//     assert_eq!(eo, o);
// }
