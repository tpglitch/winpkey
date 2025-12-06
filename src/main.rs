use winreg::RegKey;
use winreg::enums::*;

fn decode_product_key(digital_product_id: &[u8]) -> Result<String, String> {
    const KEY_CHARS: &[u8] = b"BCDFGHJKMPQRTVWXY2346789";
    const KEY_OFFSET: usize = 52;

    if digital_product_id.len() < KEY_OFFSET + 15 {
        return Err("Invalid DigitalProductId length".to_string());
    }

    // Extract the relevant bytes (52-66)
    let mut key = [0u8; 15];
    key.copy_from_slice(&digital_product_id[KEY_OFFSET..KEY_OFFSET + 15]);

    // Decode the key
    let mut decoded_key = vec![0u8; 25];
    for i in (0..25).rev() {
        let mut cur = 0u32;
        for j in (0..15).rev() {
            cur = cur * 256;
            cur = key[j] as u32 + cur;
            key[j] = (cur / 24) as u8;
            cur = cur % 24;
        }
        decoded_key[i] = KEY_CHARS[cur as usize];
    }

    // Format the key with dashes (XXXXX-XXXXX-XXXXX-XXXXX-XXXXX)
    let mut product_key = String::new();
    for (i, &ch) in decoded_key.iter().enumerate() {
        product_key.push(ch as char);
        if (i + 1) % 5 == 0 && i != 24 {
            product_key.push('-');
        }
    }

    Ok(product_key)
}

fn get_windows_product_key() -> Result<String, Box<dyn std::error::Error>> {
    // Open registry key
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey_with_flags(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        KEY_QUERY_VALUE | KEY_WOW64_64KEY,
    )?;

    // Read DigitalProductId
    let digital_product_id: Vec<u8> = key.get_raw_value("DigitalProductId")?.bytes;

    // Decode and return the product key
    decode_product_key(&digital_product_id).map_err(|e| e.into())
}

fn main() {
    match get_windows_product_key() {
        Ok(key) => {
            println!("Windows Product Key: {}", key);
        }
        Err(e) => {
            eprintln!("Error retrieving product key: {}", e);
        }
    }
}
