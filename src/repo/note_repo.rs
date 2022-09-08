use crate::models::note_model::Note;
use bson::doc;
use dotenv;
use futures::TryStreamExt;
use mongodb::{error::Error, results::InsertOneResult, Client, Collection};
use std::env;

pub struct NoteRepo {
    pub col: Collection<Note>,
}

impl NoteRepo {
    pub async fn init() -> Self {
        dotenv::dotenv().ok();
        let uri = env::var("MONGOURI").expect("ERROR: no uri found");
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("playground");
        let col: Collection<Note> = db.collection("notes");
        NoteRepo { col }
    }

    pub async fn add_note(&self, note: Note) -> Result<InsertOneResult, Error> {
        let new_note = Note {
            id: None,
            text: note.text,
        };
        self.col.insert_one(new_note, None).await
    }

    pub async fn get_notes(&self) -> Result<Vec<Note>, Error> {
        self.col.find(None, None).await.unwrap().try_collect().await
    }

    pub async fn is_duplicate(&self, note: &String) -> bool {
        matches!(
            self.col
                .find_one(
                    doc!(
                        "text": note.to_owned()
                    ),
                    None,
                )
                .await
                .unwrap(),
            Some(_)
        )
    }
}
