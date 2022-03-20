#![allow(warnings, dead_code)]
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::parse_org::AnkifiableSection;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnkiResponse {
    pub result: Option<usize>,
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnkiCreateNoteRequest {
    pub action: String,
    pub version: i64,
    pub params: Params,
}

impl AnkiCreateNoteRequest {
    pub fn new(note: Note) -> Self {
        AnkiCreateNoteRequest {
            action: "addNote".to_string(),
            version: 6,
            params: Params { note },
        }
    }
    pub async fn create_card(
        section: &mut AnkifiableSection,
    ) -> Result<AnkiResponse, Box<dyn Error>> {
        let data = AnkiCreateNoteRequest::new(section.note.clone());
        let client = reqwest::Client::new();
        let resp: AnkiResponse = client
            .post("http://localhost:8765")
            .json(&data)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub note: Note,
}

impl Default for Note {
    fn default() -> Self {
        Note {
            deck_name: "TestRs".to_string(),
            model_name: "Basic".to_string(),
            fields: Fields {
                front: "test".to_string(),
                back: "test".to_string(),
            },
            options: Options::default(),
            tags: vec![],
            audio: vec![],
            video: vec![],
            picture: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub deck_name: String,
    pub model_name: String,
    pub fields: Fields,
    pub options: Options,
    pub tags: Vec<String>,
    pub audio: Vec<Audio>,
    pub video: Vec<Video>,
    pub picture: Vec<Picture>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    #[serde(rename = "Front")]
    pub front: String,
    #[serde(rename = "Back")]
    pub back: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub allow_duplicate: bool,
    pub duplicate_scope: String,
    pub duplicate_scope_options: DuplicateScopeOptions,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            allow_duplicate: false,
            duplicate_scope: "deck".to_string(),
            duplicate_scope_options: DuplicateScopeOptions::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateScopeOptions {
    pub deck_name: String,
    pub check_children: bool,
    pub check_all_models: bool,
}

impl Default for DuplicateScopeOptions {
    fn default() -> Self {
        DuplicateScopeOptions {
            deck_name: "Default".to_string(),
            check_children: false,
            check_all_models: false,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub url: String,
    pub filename: String,
    pub skip_hash: String,
    pub fields: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub url: String,
    pub filename: String,
    pub skip_hash: String,
    pub fields: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Picture {
    pub url: String,
    pub filename: String,
    pub skip_hash: String,
    pub fields: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task::spawn_blocking;

    #[tokio::test]
    async fn basic() {
        let v = spawn_blocking(|| async {
            let mut a = AnkifiableSection::default();
            let r = AnkiCreateNoteRequest::create_card(&mut a).await.unwrap();
            r.clone()
        });
        dbg!(v.await.unwrap().await);
        assert!(false);
    }
}
