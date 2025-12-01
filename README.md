# toxannotator

- a toxdb annotator for genomic comparison for comparative genomics. 
- async and parallel threaded.
- It has a integrated spanning window algorithm that you can define the window size and it will show the gff compare with in the spanning window. 
- Added today plotting functions and tensor plotters. 
- It gives you ready to use tensors for machine and deep learning for proteins.

```
cargo build
```

```
A toxodb annotator.
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************

Usage: toxannotator <COMMAND>

Commands:
  toxsummarize     Summarize the strains across the Toxodb
  protein-plotter  Plot the protein coding regions
  protein-tensor   Prepare the protein tensor for the machine and deep learning
  extract-seq      extract the sequences from the annotation
  extract-plot     plot the exons or the cds for the regions
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version


```

```
Summarize the strains across the Toxodb

Usage: toxannotator toxsummarize <FILEPATHINPUT_STRAIN1> <FILEPATHINPUT_STRAIN2> <OVERLAPWINDOW> <READFASTA1> <READFASTA2>

Arguments:
  <FILEPATHINPUT_STRAIN1>  inputfile1
  <FILEPATHINPUT_STRAIN2>  inputfile2
  <OVERLAPWINDOW>          overlapwindow
  <READFASTA1>             readfasta1
  <READFASTA2>             readfasta2

Options:
  -h, --help  Print help

```

```
Plot the protein coding regions

Usage: toxannotator protein-plotter <INPUTFILE1> <INPUTFILE2> <INPUTFASTAFILE1> <INPUTFASTAFILE2>

Arguments:
  <INPUTFILE1>       input file 1
  <INPUTFILE2>       input file 2
  <INPUTFASTAFILE1>  input fasta file 1
  <INPUTFASTAFILE2>  input fasta file 2

Options:
  -h, --help  Print help
```

```
Prepare the protein tensor for the machine and deep learning

Usage: toxannotator protein-tensor <INPUTFILE>

Arguments:
  <INPUTFILE>  input file for the tensor

Options:
  -h, --help  Print help

```

```
extract the sequences from the annotation

Usage: toxannotator extract-seq <ANNOTATIONFILE> <FASTAFILE>

Arguments:
  <ANNOTATIONFILE>  path to the annotation file
  <FASTAFILE>       path to the fasta file

Options:
  -h, --help  Print help
```

```
plot the exons or the cds for the regions

Usage: toxannotator extract-plot <ANNOTATIONFILE> <FASTAFILE> <IDSFILE>

Arguments:
  <ANNOTATIONFILE>  file to the annotation
  <FASTAFILE>       fasta file for the genome
  <IDSFILE>         idsfile

Options:
  -h, --help  Print help
```
Gaurav Sablok \
codeprog@icloud.com
