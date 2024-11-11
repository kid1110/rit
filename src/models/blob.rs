use hex::{self, encode};
use sha1::{Digest, Sha1};
#[derive(Debug, Clone)]
pub struct Blob {
    hash: String,
    data: String,
}

impl Blob {
    pub fn new(data: &str) -> Blob {
        let hash = Blob::encode(data);
        Blob {
            hash,
            data: data.to_string(),
        }
    }
    pub fn encode(data: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(data);
        let result = hasher.finalize();
        encode(result)
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    pub fn get_data(&self) -> String {
        self.data.clone()
    }
    pub fn save(&self) {
        // generate new directory
        let fold_name = &self.hash[..2];
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_new_blob() {}
}
