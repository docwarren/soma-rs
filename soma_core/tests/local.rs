#[tokio::test]
async fn obj_store_local() {
    use soma_core::stores::StoreService;

    let local_path = format!("file://{}/tests/data/test.txt", env!("CARGO_MANIFEST_DIR"));
    let store = StoreService::from_uri(&local_path).expect("Failed to create local store");
    let data = store.get_object(&local_path).await.expect("Failed to get data from Local store");
    assert_eq!("Hello world".to_string(), String::from_utf8_lossy(&data));
}

#[tokio::test]
async fn bam_chr12_local() {
    use soma_core::api::bam_search::bam_search;
    use soma_core::api::search_options::SearchOptions;

    let options = SearchOptions::new()
        .set_file_path("file:///media/drew/ExtraSSD/vcfs/NA12877.bam")
        .set_index_path("file:///media/drew/ExtraSSD/vcfs/NA12877.bam.bai")
        .set_coordinates("chr12:10000000-10000000")
        .set_output_format("bam")
        .set_include_header(false);

    let result = bam_search(&options).await.expect(&format!("Failed to search BAM for chr12: {}", options.chromosome));
    assert_eq!(result.lines.len(), 51);
    assert_eq!(format!("{}", result.lines[0]), "HSQ1008:141:D0CC8ACXX:4:2203:18142:64281	83	chr12	9999905	60	101M	=	9999602	-404	ATCAGAGACTAGGTTTGCAACCCCTGCTTTATTTTATTTTATTTTATTTACTTATTTATTTATTTTTGCTTTCCATTTGCTTGGGAAATATTTCTCCATCA	;:5>@>ACA@C?=A;;;<=48=EC=>@>DHF;HGHF<IIIIG>HEFDBB9FBHGHCHF@ECHFEFFBHEAE@BE@DBEFECAIIGGGE>BFC;BDDBD<@@	RG:Z:NA12877	XT:A:U	NM:i:0	SM:i:37	AM:i:37	X0:i:1	X1:i:0	XM:i:0	XO:i:0	XG:i:0	MD:Z:101");
    assert_eq!(format!("{}", result.lines[50]), "HSQ1008:141:D0CC8ACXX:4:1102:6116:159280	99	chr12	9999998	52	101M	=	10000295	398	CTCCATCACTTTATTTTGAGTCTATGTGTGTCTTTGCACATTCAATGGGTCTCCTGAATACAGCACACCAATGGTTCTTGACTCTTTATCCAATTTGCCAG	@CCFFFFFHHHFFGHIIEIGGHHIJJJJIIGGHJJIEGIJIGJEIIJIGGHGIJJJFHJJJJJJGGIIIGHIII=>CHHHGHFFFD>?CCECCCEEDDDC@	RG:Z:NA12877	XT:A:U	NM:i:0	SM:i:37	AM:i:15	X0:i:1	X1:i:0	XM:i:0	XO:i:0	XG:i:0	MD:Z:101");
}

#[tokio::test]
async fn bam_chr12_more_local() {
    use soma_core::api::bam_search::bam_search;
    use soma_core::api::search_options::SearchOptions;

    let options = SearchOptions::new()
        .set_file_path("file:///media/drew/ExtraSSD/vcfs/NA12877.bam")
        .set_index_path("file:///media/drew/ExtraSSD/vcfs/NA12877.bam.bai")
        .set_coordinates("chr12:10000000-10010000")
        .set_output_format("bam")
        .set_include_header(false);

    let result = bam_search(&options).await.expect(&format!("Failed to search BAM for chr12: {}", options.chromosome));
    assert_eq!(result.lines.len(), 5639);
}

#[tokio::test]
async fn bam_chr1_local() {
    use soma_core::api::bam_search::bam_search;
    use soma_core::api::search_options::SearchOptions;

    let options = SearchOptions::new()
        .set_file_path("file:///media/drew/ExtraSSD/vcfs/NA12877.bam")
        .set_index_path("file:///media/drew/ExtraSSD/vcfs/NA12877.bam.bai")
        .set_coordinates("chr1:10000000-10200000")
        .set_output_format("bam")
        .set_include_header(false);

    let result = bam_search(&options).await.expect(&format!("Failed to search BAM for chr1: {}", options.chromosome));
    assert_eq!(result.lines.len(), 108198);
}

#[tokio::test]
async fn bam_chr_m_local() {
    use soma_core::api::bam_search::bam_search;
    use soma_core::api::search_options::SearchOptions;

    let options = SearchOptions::new()
        .set_file_path("file:///media/drew/ExtraSSD/vcfs/NA12877.bam")
        .set_index_path("file:///media/drew/ExtraSSD/vcfs/NA12877.bam.bai")
        .set_coordinates("chrM:1-1")
        .set_output_format("bam")
        .set_include_header(false);

    let result = bam_search(&options).await.expect(&format!("Failed to search BAM for chrM: {}", options.chromosome));
    assert_eq!(result.lines.len(), 120);
    assert_eq!(format!("{}", result.lines[0]), "HSQ1008:141:D0CC8ACXX:3:1308:20201:36071	117	chrM	1	0	*	=	1	0	TGGTTAATAGGGTGATAGACCTGTGATCCATCGTGATGTCTTATTTAAGGGGAACGTGTGGGCTATTTAGGCTTTATGGCCCTGAAGTAGGAACCAGATGT	:C@@C>C<?AA;;;>;;;3?@;>EBBFFDE=HHC@;HHEEIGG:GEBHF@HGGIHF@GDGG?C@DIHEGHHHDG@HGHDIIGEGFAECBHHDDFFBFF@@?	RG:Z:NA12877");
    assert_eq!(format!("{}", result.lines[119]), "HSQ1008:141:D0CC8ACXX:4:2306:19106:26897	1105	chrM	1	37	101M	=	16350	16248	GATCACAGGTCTATCACCCTATTAACCACTCACGGGAGCTCTCCATGCATTTGGTATTTTCGTCTGGGGGGTATGCACGCGATAGCATTGCGAGACGCTGG	AEEC@CCCCCADAA?9BCCDDCA8:8::?<>?CBCCCBCCBCBCECCCEACEEFFFFEA:GHGHGIIGIGIIIEHF:IIIGHHGGG:EDGFHADFFFFCCC	RG:Z:NA12877	XT:A:U	NM:i:1	SM:i:37	AM:i:37	X0:i:1	X1:i:0	XM:i:1	XO:i:0	XG:i:0	MD:Z:72G28");
}