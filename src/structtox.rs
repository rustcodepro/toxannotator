#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ToxSummarize {
    pub name: String,
    pub database: String,
    pub annotation: String,
    pub start: usize,
    pub stop: usize,
    pub strand: String,
    pub id: String,
    pub namegene: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct FastaStruct {
    pub id: String,
    pub seq: String,
    pub tag: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CompareSeq {
    pub id_1: String,
    pub id_1_seq: String,
    pub id_2: String,
    pub id_2_seq: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ToxCompare {
    pub name: String,
    pub annotation: String,
    pub start: usize,
    pub stop: usize,
    pub strand: String,
    pub id: String,
    pub namegene: String,
}
