use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "toxannotator",
    version = "1.0",
    about = "A toxodb annotator.
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Summarize the strains across the Toxodb
    Toxsummarize {
        /// inputfile1
        filepathinput_strain1: String,
        /// inputfile2
        filepathinput_strain2: String,
        /// overlapwindow
        overlapwindow: String,
        /// readfasta1
        readfasta1: String,
        /// readfasta2
        readfasta2: String,
        /// threads
        threads: String,
    },
    /// Plot the protein coding regions
    ProteinPlotter {
        /// input file 1
        inputfile1: String,
        /// input file 2
        inputfile2: String,
        /// threads
        threads: String,
    },
    /// Prepare the protein tensor for the machine and deep learning
    ProteinTensor {
        /// input file for the tensor
        inputfile: String,
        /// threads
        threads: String,
    },
    /// extract the sequences from the annotation
    ExtractSeq {
        /// path to the annotation file
        annotationfile: String,
        /// path to the fasta file
        fastafile: String,
        /// threads
        threads: String,
    },
    /// plot the exons or the cds for the regions
    ExtractPlot {
        /// file to the annotation
        annotationfile: String,
        /// idsfile
        idsfile: String,
        /// threads
        threads: String,
    },
    /// Only compare protein coding
    ProteinCompare {
        /// path to the first gff file
        gfffile1: String,
        /// path to the second gff file
        gfffile2: String,
        /// threads
        threads: String,
    },
}
