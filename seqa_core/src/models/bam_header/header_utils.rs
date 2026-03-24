use crate::models::bam_header::header::BamHeaderError;

pub async fn read_magic(bytes: &Vec<u8>) -> Result<(String, u32), BamHeaderError> {
    let magic = String::from_utf8_lossy(&bytes[0..4]).to_string();
    let l_text = u32::from_le_bytes(bytes[4..8].try_into()?);

    Ok((magic, l_text))
}