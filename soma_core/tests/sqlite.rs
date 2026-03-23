#![cfg(feature = "sqlite")]

#[test]
fn test_get_gene_coordinates() {
    use soma_core::sqlite::establish_connection;
    use soma_core::sqlite::genes::get_gene_coordinates;

    let conn = establish_connection("/media/drew/ExtraSSD/genes/grch38-genes.db").unwrap();
    let gene = get_gene_coordinates(&conn, "BRCA1").unwrap();
    assert_eq!(gene.gene, "BRCA1");
    assert_eq!(gene.chr, "chr17");
    assert_eq!(gene.begin, 43044295);
    assert_eq!(gene.end, 43170327);
}


#[test]
fn test_get_gene() {
    use soma_core::sqlite::establish_connection;
    use soma_core::sqlite::genes::get_gene_symbols;

    let conn = establish_connection("/media/drew/ExtraSSD/genes/grch38-genes.db").unwrap();
    let genes = get_gene_symbols(&conn).unwrap();
    assert_eq!(genes.len(), 2);
    assert_eq!(genes[0], "BRCA2");
    assert_eq!(genes[1], "BRCA1");
}
