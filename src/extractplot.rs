use crate::structtox::Extractplot;
use crate::structtox::SeqExtract;
use crate::structtox::SeqInfo;
use crate::tox::read_fasta;
use plotpy::Barplot;
use plotpy::Plot;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

impl Extractplot {
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
        let fileids = File::open(self.pathfile3.clone()).expect("file not present");
        let fileids_read = BufReader::new(fileids);
        let mut vecids: Vec<String> = Vec::new();
        for i in fileids_read.lines() {
            let line = i.expect("line not present");
            vecids.push(line);
        }
        let mut selectedexons: Vec<(String, Vec<usize>)> = Vec::new();
        for i in vecids.iter() {
            for val in seqextract_class.iter() {
                let mut valuexon: Vec<usize> = Vec::new();
                if *i == val.name {
                    for exoniter in val.exon.iter() {
                        valuexon.push(exoniter.1 - exoniter.0);
                    }
                }
                let valueexonpair: (String, Vec<usize>) = (i.clone(), valuexon);
                selectedexons.push(valueexonpair);
            }
        }
        let mut selectedcds: Vec<(String, Vec<usize>)> = Vec::new();
        for i in vecids.iter() {
            for val in seqextract_class.iter() {
                let mut valuecds: Vec<usize> = Vec::new();
                if *i == val.name {
                    for cdsiter in val.cds.iter() {
                        valuecds.push(cdsiter.1 - cdsiter.0);
                    }
                }
                let valuecdspair: (String, Vec<usize>) = (i.clone(), valuecds);
                selectedcds.push(valuecdspair);
            }
        }

        let exonlengthnames_1 = selectedexons
            .iter()
            .map(|x| x.0.as_str())
            .collect::<Vec<_>>();
        let exonlengthplot_1 = selectedexons
            .iter()
            .map(|x| x.1.clone())
            .flatten()
            .collect::<Vec<_>>();

        let cdslengthnames_1 = selectedcds.iter().map(|x| x.0.as_str()).collect::<Vec<_>>();
        let cdslengthplot_1 = selectedcds
            .iter()
            .map(|x| x.1.clone())
            .flatten()
            .collect::<Vec<_>>();

        let mut bar = Barplot::new();
        bar.set_horizontal(true)
            .set_with_text("edge")
            .draw_with_str(&exonlengthnames_1, &exonlengthplot_1);

        let mut plot = Plot::new();
        plot.set_inv_y()
            .add(&bar)
            .set_title("genenames")
            .set_label_x("length");

        plot.save("./plotexon.svg")?;

        let mut bar1 = Barplot::new();
        bar1.set_horizontal(true)
            .set_with_text("edge")
            .draw_with_str(&cdslengthnames_1, &cdslengthplot_1);

        let mut plot1 = Plot::new();
        plot1
            .set_inv_y()
            .add(&bar)
            .set_title("genenames")
            .set_label_x("length");

        plot1.save("./plotcds.svg")?;

        Ok("The file has been written".to_string())
    }
}
