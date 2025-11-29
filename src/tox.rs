use crate::structtox::CompareSeq;
use crate::structtox::FastaStruct;
use crate::structtox::ToxCompare;
use crate::structtox::ToxSummarize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

/*
Gaurav Sablok
codeprog@icloud.com
- a complete tox annotator for comparative genomics.
*/

#[tokio::main]
pub async fn toxsummarize(
    inputpath1: &str,
    inputpath2: &str,
    overlapwindow: &str,
) -> Result<String, Box<dyn Error>> {
    let file1 = File::open(inputpath1).expect("file not present");
    let file2 = File::open(inputpath2).expect("file not present");
    let file1_read = BufReader::new(file1);
    let file2_read = BufReader::new(file2);
    let mut file1_struct: Vec<ToxSummarize> = Vec::new();
    let mut file2_struct: Vec<ToxSummarize> = Vec::new();
    for i in file1_read.lines() {
        let line = i.expect("file not present");
        if line.starts_with("#") {
            continue;
        } else if !line.starts_with("#") {
            let linevec = line.split("\t").collect::<Vec<_>>();
            file1_struct.push(ToxSummarize {
                name: linevec[0].to_string(),
                database: linevec[1].to_string(),
                annotation: linevec[2].to_string(),
                start: linevec[3].parse::<usize>().unwrap(),
                stop: linevec[4].parse::<usize>().unwrap(),
                strand: linevec[6].to_string(),
                id: linevec[8].split(";").collect::<Vec<_>>()[0].to_string(),
                namegene: linevec[8].split(";").collect::<Vec<_>>()[1].to_string(),
                description: linevec[8].split(";").collect::<Vec<_>>()[2].to_string(),
            })
        }
    }

    for file2 in file2_read.lines() {
        let line = file2.expect("file not present");
        if line.starts_with("#") {
            continue;
        } else if !line.starts_with("#") {
            let linevec = line.split("\t").collect::<Vec<_>>();
            file2_struct.push(ToxSummarize {
                name: linevec[0].to_string(),
                database: linevec[1].to_string(),
                annotation: linevec[2].to_string(),
                start: linevec[3].parse::<usize>().unwrap(),
                stop: linevec[4].parse::<usize>().unwrap(),
                strand: linevec[6].to_string(),
                id: linevec[8].split(";").collect::<Vec<_>>()[0].to_string(),
                namegene: linevec[8].split(";").collect::<Vec<_>>()[1].to_string(),
                description: linevec[8].split(";").collect::<Vec<_>>()[2].to_string(),
            })
        }
    }

    let mut protein_coding_gene_1: Vec<String> = Vec::new();
    for i in file1_struct.iter() {
        if i.annotation == "protein_coding_gene" {
            protein_coding_gene_1.push(i.name.clone());
        }
    }
    let mut protein_coding_gene_2: Vec<String> = Vec::new();
    for i in file2_struct.iter() {
        if i.annotation == "protein_coding_gene" {
            protein_coding_gene_2.push(i.name.clone());
        }
    }
    let total_protein_count_1 = protein_coding_gene_1.len();
    let total_protein_count_2 = protein_coding_gene_2.len();

    let mut overlap: Vec<ToxSummarize> = Vec::new();
    for i in file1_struct.iter() {
        for val in file2_struct.iter() {
            if i.start == val.start && i.stop == val.stop {
                overlap.push(ToxSummarize {
                    name: i.name.clone(),
                    database: i.database.clone(),
                    annotation: i.annotation.clone(),
                    start: i.start,
                    stop: i.stop,
                    strand: i.strand.clone(),
                    id: i.id.clone(),
                    namegene: i.namegene.clone(),
                    description: i.description.clone(),
                })
            }
        }
    }

    let mut overlap_strand: Vec<ToxSummarize> = Vec::new();
    for i in file1_struct.iter() {
        for val in file2_struct.iter() {
            if i.start == val.start && i.stop == val.stop && i.strand != val.strand {
                overlap_strand.push(ToxSummarize {
                    name: i.name.clone(),
                    database: i.database.clone(),
                    annotation: i.annotation.clone(),
                    start: i.start,
                    stop: i.stop,
                    strand: i.strand.clone(),
                    id: i.id.clone(),
                    namegene: i.namegene.clone(),
                    description: i.description.clone(),
                })
            }
        }
    }

    let overlap_region = overlapwindow.parse::<usize>().unwrap();
    let comparegenes: Vec<(Vec<ToxCompare>, Vec<ToxCompare>)> = Vec::new();

    /*
    a single loop iteration for the comparison of the struct based comparison,
    a linear implementation of the blanket traits.
    */
    for i in file1_struct.iter() {
        let mut valone: Vec<ToxCompare> = Vec::new();
        let mut valtwo: Vec<ToxCompare> = Vec::new();
        for val in file2_struct.iter() {
            let vali_hold_start = i.start + overlap_region;
            let vali_hold_stop = i.stop + overlap_region;
            if val.start <= vali_hold_start && val.stop <= vali_hold_stop {
                valone.push(ToxCompare {
                    name: i.name.clone(),
                    annotation: i.annotation.clone(),
                    start: i.start,
                    stop: i.stop,
                    strand: i.strand.clone(),
                    id: i.id.clone(),
                    namegene: i.namegene.clone(),
                });
                valtwo.push(ToxCompare {
                    name: val.name.clone(),
                    annotation: val.name.clone(),
                    start: val.start,
                    stop: val.stop,
                    strand: val.strand.clone(),
                    id: val.id.clone(),
                    namegene: val.namegene.clone(),
                });
            }
        }
    }

    println!(
        "The total protein coding regions in the file1: {}",
        total_protein_count_1
    );
    println!(
        "The total protein coding regions in the file2: {}",
        total_protein_count_2
    );

    /*
    implementing a json based file structure so that it can easily used in javascript.
    */

    let mut filewrite = File::open("comparison.txt").expect("file not present");
    for i in comparegenes.iter() {
        for val in i.0.iter() {
            writeln!(
                filewrite,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                val.name, val.annotation, val.start, val.stop, val.strand, val.id, val.namegene
            )
            .expect("file not present");
        }
        writeln!(filewrite, "----------------------------------").expect("file not present");
        for valseq in i.1.iter() {
            writeln!(
                filewrite,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                valseq.name,
                valseq.annotation,
                valseq.start,
                valseq.stop,
                valseq.strand,
                valseq.id,
                valseq.namegene
            )
            .expect("file not present");
        }
    }
    Ok("The summary information has been written".to_string())
}

pub fn read_fasta<P: AsRef<Path>>(path: P) -> std::io::Result<HashMap<String, FastaStruct>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut records = HashMap::new();
    let mut current_id = String::new();
    let mut current_sequence = String::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            if !current_id.is_empty() {
                records.insert(
                    current_id.clone().clone().split("|").collect::<Vec<_>>()[0].to_string(),
                    FastaStruct {
                        id: current_id.clone().split("|").collect::<Vec<_>>()[0].to_string(),
                        seq: current_sequence.clone(),
                        tag: current_id.clone().split("|").collect::<Vec<_>>()[2].to_string(),
                    },
                );
                current_sequence.clear();
            }
            current_id = line[1..].to_string();
        } else {
            current_sequence.push_str(&line);
        }
    }

    if !current_id.is_empty() {
        records.insert(
            current_id.clone().split("|").collect::<Vec<_>>()[0].to_string(),
            FastaStruct {
                id: current_id.clone().split("|").collect::<Vec<_>>()[0].to_string(),
                seq: current_sequence,
                tag: current_id.clone().split("|").collect::<Vec<_>>()[2].to_string(),
            },
        );
    }

    Ok(records)
}

pub fn compare_seq(path_1: &str, path_2: &str) -> Result<String, Box<dyn Error>> {
    let mut compareseq: Vec<CompareSeq> = Vec::new();
    let input1 = read_fasta(path_1).unwrap();
    let input2 = read_fasta(path_2).unwrap();
    for (_i, val) in input1.iter() {
        for (_vali, valseq) in input2.iter() {
            if val.id == valseq.id {
                compareseq.push(CompareSeq {
                    id_1: val.id.clone(),
                    id_1_seq: val.seq.clone(),
                    id_2: valseq.id.clone(),
                    id_2_seq: valseq.seq.clone(),
                });
            }
        }
    }
    let mut filewrite = File::open("compareseq.txt").expect("file not present");
    writeln!(filewrite, "These genes are identical").expect("line not present");
    for i in compareseq.iter() {
        writeln!(
            filewrite,
            "{}\n>{}\n{}\n>{}",
            i.id_1.clone(),
            i.id_1_seq,
            i.id_2.clone(),
            i.id_2_seq.clone()
        )
        .expect("line not present");
    }
    Ok("The file has been written".to_string())
}

pub fn compare_seq_annotation(path_1: &str, path_2: &str) -> Result<String, Box<dyn Error>> {
    let mut compareseq: Vec<CompareSeq> = Vec::new();
    let input1 = read_fasta(path_1).unwrap();
    let input2 = read_fasta(path_2).unwrap();
    for (_i, val) in input1.iter() {
        for (_vali, valseq) in input2.iter() {
            if val.id == valseq.id && val.seq == valseq.seq {
                compareseq.push(CompareSeq {
                    id_1: val.id.clone(),
                    id_1_seq: val.seq.clone(),
                    id_2: valseq.id.clone(),
                    id_2_seq: valseq.seq.clone(),
                });
            }
        }
    }
    let mut filewrite = File::open("compareseqannotation.txt").expect("file not present");
    writeln!(filewrite, "These genes are identical").expect("line not present");
    for i in compareseq.iter() {
        writeln!(
            filewrite,
            "{}\n>{}\n{}\n>{}",
            i.id_1.clone(),
            i.id_1_seq,
            i.id_2.clone(),
            i.id_2_seq.clone()
        )
        .expect("line not present");
    }
    Ok("The file has been written to string".to_string())
}

pub fn un_compare_seq(path_1: &str, path_2: &str) -> Result<String, Box<dyn Error>> {
    let mut compareseq: Vec<CompareSeq> = Vec::new();
    let input1 = read_fasta(path_1).unwrap();
    let input2 = read_fasta(path_2).unwrap();
    for (_i, val) in input1.iter() {
        for (_vali, valseq) in input2.iter() {
            if val.id != valseq.id && val.seq != valseq.seq {
                compareseq.push(CompareSeq {
                    id_1: val.id.clone(),
                    id_1_seq: val.seq.clone(),
                    id_2: valseq.id.clone(),
                    id_2_seq: valseq.seq.clone(),
                });
            }
        }
    }
    let mut filewrite = File::open("uncompareseq.txt").expect("file not present");
    writeln!(filewrite, "These genes are identical").expect("line not present");
    for i in compareseq.iter() {
        writeln!(
            filewrite,
            "{}\n>{}\n{}\n>{}",
            i.id_1.clone(),
            i.id_1_seq,
            i.id_2.clone(),
            i.id_2_seq.clone()
        )
        .expect("line not present");
    }
    Ok("The file has been written to string".to_string())
}
