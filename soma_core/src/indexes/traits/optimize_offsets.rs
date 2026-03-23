use std::collections::{ HashMap, HashSet };

use crate::api::constants::MIN_SEARCH_BLOCK_BYTES;

pub trait OptimiseOffsets {
    fn get_offset_map(&mut self) -> &mut HashMap<String,HashSet<u64>>;
    
    /// Optimise interval offsets by merging offsets that are close together.
    /// This can help reduce the number of intervals and make the index more efficient.
    fn optimise_offsets(&mut self) -> Result<(), String> {
        for (_, offsets) in self.get_offset_map().iter_mut() {
            if offsets.is_empty() {
                continue;
            }

            let mut offset_vec = offsets.clone().into_iter().collect::<Vec<_>>();
            offset_vec.sort();

            let mut merged_offsets = HashSet::new();
            let mut last_offset = *offset_vec.first().ok_or_else(|| format!("Error optimising offsets"))?;

            for &offset in offset_vec.iter() {
                if offset - last_offset <= MIN_SEARCH_BLOCK_BYTES {
                    merged_offsets.insert(last_offset);
                } else {
                    merged_offsets.insert(offset);
                    last_offset = offset;
                }
            }
            *offsets = merged_offsets;
        }
        Ok(())
    }
}
