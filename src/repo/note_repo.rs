use crate::models::note_model::Note;
use dotenv;
use futures::TryStreamExt;
use mongodb::{error::Error, results::InsertOneResult, Client, Collection};
use std::env;

pub struct NoteRepo {
    col: Collection<Note>,
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
        let cursor = match self.col.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!(),
        };
        cursor.try_collect().await
    }
}
