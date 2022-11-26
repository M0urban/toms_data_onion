use super::super::base85;

pub fn run_step(payload: &[u8]) -> Vec<u8> {
    let encode = base85(payload);
    step3_extension(encode.as_slice())
}

fn step3_extension(payload: &[u8]) -> Vec<u8> {
    //first find the key
    //looking at previous step file it is likely,
    //that the last 32 characters of the first line(len 60) is a '='
    //After testing this assumption is correct for the last 28 bytes
    //The first bytes 28-31 are 'ic ]'
    let mut key = Vec::<u8>::with_capacity(32);
    for (xor, byte) in b"ic ]".iter().zip(payload[28..32].iter()) {
        key.push(byte ^ xor);
    }
    //we assume each of these bytes decoded is '='
    for byte in payload[32..60].iter() {
        //xoring with the assumed value gives us the key-byte
        key.push(byte ^ b'=');
    }
    // we start at byte 28 therefore we have to rotate
    //[28,29,30,31, 0, 1, 2, 3, ..., 27]
    // ^____________| rotate left by 4
    key.rotate_left(4);
    //now that we have key simply decrypt
    payload
        .iter()
        .zip(key.iter().cycle())
        .map(|(byte, key)| byte ^ key)
        .collect()
}
