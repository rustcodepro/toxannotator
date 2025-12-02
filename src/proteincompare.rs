use crate::structtox::GenomeTable;
use crate::structtox::ProteinCompareExtract;
use crate::structtox::ProteinEqual;
use crate::structtox::ProteomeRest;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl ProteinCompareExtract {
    pub fn proteincompare(&self) -> Result<String, Box<dyn Error>> {
        let file1open = File::open(self.pathfile1.clone()).expect("file not present");
        let file2open = File::open(self.pathfile2.clone()).expect("file not present");
        let file1read = BufReader::new(file1open);
        let file2read = BufReader::new(file2open);
        let mut filecompare_1: Vec<ProteinEqual> = Vec::new();
        let mut filecompare_2: Vec<ProteinEqual> = Vec::new();
        for i in file1read.lines() {
            let line = i.expect("file not present");
            if !line.starts_with("#") {
                let linevec = line.split("\t").collect::<Vec<_>>();
                if linevec[2] == "protein_coding_gene" {
                    filecompare_1.push(ProteinEqual {
                        name: linevec[8].split(";").collect::<Vec<_>>()[0].replace("ID=", ""),
                        start: linevec[3].parse::<usize>().unwrap(),
                        stop: linevec[4].parse::<usize>().unwrap(),
                        strand: linevec[6].to_string(),
                    })
                }
            }
        }
        for i in file2read.lines() {
            let line = i.expect("file not present");
            if !line.starts_with("#") {
                let linevec = line.split("\t").collect::<Vec<_>>();
                if linevec[2] == "protein_coding_gene" {
                    filecompare_2.push(ProteinEqual {
                        name: linevec[8].split(";").collect::<Vec<_>>()[0].replace("ID=", ""),
                        start: linevec[3].parse::<usize>().unwrap(),
                        stop: linevec[4].parse::<usize>().unwrap(),
                        strand: linevec[6].to_string(),
                    });
                }
            }
        }

        let mut commongenes: Vec<ProteomeRest> = Vec::new();
        let mut dissimilargenes: Vec<ProteomeRest> = Vec::new();
        for i in filecompare_1.iter() {
            for val in filecompare_2.iter() {
                if i.name == val.name {
                    commongenes.push(ProteomeRest {
                        name1: i.name.clone(),
                        name2: val.name.clone(),
                        start1: i.start,
                        start2: val.start,
                        stop1: i.stop,
                        stop2: val.stop,
                        strand1: i.strand.clone(),
                        strand2: val.strand.clone(),
                    });
                }
            }
        }
        for i in filecompare_1.iter() {
            for val in filecompare_2.iter() {
                if i.name != val.name {
                    dissimilargenes.push(ProteomeRest {
                        name1: i.name.clone(),
                        name2: val.name.clone(),
                        start1: i.start,
                        start2: val.start,
                        stop1: i.stop,
                        stop2: val.stop,
                        strand1: i.strand.clone(),
                        strand2: val.strand.clone(),
                    });
                }
            }
        }

        let mut genometablevec: Vec<GenomeTable> = Vec::new();
        for i in commongenes.iter() {
            genometablevec.push(GenomeTable {
                name: i.name1.clone(),
                start1: i.start1,
                start2: i.start2,
                end1: i.stop1,
                end2: i.stop2,
                startdifference: if i.start2 >= i.start1 {
                    (i.start2 - i.start1).to_string()
                } else {
                    (i.start1 - i.start2).to_string()
                },
                enddifference: if i.stop1 >= i.stop2 {
                    (i.stop1 - i.stop2).to_string()
                } else {
                    (i.stop2 - i.stop1).to_string()
                },
            })
        }

        let mut filewrite = File::create("comparison-result.txt").expect("file not present");
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            "GeneName", "Start1", "Start2", "End1", "End2", "Start Difference", "End Difference"
        )
        .expect("line not present");
        for i in genometablevec.iter() {
            writeln!(
                filewrite,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.name, i.start1, i.start2, i.end1, i.end2, i.startdifference, i.enddifference
            )
            .expect("line not present");
        }
        Ok("The comparative file has been written".to_string())
    }
}
