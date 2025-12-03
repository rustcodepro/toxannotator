# toxannotator

- a toxdb annotator for genomic comparison for comparative genomics. 
- ready to use tables for comparative analysis. 
- genomic comparison and annotation plotters. 

```
cargo build
```

```
_____                                                  _             _                  
_____                                                  _             _                  
|_   _|   ___   __  __   __ _   _ __    _ __     ___   | |_    __ _  | |_    ___    _ __ 
 | |    / _ \  \ \/ /  / _` | | '_ \  | '_ \   / _ \  | __|  / _` | | __|  / _ \  | '__|
 | |   | (_) |  >  <  | (_| | | | | | | | | | | (_) | | |_  | (_| | | |_  | (_) | | |   
 |_|    \___/  /_/\_\  \__,_| |_| |_| |_| |_|  \___/   \__|  \__,_|  \__|  \___/  |_|   
                                                                                        

A toxodb annotator.
     ************************************************
     Gaurav Sablok,
     Email: codeprog@icloud.com
    ************************************************

Usage: toxannotator <COMMAND>

Commands:
protein-plotter  Plot the protein coding regions
protein-tensor   Prepare the protein tensor for the machine and deep learning
extract-seq      plot the specific ids information
protein-compare  Only compare protein coding
help             Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version

```

```
Only compare protein coding

Usage: toxannotator protein-compare <GFFFILE1> <GFFFILE2>

Arguments:
  <GFFFILE1>  path to the first gff file
  <GFFFILE2>  path to the second gff file

Options:
  -h, --help  Print help
  
toxannotator protein-compare ./testfiles/a1.gff ./testfiles/b1.gff  4

GeneName	Start1	Start2	End1	End2	Start Difference	End Difference
TGME49_200010	2245476	2245476	2249210	2248187	0	1023

```

```
Plot the protein coding regions

Usage: toxannotator protein-plotter <INPUTFILE1> <INPUTFILE2> <THREADS>

Arguments:
  <INPUTFILE1>  input file 1
  <INPUTFILE2>  input file 2
  <THREADS>     threads

Options:
  -h, --help  Print help
  
toxannotator  extract-plot ./testfiles/a1.gff ./testfiles/id.test 4

```

```
Prepare the protein tensor for the machine and deep learning

Usage: toxannotator protein-tensor <INPUTFILE> <THREADS>

Arguments:
  <INPUTFILE>  input file for the tensor
  <THREADS>    threads

Options:
  -h, --help  Print help

```

```
extract the sequences from the annotation

Usage: toxannotator extract-seq <ANNOTATIONFILE> <FASTAFILE> <THREADS>

Arguments:
  <ANNOTATIONFILE>  path to the annotation file
  <FASTAFILE>       path to the fasta file
  <THREADS>         threads

Options:
  -h, --help  Print help

```
Gaurav Sablok \
codeprog@icloud.com
