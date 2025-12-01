use crate::structtox::SeqExtract;
use crate::structtox::SeqInfo;
use crate::structtox::SeqStruct;
use crate::tox::read_fasta;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

impl SeqStruct {
    pub fn seqhash(&self) -> Result<HashSet<String>, Box<dyn Error>> {
        let mut hashid: HashSet<String> = HashSet::new();
        let fileopen = File::open(&self.pathfile2).expect("File not present");
        let fileread = BufReader::new(fileopen);
        for i in fileread.lines() {
            let line = i.expect("line not present");
            if !line.starts_with("#") {
                let linevec = line.split("\t").collect::<Vec<_>>();
                hashid.insert(linevec[0].to_string());
            }
        }
        Ok(hashid)
    }

    pub fn seqhashadd(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut seqstore: Vec<String> = Vec::new();
        let fileopen = File::open(&self.pathfile2).expect("File not present");
        let fileread = BufReader::new(fileopen);
        for i in fileread.lines() {
            let line = i.expect("line not present");
            if !line.starts_with("#") {
                seqstore.push(line.clone());
            }
        }
        Ok(seqstore)
    }

    pub fn extractseq(&self) -> Result<Vec<SeqInfo>, Box<dyn Error>> {
        let hashidcloned = self.seqhash().unwrap();
        let hashidsseq = self.seqhashadd().unwrap();
        let mut seqvector: Vec<SeqInfo> = Vec::new();
        for i in hashidcloned.iter() {
            let mut exonvec: Vec<(usize, usize)> = Vec::new();
            let mut cdsvec: Vec<(usize, usize)> = Vec::new();
            let mut proteincodingvec: Vec<(usize, usize)> = Vec::new();
            let mut three_prime_utrvec: Vec<(usize, usize)> = Vec::new();
            let mut five_prime_utrvec: Vec<(usize, usize)> = Vec::new();
            for val in hashidsseq.iter() {
                let valinter = val.split("\t").collect::<Vec<_>>();
                if i == valinter[0] && valinter[3] == "protein_coding_gene" {
                    let value: (usize, usize) = (
                        valinter[4].parse::<usize>().unwrap(),
                        valinter[5].parse::<usize>().unwrap(),
                    );
                    proteincodingvec.push(value);
                } else if i == valinter[0] && valinter[3] == "exon" {
                    let exonpush: (usize, usize) = (
                        valinter[4].parse::<usize>().unwrap(),
                        valinter[5].parse::<usize>().unwrap(),
                    );
                    exonvec.push(exonpush);
                } else if i == valinter[0] && valinter[3] == "CDS" {
                    let cdspush: (usize, usize) = (
                        valinter[4].parse::<usize>().unwrap(),
                        valinter[5].parse::<usize>().unwrap(),
                    );
                    cdsvec.push(cdspush);
                } else if i == valinter[0] && valinter[3] == "three_prime_UTR" {
                    let threeutr: (usize, usize) = (
                        valinter[4].parse::<usize>().unwrap(),
                        valinter[5].parse::<usize>().unwrap(),
                    );
                    three_prime_utrvec.push(threeutr);
                } else if i == valinter[0] && valinter[3] == "five_prime_UTR" {
                    let fiveutr: (usize, usize) = (
                        valinter[4].parse::<usize>().unwrap(),
                        valinter[5].parse::<usize>().unwrap(),
                    );
                    five_prime_utrvec.push(fiveutr);
                } else {
                    continue;
                }
            }
            seqvector.push(SeqInfo {
                name: i.clone(),
                protein_coding: proteincodingvec,
                exon: exonvec,
                cds: cdsvec,
                three_prime: three_prime_utrvec,
                five_prime: five_prime_utrvec,
            });
        }
        Ok(seqvector)
    }

    pub fn extractspecific(&self) -> Result<String, Box<dyn Error>> {
        let file1open = read_fasta(&self.pathfile1).unwrap();
        let extractid = self.extractseq().unwrap();
        let mut seqextract_class: Vec<SeqExtract> = Vec::new();
        for (_val, seq) in file1open.iter() {
            for iterval in extractid.iter() {
                if seq.id == iterval.name {
                    let mut exonextract: Vec<(usize, usize, String)> = Vec::new();
                    let mut cdsextract: Vec<(usize, usize, String)> = Vec::new();
                    let mut proteinextract: Vec<(usize, usize, String)> = Vec::new();
                    let mut threeutrextract: Vec<(usize, usize, String)> = Vec::new();
                    let mut fiveutrextract: Vec<(usize, usize, String)> = Vec::new();
                    for exoni in iterval.exon.iter() {
                        let valueexon: (usize, usize, String) =
                            (exoni.0, exoni.1, seq.seq[exoni.0..exoni.1].to_string());
                        exonextract.push(valueexon);
                    }
                    for cdsi in iterval.cds.iter() {
                        let valuecds: (usize, usize, String) =
                            (cdsi.0, cdsi.1, seq.seq[cdsi.0..cdsi.1].to_string());
                        cdsextract.push(valuecds);
                    }
                    for proteini in iterval.protein_coding.iter() {
                        let valueprotein: (usize, usize, String) = (
                            proteini.0,
                            proteini.1,
                            seq.seq[proteini.0..proteini.1].to_string(),
                        );
                        proteinextract.push(valueprotein);
                    }
                    for threei in iterval.three_prime.iter() {
                        let valuethree: (usize, usize, String) =
                            (threei.0, threei.1, seq.seq[threei.0..threei.1].to_string());
                        threeutrextract.push(valuethree);
                    }
                    for fivei in iterval.five_prime.iter() {
                        let valuefive: (usize, usize, String) =
                            (fivei.0, fivei.1, seq.seq[fivei.0..fivei.1].to_string());
                        fiveutrextract.push(valuefive);
                    }
                    seqextract_class.push(SeqExtract {
                        name: seq.id.clone(),
                        protein_coding: proteinextract,
                        exon: exonextract,
                        cds: cdsextract,
                        three_prime: threeutrextract,
                        five_prime: fiveutrextract,
                    })
                }
            }
        }
        let mut filewrite = File::open("annotationwrite.txt").expect("file not present");
        for val in seqextract_class.iter() {
            writeln!(
                filewrite,
                "{}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
                val.name, val.protein_coding, val.exon, val.cds, val.three_prime, val.five_prime
            )
            .expect("file not present");
        }
        Ok("The file has been written".to_string())
    }
}
