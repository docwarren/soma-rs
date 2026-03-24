pub const MAX_BLOCK_SIZE: u64 = 64 * 1024;
pub const MAX_BIN_SIZE: usize = (((1<<18) - 1) / 7) as usize;
pub const LINEAR_BIN_SIZE: u32 = 16384;
pub const BIGWIG_HEADER_SIZE: u64 = 64;
pub const BIGWIG_ZOOM_HEADER_SIZE: u64 = 24;
pub const DEFAULT_ZOOM_PIXELS: f32 = 3000.0;