pub fn unmask_payload(payload: &[u8], mask: &[u8; 4]) -> Vec<u8> {
    let mut unmasked_payload: Vec<u8> = Vec::with_capacity(payload.len());
    for (i, &byte) in payload.iter().enumerate() {
        unmasked_payload.push(byte ^ mask[i % 4]);
    }
    unmasked_payload
}
