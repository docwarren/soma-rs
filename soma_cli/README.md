# GHR CLI
GHR CLI is a Rust command-line tool for querying genomic files.

## Installation
To install GHR CLI, you can use the following command:

```bash
cargo build -r
```
You might want ot update your .bashrc or .zshrc file to include the binary in your PATH:
```bash
# Add the following line to your .bashrc or .zshrc
# This assumes you are in the root directory of the GHR CLI project
export PATH="$PATH:$(pwd)/target/release"
```

## Usage
After installation, you can use the `ghr-cli` command to query genomic files. Here are
some examples of how to use the tool:
### Querying a Genomic File
```bash
ghr-cli search <path to genomic file> <path to index file> <coordinates>
```
### Examples
```bash
ghr-cli search /path/to/genomic/file.bam /path/to/index/file.bam.bai chr4:12345-13456

ghr-cli search /path/to/genomic/file.vcf.gz /path/to/index/file.vcf.gz.tbi chr4:12345-13456

ghr-cli search /path/to/genomic/file.bw - chr4:12345-13456
ghr-cli search /path/to/genomic/file.bigwig - chr4:12345-13456

ghr-cli search /path/to/genomic/file.gff.gz /path/to/index/file.gff.gz.tbi chr4:12345-13456
ghr-cli search /path/to/genomic/file.gtf.gz /path/to/index/file.gtf.gz.tbi chr4:12345-13456

ghr-cli search /path/to/genomic/file.bed.gz /path/to/index/file.bed.gz.tbi chr4:12345-13456
ghr-cli search /path/to/genomic/file.bedgraph.gz /path/to/index/file.bedgraph.gz.tbi chr4:12345-13456

ghr-cli search /path/to/genomic/file.fa /path/to/index/file.fa.fai chr4:12345-13456
```
