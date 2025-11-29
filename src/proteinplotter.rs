use crate::structtox::ToxPath;
use crate::tox::read_fasta;
use charts_rs::BarChart;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, Write};
use tokio::io::BufReader;
use tokio::sync::watch::Ref;

/*
Gaurav Sablok
codeprog@icloud.com

comparative annotation plotters

*/

impl ToxPath {
    pub fn proteinplotter(&self) -> (Vec<(String, usize)>, Vec<(String, usize)>) {
        let file1_open = File::open(&self.fileinput1).expect("file not present");
        let file2_open = File::open(&self.filepath2).expect("file not present");
        let file1_read = BufReader::new(file1_open);
        let file2_read = Bufreader::new(file2_open);
        let file1vec: Vec<(String, usize, usize)> = Vec::new();
        let file2vec: Vec<(String, usize, usize)> = Vec::new();
        for i in file1_read.lines() {
            let line = i.expect("line not present");
            if line.starts_with("#") {
                continue;
            } else if !line.starts_with("#") {
                let lineec = line.split("\t").collect::<Vec<_>>();
                let inserttuple: (String, usize, usize) = (
                    linevec[0].to_string(),
                    linevec[3].parse::<usize>().unwrap(),
                    linevec[4].parse::<usize>().unwrap(),
                );
                file1vec.push(inserttuple);
            }
        }
        for i in file2_read.lines() {
            let line = i.expect("line not present");
            if line.starts_with("#") {
                continue;
            } else if !line.starts_with("#") {
                let lineec = line.split("\t").collect::<Vec<_>>();
                let inserttuple: (String, usize, usize) = (
                    linevec[0].to_string(),
                    linevec[3].parse::<usize>().unwrap(),
                    linevec[4].parse::<usize>().unwrap(),
                );
                file2vec.push(inserttuple);
            }
        }
        let filecoordinates_1_plot: Vec<(String, usize)> = Vec::new();
        let filecoordinates_2_plot: Vec<(String, usize)> = Vec::new();
        for i in file1vec.iter() {
            let valueinsert: (String, usize) = (i.0, (i.2 - i.1));
            filecoordinates_1_plot.push(valueinsert);
        }
        for i in file2vec.iter() {
            let valueinsert: (String, usize) = (i.0, (i.2 - i.1));
            filecoordinates_2_plot.push(valueinsert);
        }
        (filecoordinates_1_plot, filecoordinates_2_plot)
    }

    pub fn unpackseq(
        &self,
        path1: &str,
        path2: &str,
    ) -> Result<(
        Vec<(String, String, usize, usize)>,
        Vec<(String, String, usize, usize)>,
        Box<dyn Error>,
    )> {
        let file1open = File::open(&self.filepath1).expect("file not present");
        let file2open = File::open(&self.filepath2).expect("file not present");
        let file1fasta = read_fasta(path1).unwrap();
        let file2fasta = read_fasta(path2).unwrap();
        let returnvec_1: Vec<(String, String, usize, usize)> = Vec::new();
        let returnvec_2: Vec<(String, String, usize, usize)> = Vec::new();
        let file1read = BufReader::new(file1open);
        let file2read = BufReader::new(file2open);
        for i in file1read.lines() {
            for (val, seq) in file1fasta.iter() {
                let line = i.expect("line not present");
                let linevec = line.split("\t").collect::<Vec<_>>();
                if linevec[8].split(";").collect::<Vec<_>>()[0]
                    .to_string()
                    .replace("ID=", "")
                    .to_string()
                    == val.clone()
                {
                    let tupleinsert: (String, String, usize, usize) = (
                        seq.id,
                        seq.seq,
                        linevec[3].parse::<usize>().unwrap(),
                        linevec[4].parse::<usize>().unwrap(),
                    );
                    returnvec_1.push(tupleinsert);
                }
            }
        }
        for genomseq in file2read.lines() {
            for (val, seq) in file2fasta.iter() {
                let line = genomeseq.expect("line not present");
                let linevec = line.split("\t").collect::<Vec<_>>();
                if linevec[8].split(";").collect::<Vec<_>>()[0]
                    .to_string()
                    .replace("ID=", "")
                    .to_string()
                    == val.clone()
                {
                    let tupleinsert: (String, String, usize, usize) = (
                        seq.id,
                        seq.seq,
                        linevec[3].parse::<usize>().unwrap(),
                        linevec[4].parse::<usize>().unwrap(),
                    );
                    returnvec_2.push(tupleinsert);
                }
            }
        }
        Ok((returnvec_1, returnvec_2))
    }

    pub fn unpackseq_diff(
        &self,
        path1: &str,
        path2: &str,
    ) -> Result<(
        Vec<(String, String, usize, usize)>,
        Vec<(String, String, usize, usize)>,
        Box<dyn Error>,
    )> {
        let file1open = File::open(&self.filepath1).expect("file not present");
        let file2open = File::open(&self.filepath2).expect("file not present");
        let file1fasta = read_fasta(path1).unwrap();
        let file2fasta = read_fasta(path2).unwrap();
        let returnvec_1: Vec<(String, String, usize, usize)> = Vec::new();
        let returnvec_2: Vec<(String, String, usize, usize)> = Vec::new();
        let file1read = BufReader::new(file1open);
        let file2read = BufReader::new(file2open);
        for i in file1read.lines() {
            for (val, seq) in file1fasta.iter() {
                let line = i.expect("line not present");
                let linevec = line.split("\t").collect::<Vec<_>>();
                if linevec[8].split(";").collect::<Vec<_>>()[0]
                    .to_string()
                    .replace("ID=", "")
                    .to_string()
                    != val.clone()
                {
                    let tupleinsert: (String, String, usize, usize) = (
                        seq.id,
                        seq.seq,
                        linevec[3].parse::<usize>().unwrap(),
                        linevec[4].parse::<usize>().unwrap(),
                    );
                    returnvec_1.push(tupleinsert);
                }
            }
        }
        for genomseq in file2read.lines() {
            for (val, seq) in file2fasta.iter() {
                let line = genomeseq.expect("line not present");
                let linevec = line.split("\t").collect::<Vec<_>>();
                if linevec[8].split(";").collect::<Vec<_>>()[0]
                    .to_string()
                    .replace("ID=", "")
                    .to_string()
                    != val.clone()
                {
                    let tupleinsert: (String, String, usize, usize) = (
                        seq.id,
                        seq.seq,
                        linevec[3].parse::<usize>().unwrap(),
                        linevec[4].parse::<usize>().unwrap(),
                    );
                    returnvec_2.push(tupleinsert);
                }
            }
        }
        Ok((returnvec_1, returnvec_2))
    }
}
