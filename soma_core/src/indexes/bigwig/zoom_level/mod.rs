use crate::indexes::bigwig::chr_tree::BigwigChrTree;
use crate::indexes::bigwig::r_tree::RTree;
use crate::models::bigwig::zoom_data::ZoomData;

pub struct ZoomLevel {
    pub zoom_count: u32,
    pub zoom_data: Vec<ZoomData>,
    pub zoom_index: RTree,
}

impl ZoomLevel {

    pub fn from_decompressed_bytes(bytes: &[u8], chr_tree: &BigwigChrTree) -> Option<Self> {

        let mut zoom_data = Vec::new();
        let mut offset = 4;

        while offset + ZoomData::SIZE <= bytes.len() {
            if let Ok(data) = ZoomData::from_bytes(&bytes[offset..offset + ZoomData::SIZE], chr_tree) {
                zoom_data.push(data);
            }
            offset += ZoomData::SIZE;
        }

        Some(ZoomLevel {
            zoom_count: zoom_data.len() as u32,
            zoom_data,
            zoom_index: RTree::new(), // Placeholder, should be initialized properly
        })
    }
}
