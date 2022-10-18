use std::format;

pub trait Note {
    fn create_qry(&self) -> String;
}

pub struct NoteWoTitle{
    body: String
}

impl Note for NoteWoTitle{
    #[inline(never)]
    fn create_qry(&self) -> String {
        format!("INSERT INTO notes(note_id, 'title', 'body') values(NULL, NULL, {})", self.body)
    }
}

pub struct NoteWoBody{
    title: String
}

impl Note for NoteWoBody{
    #[inline(never)]
    fn create_qry(&self) -> String {
        format!("INSERT INTO notes(note_id, 'title', 'body') values(NULL, {}, NULL)", self.title)
    } 
}

pub struct CompleteNote{
    title: String,
    body: String
}

impl Note for CompleteNote {
    #[inline(never)]
    fn create_qry(&self) -> String {
        format!("INSERT INTO notes('title, body') values(NULL, {}, {})", self.title, self.body)
    }
}
