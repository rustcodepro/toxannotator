mod args;
mod structtox;
mod tox;
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
- a smartcore implementation of the node clustering where you ahve the defined
- nodes and you have the weights defined. It will estimate teh features from the
- provided nodes file and then will use the weights to implement the cluster.
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
    }
}
