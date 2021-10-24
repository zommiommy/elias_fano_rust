


/// Efficient structure to univocally associate a 
/// compact id to a set of strings while compressing
/// prefixes of these.
pub struct TrieVocabulary {

}

pub struct Node {
    father: usize,
    childs: Vec<usize>,
}

impl TrieVocabulary {

    fn get(&self, key: &str) -> u64 {
        0
    }

    fn insert(&self, key: &str) {

    }

    fn translate(&self, id: u64) -> String {
        "".into()
    }
}