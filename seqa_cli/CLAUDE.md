# seqa_cli

Command-line tool for querying genomic files. Uses `clap` for argument parsing.

## Commands

- `search <file> <coordinates>` — Genomic range query against BAM, VCF, GFF, BED, GTF, BedGraph, BigWig, or BigBed files. Supports `--with-header` and `--only-header` flags.
- `filter --file <file>` — File filtering (details in implementation).
- `jam --file <file>` — JAM format processing (custom format in `src/jam/`).

## Structure

- `src/main.rs` — CLI entry point. Routes commands to `seqa_core::api` search functions.
- `src/jam/` — JAM format module with its own models and constants.

## Key Patterns

- Detects file type from extension via `seqa_core::utils` and dispatches to the appropriate search function.
- Outputs results in the format determined by `OutputFormat` (JSON, TSV, etc.).
- All genomic parsing is delegated to `seqa_core`.
