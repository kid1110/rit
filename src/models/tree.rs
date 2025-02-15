#[derive(Debug, Clone)]
pub struct Entry {
    name: String,
    mode: u32,
    hash: String,
}
impl Entry {
    pub fn new(name: &str, mode: u32, hash: &str) -> Entry {
        Entry {
            name: name.to_string(),
            mode,
            hash: hash.to_string(),
        }
    }
}


#[derive(Debug)]
pub  struct Tree {
    hash: String,
    entries: Vec<Entry>,
}