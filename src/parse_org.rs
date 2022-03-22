#![allow(warnings)]
use crate::anki::Note;
use indextree::{Arena, NodeId};
use orgize::{elements::*, Document, Headline, Org};
use serde_json::map::IterMut;
use std::{fs, num::NonZeroU32};

pub struct OrgFile<'a> {
    pub sections: Vec<AnkifiableSection>,
    file: Org<'a>,
}

impl OrgFile<'static> {
    pub fn parse(path: String) -> Self {
        let file_content = fs::read_to_string(path).expect("Something went wrong reading the file");
        let org = Org::parse_string(file_content);
        OrgFile {
            sections: vec![],
            file: org,
        }
    }
    pub fn scrape_top(&mut self) {}
    pub fn scrape_headings(&mut self) {}
    pub fn filter_tags(&mut self, tags: Vec<String>) {}
}

// impl<'a> Iterator for OrgFile<'a> {
//     type Item = &'a mut AnkifiableSection;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.sections.iter_mut().next()
//     }
// }

pub struct AnkifiableSection {
    pub arena_id: NodeId,
    pub note: Note,
    pub org_id: Option<Id>,
    pub anki_id: Option<usize>,
}

impl Default for AnkifiableSection {
    fn default() -> Self {
        Self {
            arena_id: {
                let arena = &mut Arena::new();
                let node = arena.new_node(0);
                let node = arena.get(node).unwrap();
                arena.get_node_id(node).unwrap()
            },
            ..Default::default()
        }
    }
}

pub enum Id {
    Top(String),
    Heading(String),
}

pub fn parse_org(content: &'static str) -> Org<'static> {
    Org::parse(content)
}
