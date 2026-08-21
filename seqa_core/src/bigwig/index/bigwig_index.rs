use crate::api::search_options::SearchOptions;
use crate::bigwig::index::bigwig_index_error::BigwigIndexError;
use crate::bigwig::index::chr_tree::BigwigChrTree;
use crate::bigwig::index::header;
use crate::bigwig::index::total_summary::TotalSummary;
use crate::bigwig::index::util::{get_bigwig_detail_bytes, get_bigwig_header, get_data_count, get_total_summary, get_zoom_headers};
use crate::bigwig::index::zoom_header::ZoomHeader;
use crate::indexes::constants::DEFAULT_ZOOM_PIXELS;
use crate::stores::StoreService;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BigwigIndex {
    pub header: header::BigwigHeader,
    pub zoom_headers: Vec<ZoomHeader>,
    pub total_summary: TotalSummary,
    pub chromosome_tree: BigwigChrTree,
    pub data_count: u32,
    pub file_path: String,
}

impl BigwigIndex {
    pub async fn new(
        store: &StoreService,
        file_path: &str,
    ) -> Result<BigwigIndex, BigwigIndexError> {
        let bigwig_header =  get_bigwig_header(store, file_path).await?;
        let detail_bytes =  get_bigwig_detail_bytes(store, &bigwig_header, file_path).await?;
        let zoom_headers =  get_zoom_headers(&bigwig_header, &detail_bytes)?;
        let total_summary =  get_total_summary(&bigwig_header, &detail_bytes)?;
        let chromosome_tree =  BigwigChrTree::from_bytes(&detail_bytes, &bigwig_header)?;
        let data_count =  get_data_count(&detail_bytes)?;


        Ok(BigwigIndex {
            header: bigwig_header,
            zoom_headers,
            total_summary,
            chromosome_tree,
            data_count,
            file_path: file_path.to_string(),
        })
    }

    pub fn get_zoom_header(&self, options: &SearchOptions) -> Option<&ZoomHeader> {
        let reduction_level = (options.end as f32 - options.begin as f32) / DEFAULT_ZOOM_PIXELS;
        self.zoom_headers.iter().find(|zoom| zoom.matches(reduction_level))
    }

    pub async fn get_end_for_zoom_header(
        &self,
        store: &StoreService,
        zoom_header: &ZoomHeader,
        path_str: &str,
    ) -> Result<u64, BigwigIndexError> {
        match self.get_next_zoom_header(zoom_header) {
            Some(next) => Ok(next.index_offset as u64),
            None => Ok(store.get_file_size(path_str).await?),
        }
    }

    pub fn get_next_zoom_header(&self, zoom_header: &ZoomHeader) -> Option<ZoomHeader> {
        let mut zoom_headers = self.zoom_headers.clone();
        zoom_headers.sort_by_key(|z| z.index_offset);
        let index = zoom_headers.iter().position(|z| z.index_offset == zoom_header.index_offset);
        match index {
            Some(i) => zoom_headers.get(i + 1).cloned(),
            _ => None,
        }
    }

    pub async fn get_full_index_end(
        &self,
        store: &StoreService,
        path_str: &str,
    ) -> Result<u64, BigwigIndexError> {
        match self.zoom_headers.first() {
            Some(zoom) => Ok(zoom.index_offset as u64),
            None => Ok(store.get_file_size(path_str).await?),
        }
    }

    pub async fn get_data(
        &self,
        store: &StoreService,
        range: &Range<u64>,
        path_str: &str,
    ) -> Result<Vec<u8>, BigwigIndexError> {
        Ok(store.get_range(path_str, range.clone()).await?)
    }
}
