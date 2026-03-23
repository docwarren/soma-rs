/// Tests for BigBed file search functionality.
/// Compares bigbed_search results against tabix_search on equivalent BED data.

const BIGBED_PATH: &str = "file:///media/drew/ExtraSSD/bigbed/mane.bb";
const BED_PATH: &str = "file:///media/drew/ExtraSSD/bigbed/mane.bed.gz";
const BED_INDEX_PATH: &str = "file:///media/drew/ExtraSSD/bigbed/mane.bed.gz.tbi";

#[tokio::test]
async fn bigbed_search_matches_tabix() {
    use soma_core::api::bigbed_search::bigbed_search;
    use soma_core::api::tabix_search::tabix_search;
    use soma_core::api::search_options::SearchOptions;

    // Search BigBed file
    let bb_options = SearchOptions::new()
        .set_file_path(BIGBED_PATH)
        .set_coordinates("chr1:1000000-1300000")
        .set_include_header(false);

    let bb_result = bigbed_search(&bb_options).await.expect("Failed to search BigBed");

    // Search tabix-indexed BED file
    let bed_options = SearchOptions::new()
        .set_file_path(BED_PATH)
        .set_index_path(BED_INDEX_PATH)
        .set_coordinates("chr1:1000000-1300000")
        .set_output_format("bed")
        .set_include_header(false);

    let bed_result = tabix_search(&bed_options).await.expect("Failed to search BED");

    // Results should have the same number of lines
    assert_eq!(
        bb_result.lines.len(),
        bed_result.lines.len(),
        "BigBed returned {} lines, BED returned {} lines",
        bb_result.lines.len(),
        bed_result.lines.len()
    );

    // Each line should match
    for (i, (bb_line, bed_line)) in bb_result.lines.iter().zip(bed_result.lines.iter()).enumerate() {
        assert_eq!(
            bb_line, bed_line,
            "Line {} differs:\nBigBed: {}\nBED:    {}",
            i, bb_line, bed_line
        );
    }
}

#[tokio::test]
async fn bigbed_search_chr1_small_region() {
    use soma_core::api::bigbed_search::bigbed_search;
    use soma_core::api::tabix_search::tabix_search;
    use soma_core::api::search_options::SearchOptions;

    // Search a smaller region
    let bb_options = SearchOptions::new()
        .set_file_path(BIGBED_PATH)
        .set_coordinates("chr1:65000-72000")
        .set_include_header(false);

    let bb_result = bigbed_search(&bb_options).await.expect("Failed to search BigBed");

    let bed_options = SearchOptions::new()
        .set_file_path(BED_PATH)
        .set_index_path(BED_INDEX_PATH)
        .set_coordinates("chr1:65000-72000")
        .set_output_format("bed")
        .set_include_header(false);

    let bed_result = tabix_search(&bed_options).await.expect("Failed to search BED");

    assert_eq!(bb_result.lines.len(), bed_result.lines.len());

    for (bb_line, bed_line) in bb_result.lines.iter().zip(bed_result.lines.iter()) {
        assert_eq!(bb_line, bed_line);
    }
}

#[tokio::test]
async fn bigbed_search_different_chromosome() {
    use soma_core::api::bigbed_search::bigbed_search;
    use soma_core::api::tabix_search::tabix_search;
    use soma_core::api::search_options::SearchOptions;

    // Search chr2
    let bb_options = SearchOptions::new()
        .set_file_path(BIGBED_PATH)
        .set_coordinates("chr2:1000000-2000000")
        .set_include_header(false);

    let bb_result = bigbed_search(&bb_options).await.expect("Failed to search BigBed");

    let bed_options = SearchOptions::new()
        .set_file_path(BED_PATH)
        .set_index_path(BED_INDEX_PATH)
        .set_coordinates("chr2:1000000-2000000")
        .set_output_format("bed")
        .set_include_header(false);

    let bed_result = tabix_search(&bed_options).await.expect("Failed to search BED");

    assert_eq!(bb_result.lines.len(), bed_result.lines.len());

    for (bb_line, bed_line) in bb_result.lines.iter().zip(bed_result.lines.iter()) {
        assert_eq!(bb_line, bed_line);
    }
}

#[tokio::test]
async fn bigbed_returns_correct_coordinates() {
    use soma_core::api::bigbed_search::bigbed_search;
    use soma_core::api::search_options::SearchOptions;

    let options = SearchOptions::new()
        .set_file_path(BIGBED_PATH)
        .set_coordinates("chr1:1000000-1100000")
        .set_include_header(false);

    let result = bigbed_search(&options).await.expect("Failed to search BigBed");

    // Verify all returned records overlap the query region
    for line in &result.lines {
        let fields: Vec<&str> = line.split('\t').collect();
        let begin: u32 = fields[1].parse().expect("Failed to parse begin");
        let end: u32 = fields[2].parse().expect("Failed to parse end");

        // Record should overlap the query region [1000000, 1100000)
        assert!(end > 1000000 && begin < 1100000,
            "Record {}:{}-{} does not overlap query region 1000000-1100000",
            fields[0], begin, end
        );
    }
}
