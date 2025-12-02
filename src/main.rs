mod args;
mod extractplot;
mod proteincompare;
mod proteinplotter;
mod seqseq;
mod structtox;
mod tensor;
mod tox;
use self::structtox::Extractplot;
use self::structtox::PathFile;
use self::structtox::ProteinCompareExtract;
use self::structtox::SeqStruct;
use self::structtox::ToxPath;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::tox::compare_seq;
use crate::tox::compare_seq_annotation;
use crate::tox::toxsummarize;
use crate::tox::un_compare_seq;
use clap::Parser;
use figlet_rs::FIGfont;

/*
Gaurav Sablok
codeprog@icloud.com
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Toxannotator");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());

    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::Toxsummarize {
            filepathinput_strain1,
            filepathinput_strain2,
            overlapwindow,
            readfasta1,
            readfasta2,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let toxsummarizer =
                    toxsummarize(filepathinput_strain1, filepathinput_strain2, overlapwindow)
                        .unwrap();
                let compareseq_output = compare_seq(readfasta1, readfasta1).unwrap();
                let compareseqanno_output = compare_seq_annotation(readfasta1, readfasta2).unwrap();
                let uncompareseqoutput = un_compare_seq(readfasta1, readfasta2).unwrap();
                println!(
                    "The command has finished:{}\t{}\t{}\t{}",
                    toxsummarizer, compareseq_output, compareseqanno_output, uncompareseqoutput
                );
            });
        }
        Commands::ProteinPlotter {
            inputfile1,
            inputfile2,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = ToxPath {
                    filepath1: inputfile1.clone(),
                    filepath2: inputfile2.clone(),
                };
                let proteinplot = command.proteinplotter().unwrap();
                println!("The command has finished:{:?}", proteinplot);
            });
        }
        Commands::ProteinTensor { inputfile, threads } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = PathFile {
                    inputpath: inputfile.clone(),
                };
                let _ = command.tensor();
                let _ = command.padded_tensor();
                println!("The padded tensor have been written:{:?}", command);
            });
        }
        Commands::ExtractSeq {
            annotationfile,
            fastafile,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let filepathread = SeqStruct {
                    pathfile1: annotationfile.clone(),
                    pathfile2: fastafile.clone(),
                };
                let command = filepathread.extractspecific().unwrap();
                println!("The file annotation write has been completed:{:?}", command);
            });
        }
        Commands::ExtractPlot {
            annotationfile,
            idsfile,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let filepathread = Extractplot {
                    pathfile1: annotationfile.clone(),
                    pathfile2: idsfile.clone(),
                };
                let command = filepathread.extractseq(&filepathread.pathfile2).unwrap();
                println!("The command has finished:{:?}", command);
            });
        }
        Commands::ProteinCompare {
            gfffile1,
            gfffile2,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let filepath = ProteinCompareExtract {
                    pathfile1: gfffile1.clone(),
                    pathfile2: gfffile2.clone(),
                };
                let filepathrun = filepath.proteincompare().unwrap();
                println!(
                    "The file for the comparison for the proteome has been written: {:?}",
                    filepathrun
                );
            });
        }
    }
}
