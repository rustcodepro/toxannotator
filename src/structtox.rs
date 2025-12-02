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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ToxPath {
    pub filepath1: String,
    pub filepath2: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PathFile {
    pub inputpath: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct SeqStruct {
    pub pathfile1: String,
    pub pathfile2: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct SeqInfo {
    pub name: String,
    pub protein_coding: Vec<(usize, usize)>,
    pub exon: Vec<(usize, usize)>,
    pub cds: Vec<(usize, usize)>,
    pub three_prime: Vec<(usize, usize)>,
    pub five_prime: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct SeqExtract {
    pub name: String,
    pub protein_coding: Vec<(usize, usize, String)>,
    pub exon: Vec<(usize, usize, String)>,
    pub cds: Vec<(usize, usize, String)>,
    pub three_prime: Vec<(usize, usize, String)>,
    pub five_prime: Vec<(usize, usize, String)>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Extractplot {
    pub pathfile1: String,
    pub pathfile2: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ProteinCompareExtract {
    pub pathfile1: String,
    pub pathfile2: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ProteinEqual {
    pub name: String,
    pub start: usize,
    pub stop: usize,
    pub strand: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ProteomeRest {
    pub name1: String,
    pub name2: String,
    pub start1: usize,
    pub start2: usize,
    pub stop1: usize,
    pub stop2: usize,
    pub strand1: String,
    pub strand2: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GenomeTable {
    pub name: String,
    pub start1: usize,
    pub start2: usize,
    pub end1: usize,
    pub end2: usize,
    pub startdifference: String,
    pub enddifference: String,
}
