use crate::structtox::ToxPath;
use crate::structtox::ToxSeq;
use crate::tox::read_fasta;
use plotpy::Barplot;
use plotpy::Plot;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com

comparative annotation plotters

*/

impl ToxPath {
    pub fn proteinplotter(&self) -> Result<(), Box<dyn Error>> {
        let file1_open = File::open(&self.filepath1).expect("file not present");
        let file2_open = File::open(&self.filepath2).expect("file not present");
        let file1_read = BufReader::new(file1_open);
        let file2_read = BufReader::new(file2_open);
        let mut file1vec: Vec<(String, usize, usize)> = Vec::new();
        let mut file2vec: Vec<(String, usize, usize)> = Vec::new();
        for i in file1_read.lines() {
            let line = i.expect("line not present");
            if line.starts_with("#") {
                continue;
            } else if !line.starts_with("#") {
                let linevec = line.split("\t").collect::<Vec<_>>();
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
                let linevec = line.split("\t").collect::<Vec<_>>();
                let inserttuple: (String, usize, usize) = (
                    linevec[0].to_string(),
                    linevec[3].parse::<usize>().unwrap(),
                    linevec[4].parse::<usize>().unwrap(),
                );
                file2vec.push(inserttuple);
            }
        }
        let mut filecoordinates_1_plot: Vec<(String, usize)> = Vec::new();
        let mut filecoordinates_2_plot: Vec<(String, usize)> = Vec::new();
        for i in file1vec.iter() {
            let valueinsert: (String, usize) = (i.0.clone(), (i.2 - i.1));
            filecoordinates_1_plot.push(valueinsert);
        }
        for i in file2vec.iter() {
            let valueinsert: (String, usize) = (i.0.clone(), (i.2 - i.1));
            filecoordinates_2_plot.push(valueinsert);
        }

        let lengthnames_1 = filecoordinates_1_plot
            .iter()
            .map(|x| x.0.as_str())
            .collect::<Vec<_>>();
        let lengthplot_1 = filecoordinates_1_plot
            .iter()
            .map(|x| x.1)
            .collect::<Vec<_>>();

        let lengthnames_2 = filecoordinates_2_plot
            .iter()
            .map(|x| x.0.as_str())
            .collect::<Vec<_>>();
        let lengthplot_2 = filecoordinates_2_plot
            .iter()
            .map(|x| x.1)
            .collect::<Vec<_>>();

        let mut bar = Barplot::new();
        bar.set_horizontal(true)
            .set_with_text("edge")
            .draw_with_str(&lengthnames_1, &lengthplot_1);

        let mut plot = Plot::new();
        plot.set_inv_y()
            .add(&bar)
            .set_title("genenames")
            .set_label_x("length");

        plot.save("./plot.svg")?;

        let mut bar1 = Barplot::new();
        bar1.set_horizontal(true)
            .set_with_text("edge")
            .draw_with_str(&lengthnames_2, &lengthplot_2);

        let mut plot1 = Plot::new();
        plot1
            .set_inv_y()
            .add(&bar)
            .set_title("genenames")
            .set_label_x("length");

        plot1.save("./plot1.svg")?;

        Ok(())
    }

    pub fn unpackseq(
        &self,
        path1: &str,
        path2: &str,
    ) -> Result<
        (
            Vec<(String, String, usize, usize)>,
            Vec<(String, String, usize, usize)>,
        ),
        Box<dyn Error>,
    > {
        let file1open = File::open(&self.filepath1).expect("file not present");
        let file2open = File::open(&self.filepath2).expect("file not present");
        let file1fasta = read_fasta(path1).unwrap();
        let file2fasta = read_fasta(path2).unwrap();
        let mut returnvec_1: Vec<(String, String, usize, usize)> = Vec::new();
        let mut returnvec_2: Vec<(String, String, usize, usize)> = Vec::new();
        let file1read = BufReader::new(file1open);
        let file2read = BufReader::new(file2open);
        let mut toxseq_1: Vec<ToxSeq> = Vec::new();
        let mut toxseq_2: Vec<ToxSeq> = Vec::new();
        for genomevel in file1read.lines() {
            let linenew = genomevel.expect("line not present");
            let linevec = linenew.split("\t").collect::<Vec<_>>();
            toxseq_1.push(ToxSeq {
                name: linevec[8].split(";").collect::<Vec<_>>()[0]
                    .to_string()
                    .replace("ID=", ""),
                start: linevec[3].parse::<usize>().unwrap(),
                stop: linevec[4].parse::<usize>().unwrap(),
            });
        }
        for genomevel in file2read.lines() {
            let linenew = genomevel.expect("line not present");
            let linevec = linenew.split("\t").collect::<Vec<_>>();
            toxseq_2.push(ToxSeq {
                name: linevec[8].split(";").collect::<Vec<_>>()[0]
                    .to_string()
                    .replace("ID=", ""),
                start: linevec[3].parse::<usize>().unwrap(),
                stop: linevec[4].parse::<usize>().unwrap(),
            });
        }

        for i in toxseq_1.iter() {
            for (val, seq) in file1fasta.iter() {
                if i.name == val.clone() {
                    let value: (String, String, usize, usize) =
                        (seq.id.clone(), seq.seq.clone(), i.start, i.stop);
                    returnvec_1.push(value);
                }
            }
        }
        for i in toxseq_2.iter() {
            for (val, seq) in file2fasta.iter() {
                if i.name == val.clone() {
                    let value: (String, String, usize, usize) =
                        (seq.id.clone(), seq.seq.clone(), i.start, i.stop);
                    returnvec_2.push(value);
                }
            }
        }
        Ok((returnvec_1, returnvec_2))
    }

    pub fn unpackseq_diff(
        &self,
        path1: &str,
        path2: &str,
    ) -> Result<
        (
            Vec<(String, String, usize, usize)>,
            Vec<(String, String, usize, usize)>,
        ),
        Box<dyn Error>,
    > {
        let file1open = File::open(&self.filepath1).expect("file not present");
        let file2open = File::open(&self.filepath2).expect("file not present");
        let file1fasta = read_fasta(path1).unwrap();
        let file2fasta = read_fasta(path2).unwrap();
        let mut returnvec_1: Vec<(String, String, usize, usize)> = Vec::new();
        let mut returnvec_2: Vec<(String, String, usize, usize)> = Vec::new();
        let file1read = BufReader::new(file1open);
        let file2read = BufReader::new(file2open);
        let mut toxseq_1: Vec<ToxSeq> = Vec::new();
        let mut toxseq_2: Vec<ToxSeq> = Vec::new();
        for genomevel in file1read.lines() {
            let linenew = genomevel.expect("line not present");
            let linevec = linenew.split("\t").collect::<Vec<_>>();
            toxseq_1.push(ToxSeq {
                name: linevec[8].split(";").collect::<Vec<_>>()[0]
                    .to_string()
                    .replace("ID=", ""),
                start: linevec[3].parse::<usize>().unwrap(),
                stop: linevec[4].parse::<usize>().unwrap(),
            });
        }
        for genomevel in file2read.lines() {
            let linenew = genomevel.expect("line not present");
            let linevec = linenew.split("\t").collect::<Vec<_>>();
            toxseq_2.push(ToxSeq {
                name: linevec[8].split(";").collect::<Vec<_>>()[0]
                    .to_string()
                    .replace("ID=", ""),
                start: linevec[3].parse::<usize>().unwrap(),
                stop: linevec[4].parse::<usize>().unwrap(),
            });
        }

        for i in toxseq_1.iter() {
            for (val, seq) in file1fasta.iter() {
                if i.name != val.clone() {
                    let value: (String, String, usize, usize) =
                        (seq.id.clone(), seq.seq.clone(), i.start, i.stop);
                    returnvec_1.push(value);
                }
            }
        }
        for i in toxseq_2.iter() {
            for (val, seq) in file2fasta.iter() {
                if i.name != val.clone() {
                    let value: (String, String, usize, usize) =
                        (seq.id.clone(), seq.seq.clone(), i.start, i.stop);
                    returnvec_2.push(value);
                }
            }
        }
        Ok((returnvec_1, returnvec_2))
    }
}
