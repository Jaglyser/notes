use std::fs::File;
use std::path::Path;
use clap::Parser;
use std::io::{Write, self, BufRead};
use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCrypt256};
mod database;
mod note;
use crate::note::{NoteWoBody, NoteWoTitle, CompleteNote};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    title: Option<String>,
    #[clap(short, long, value_parser)]
    note: Option<String>,
    #[clap(short, long, value_parser, takes_value = false)] 
    list: bool
}

fn main() {
    let args: Args = Args::parse();

    if args.list {
        print_notes();
    } else if args.note.is_some(){
        let note: String = args.note.unwrap();
        let mut data = EncryptedData {
            encrypted_data: note
        };

        data.encrypt_data();
        create_note(data.encrypted_data);
    } 

}

fn print_notes() {
    if let Ok(lines) = read_lines("notes.txt"){
        let mut decrypt = DecryptedData {
            mcrypt: new_magic_crypt!("magickey", 256)
        };
        for line in lines {
            match line {
                Ok(line) => println!("{}", decrypt.decrypt_data(line)),
                Err(error) => panic!("{}", error)
            };
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn create_note(note: String) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true) // This is needed to append to file
        .create(true)
        .open("notes.txt")
        .unwrap();
    file.write_all(format!("{}\n", note).as_bytes())
        .expect("You should have been able to create a note");
}


struct EncryptedData {
    encrypted_data: String,
}

impl EncryptedData {

    fn encrypt_data(&mut self) {
        let mcrypt = new_magic_crypt!("magickey", 256);
        let data = mcrypt.encrypt_str_to_base64(&self.encrypted_data); //Encrypts the string and saves it to the 'encrypted_string' variable.
        self.encrypted_data = data;
    }
}

struct DecryptedData {
    mcrypt: MagicCrypt256,
}

impl DecryptedData{
    fn decrypt_data(&mut self, encrypted_string: String) -> String {
        return self.mcrypt.decrypt_base64_to_string(encrypted_string).unwrap(); //Decrypts the string so we can read it.
    }
}
