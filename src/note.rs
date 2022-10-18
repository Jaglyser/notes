use std::format;

pub trait Note {
    fn create_qry(&self);
}

pub struct NoteWoTitle{
    body: String
}
pub impl Note for NoteWoTitle{
    #[inline(never)]
    fn create_qry(&self) -> String {
        format!("INSERT INTO notes('body') values({})", self.body);
    }
}
pub struct NoteWoBody{
    title: String
}
pub impl Note for NoteWoBody{
    #[inline[never]]
    fn create_qry(&self) -> String {
        format!("INSERT INTO notes('title') values({})", self.title);
    }
}
pub struct CompleteNote{
    title: String,
    body: String
}
pub impl Note for Completenote{
    #[inline[never]]
    fn create_qry(&self) -> String {
        format!("INSERT INTO notes('title, body') values({}, {})", self.title, self.body);
    }
}
