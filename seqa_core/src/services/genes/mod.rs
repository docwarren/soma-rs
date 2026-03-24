use crate::sqlite::establish_connection;
use crate::sqlite::genes::{ get_gene_coordinates, get_gene_symbols, GeneError };
use crate::models::gene_coordinate::GeneCoordinate;
use rusqlite::Connection;

// const GENES_DB: &str = "/home/drew/.ghr/genes.db";
const GENE_COORDINATES_DB: &str = "/home/drew/.ghr/gene_coordinates.db";

pub struct GeneCoordinateService {
    pub genes_coordinates_conn: Connection,
}

impl GeneCoordinateService {
    pub fn new() -> Self {
        let genes_coordinates_conn = establish_connection(GENE_COORDINATES_DB).expect("Failed to establish connection to gene coordinates database");
        GeneCoordinateService { genes_coordinates_conn }
    }

    pub fn gene_coordinates(&self, gene: &str) -> Result<GeneCoordinate, GeneError> {
        get_gene_coordinates(&self.genes_coordinates_conn, gene)
    }

    pub fn gene_symbols(&self) -> Result<Vec<String>, GeneError> {
        get_gene_symbols(&self.genes_coordinates_conn)
    }
}
