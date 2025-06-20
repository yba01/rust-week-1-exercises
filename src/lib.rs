// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // todo!()
    let transaction = hex::decode(raw_tx_hex);
    if transaction.is_err() {
        return Err("Hex decode error".to_string());
    }
    if transaction.clone().unwrap().len() < 4 {
        return Err("Transaction data too short".to_string());
    }
    let version_byte: [u8; 4] = (&transaction.unwrap()[0..4]).try_into().unwrap();
    let version = u32::from_le_bytes(version_byte);
    Ok(version)
}
