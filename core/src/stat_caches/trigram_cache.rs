use fxhash::FxHashMap as HashMap;

#[derive(Clone, Debug)]
pub struct TrigramCache {
    bigram_trigrams: HashMap<[u8; 2], Vec<([u8; 3], i64)>>,
}
