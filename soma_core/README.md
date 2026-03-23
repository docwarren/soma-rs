<!-- [![Rust](https://github.com/docwarren/ghr-services/actions/workflows/coverage.yml/badge.svg)](https://github.com/docwarren/ghr-services/actions/workflows/coverage.yml) -->

* GHR Services

A library crate for the following GHR services:
- mongo db for patient and user collections
- S3 for clinical documents
- indexing for reading genomic index files

## Envirnment Variables
### S3
- AWS_ACCESS_KEY_ID
- AWS_SECRET_ACCESS_KEY
- AWS_REGION
- S3_BUCKET

### MongoDB
- MONGO_CONNECTION
- MONGO_DB_NAME
- MONGO_USERNAME
- MONGO_PASSWORD

### Azure
- AZURE_TENANT_ID
- AZURE_CLIENT_ID
- AZURE_CLIENT_SECRET
- AZURE_STORAGE_CONTAINER
- AZURE_STORAGE_ACCOUNT

### Google Cloud
- GOOGLE_STORAGE_ACCOUNT
- GOOGLE_BUCKET

## Testing
Setup the environment variables above
Then:
```bash
cargo test
```
## Mongo DB
Hosted at [MongoDB Atlas](https://cloud.mongodb.com/v2/68090aa08b1883449942c711#/overview)

## Coordinate System

Genomic file formats use different coordinate systems. This crate preserves original coordinates from source files and provides utilities for conversion when needed.

### File Format Coordinate Systems

| Format | System | Description |
|--------|--------|-------------|
| BED | 0-based half-open | `[begin, end)` - begin inclusive, end exclusive |
| BAM | 0-based half-open | `[begin, end)` - begin inclusive, end exclusive |
| BigWig | 0-based half-open | `[begin, end)` - begin inclusive, end exclusive |
| VCF | 1-based closed | Position refers to the nucleotide itself |
| GFF/GTF | 1-based closed | `[begin, end]` - both inclusive |

### Visual Representation
```
Nucleotide      | A | C | G | T | A |
1-based pos       1   2   3   4   5
0-based pos       0   1   2   3   4
0-based interval 0   1   2   3   4   5
```

### Canonical Internal Format

For interval math (overlaps, contains, merge), we use **0-based half-open** as the canonical format:
- Conversion: `[1-based begin, 1-based end]` → `[begin - 1, end)`
- Example: GFF region `[100, 200]` → canonical `[99, 200)`

### Key Terms

- **Position**: A single nucleotide location (e.g., VCF POS field). Point feature.
- **Interval**: A range of nucleotides defined by `[begin, end)`. Range feature.
- **begin/end**: Interval boundaries. We use begin/end consistently (not start/stop).

### Variant Examples (VCF format)

Reference sequence:
```
1-based pos     |  1  |  2  |  3  |  4  |  5  |
Reference       |  A  |  C  |  G  |  T  |  A  |
```

#### Substitution (SNV)
G>C at position 3:
```
Result          |  A  |  C  |  C  |  T  |  A  |
VCF: POS=3, REF=G, ALT=C
```

#### Deletion
Delete G at position 3 (anchor base is C at position 2):
```
Result          |  A  |  C  |     |  T  |  A  |
VCF: POS=2, REF=CG, ALT=C
```

#### Insertion
Insert TAG after G at position 3 (anchor base is G):
```
Result          |  A  |  C  |  G  | T | A | G |  T  |  A  |
                                   ^^^^^^^^^^^
                                   inserted bases
VCF: POS=3, REF=G, ALT=GTAG
```
The anchor base (G) is included in both REF and ALT. The inserted bases (TAG) follow the anchor.

### CoordinateSystem Enum

Each model type implements `coordinate_system()` to identify its native format:

```rust
pub enum CoordinateSystem {
    ZeroBasedHalfOpen,  // BED, BAM, BigWig
    OneBasedClosed,     // VCF, GFF, GTF
}
```

Use `canonical_interval()` on Feature types for consistent interval math:

```rust
let (begin, end) = feature.canonical_interval();  // Always 0-based half-open
```
