mod args;
mod encoder;
mod proteinplotter;
mod structtox;
mod tensor;
mod tox;
use self::structtox::DNAencoder;
use self::structtox::PathFile;
use self::structtox::ToxPath;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::tox::compare_seq;
use crate::tox::compare_seq_annotation;
use crate::tox::toxsummarize;
use crate::tox::un_compare_seq;
use clap::Parser;

/*
Gaurav Sablok
codeprog@icloud.com
*/

fn main() {
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::Toxsummarize {
            filepathinput_strain1,
            filepathinput_strain2,
            overlapwindow,
            readfasta1,
            readfasta2,
        } => {
            let toxsummarizer =
                toxsummarize(filepathinput_strain1, filepathinput_strain2, overlapwindow).unwrap();
            let compareseq_output = compare_seq(readfasta1, readfasta1).unwrap();
            let compareseqanno_output = compare_seq_annotation(readfasta1, readfasta2).unwrap();
            let uncompareseqoutput = un_compare_seq(readfasta1, readfasta2).unwrap();
            println!(
                "The command has finished:{}\t{}\t{}\t{}",
                toxsummarizer, compareseq_output, compareseqanno_output, uncompareseqoutput
            );
        }
        Commands::ProteinPlotter {
            inputfile1,
            inputfile2,
            inputfastafile1,
            inputfastafile2,
        } => {
            let command = ToxPath {
                filepath1: inputfile1.clone(),
                filepath2: inputfile2.clone(),
            };
            let proteinplot = command.proteinplotter();
            let unpacksame = command.unpackseq(inputfastafile1, inputfastafile2).unwrap();
            let unpackdiff = command
                .unpackseq_diff(inputfastafile1, inputfastafile2)
                .unwrap();
            println!(
                "The command has finished:{:?}\t{:?}\t{:?}\t{:?}",
                command, proteinplot, unpacksame, unpackdiff
            );
        }
        Commands::ProteinTensor { inputfile } => {
            let command = PathFile {
                inputpath: inputfile.clone(),
            };
            let _ = command.tensor();
            let _ = command.padded_tensor();
            println!("The padded tensor have been written:{:?}", command);
        }
        Commands::DNAEncoder {
            inputfastafile,
            inputdim,
            bottleneck,
            epochs,
        } => {
            let filepath = DNAencoder {
                pathfile: inputfastafile.to_string(),
            };
            let command =
                Some(filepath.run_encoder(inputfastafile, *inputdim, *bottleneck, *epochs));
            println!("The command has finished:{:?}", command)
        }
    }
}
