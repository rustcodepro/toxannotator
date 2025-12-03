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
    /// Only compare protein coding
    ProteinCompare {
        /// path to the first gff file
        gfffile1: String,
        /// path to the second gff file
        gfffile2: String,
        /// threads
        threads: String,
    },
    /// Compare protein coding coordinates and sequences also
    ProteinCompareSeqCommand {
        /// path to the first gff file
        gfffile1: String,
        /// path to the second gff file
        gfffile2: String,
        /// fasta1 file
        fastafile_1: String,
        /// fasta2 file
        fastafile_2: String,
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
    /// plot the specific ids information
    ExtractSeq {
        /// file to the annotation
        annotationfile: String,
        /// idsfile
        idsfile: String,
        /// threads
        threads: String,
    },
}
