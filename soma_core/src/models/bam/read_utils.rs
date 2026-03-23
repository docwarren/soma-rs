pub fn map_sequence_code(code: u8) -> char {
    match code {
        0 => '=',
        1 => 'A',
        2 => 'C',
        3 => 'M',
        4 => 'G',
        5 => 'R',
        6 => 'S',
        7 => 'V',
        8 => 'T',
        9 => 'W',
        10 => 'Y',
        11 => 'H',
        12 => 'K',
        13 => 'D',
        14 => 'B',
        _ => 'N', // Default case for any unexpected value
    }
}

pub fn get_tag_ua_value(bytes: &[u8], i: usize) -> Result<(String, usize), String> {
    if i < bytes.len() {
        Ok((format!("{}", bytes[i] as char), i + 1))
    } else {
        Err("Invalid index".into())
    }
}

pub fn get_tag_c_value(bytes: &[u8], i: usize) -> Result<(String, usize), String> {
    if i < bytes.len() {
        Ok((format!("{}", i8::from_le_bytes([bytes[i]])), i + 1))
    } else {
        Err("Invalid index".into())
    }
}

pub fn get_tag_uc_value(bytes: &[u8], i: usize) -> Result<(String, usize), String> {
    if i < bytes.len() {
        Ok((format!("{}", bytes[i]), i + 1))
    } else {
        Err("Invalid index".into())
    }
}

pub fn get_tag_s_value(bytes: &[u8], i: usize) -> Result<(String, usize), String> {
    if i + 2 <= bytes.len() {
        let val_str = format!("{}", i16::from_le_bytes(bytes[i..i + 2].try_into().map_err(|e| format!("Invalid i16: {}", e))?));
        Ok((val_str, i + 2))
    } else {
        Err("Invalid index".into())
    }
}

pub fn get_tag_us_value(bytes: &[u8], i: usize) -> Result<(String, usize), String> {
    if i + 2 <= bytes.len() {
        let val_str = format!("{}", u16::from_le_bytes(bytes[i..i + 2].try_into().map_err(|e| format!("Invalid u16: {}", e))?));
        Ok((val_str, i + 2))
    } else {
        Err("Invalid index".into())
    }
}

pub fn get_tag_i_value(bytes: &[u8], i: usize) -> Result<(String, usize), String> {
    if i + 4 <= bytes.len() {
        let val_str = format!("{}", i32::from_le_bytes(bytes[i..i + 4].try_into().map_err(|e| format!("Invalid i32: {}", e))?));
        Ok((val_str, i + 4))
    } else {
        Err("Invalid index".into())
    }
}

pub fn get_tag_ui_value(bytes: &[u8], i: usize) -> Result<(String, usize), String> {
    if i + 4 <= bytes.len() {
        let val_str = format!("{}", u32::from_le_bytes(bytes[i..i + 4].try_into().map_err(|e| format!("Invalid u32: {}", e))?));
        Ok((val_str, i + 4))
    } else {
        Err("Invalid index".into())
    }
}

pub fn get_tag_f_value(bytes: &[u8], i: usize) -> Result<(String, usize), String> {
    if i + 4 <= bytes.len() {
        let val_str = format!("{}", f32::from_le_bytes(bytes[i..i + 4].try_into().map_err(|e| format!("Invalid f32: {}", e))?));
        Ok((val_str, i + 4))
    } else {
        Err("Invalid index".into())
    }
}

pub fn get_tag_z_value(bytes: &[u8], i: usize) -> Result<(String, usize), String> {
    let mut end = i;
    while end < bytes.len() && bytes[end] != 0 {
        end += 1;
    }
    let value = String::from_utf8_lossy(&bytes[i..end]).to_string();
    Ok((value, end + 1)) // +1 to skip the null terminator
}

pub fn get_tag_h_value(bytes: &[u8], i: usize) -> Result<(String, usize), String> {
    let mut end = i;
    while end < bytes.len() && bytes[end] != 0 {
        end += 1;
    }
    let value = String::from_utf8_lossy(&bytes[i..end]).to_string();
    Ok((value, end + 1)) // +1 to skip the null terminator
}

pub fn map_tag_type_to_result(bytes: &[u8], i: usize, value_type: u8) -> Result<(String, usize), String> {
    match value_type {
        b'A' => get_tag_ua_value(bytes, i),
        b'c' => get_tag_c_value(bytes, i),
        b'C' => get_tag_uc_value(bytes, i),
        b's' => get_tag_s_value(bytes, i),
        b'S' => get_tag_us_value(bytes, i),
        b'i' => get_tag_i_value(bytes, i),
        b'I' => get_tag_ui_value(bytes, i),
        b'f' => get_tag_f_value(bytes, i),
        b'Z' => get_tag_z_value(bytes, i),
        b'H' => get_tag_h_value(bytes, i),
        _ => Ok((String::new(), i)), // Default case for unsupported types
    }
}

pub fn get_tag_value(bytes: &[u8], mut i: usize) -> Result<(String, usize), String> {
    let tag: String = bytes[i..i + 2].iter().map(|&b| b as char).collect();
    i += 2;

    let value_type = bytes[i];
    i += 1;


    let (result, i) = match value_type {
        b'B' => {
            let val_type = bytes[i];
            i += 1;

            let count = i32::from_le_bytes(bytes[i..i + 4].try_into().map_err(|e| format!("Invalid count: {}", e))?);
            i += 4;

            let mut values = Vec::new();
            for _ in 0..count {
                let (value, new_i) = map_tag_type_to_result(bytes, i, val_type)?;
                values.push(value);
                i = new_i;
            }
            (values.join(","), i)
        }
        _ => map_tag_type_to_result(bytes, i, value_type)?,
    };

    let type_char = if "cCsSiIf".contains(value_type as char) {
        'i'
    } else {
        value_type as char
    };

    let formatted_result = format!("{}:{}:{}", tag, type_char, result);
    Ok((formatted_result, i))
}