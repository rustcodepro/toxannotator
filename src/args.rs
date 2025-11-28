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
    },
}
