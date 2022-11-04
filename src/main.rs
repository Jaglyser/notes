mod database;
mod note;
mod args;
mod encryption;

use std::path::Path;
use clap::Parser;
use crate::database::{connect};
use crate::note::{NoteWoBody, NoteWoTitle, CompleteNote, create_note};
use crate::args::Args;
use crate::encryption::{EncryptedData, DecryptedData};

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    connect().await;

    if args.list {
        println!("print list of note titles");
    } else if args.note.is_some(){
        let note: String = args.note.unwrap();
        let mut data = EncryptedData {
            encrypted_data: note
        };
        data.encrypt_data();
    } 

}
