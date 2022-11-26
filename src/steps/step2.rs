use super::super::base85;

pub fn run_step(payload: &[u8]) -> Vec<u8> {
    let encode = base85(payload);
    step2_extension(encode.as_slice())
}

fn step2_extension(payload: &[u8]) -> Vec<u8> {
    let good_bytes: Vec<u8> = payload
        .iter()
        .copied()
        .filter(|byte| parity_is_correct(*byte))
        .collect();
    // if good_bytes.len() % 8 != 0 {
    //     panic!("invalid message {}", good_bytes.len());
    // }
    let vec: Vec<u8> = good_bytes
        .chunks(8)
        .flat_map(|chunk| {
            let mut ret = Vec::<u8>::with_capacity(7);
            let mut shifted = 0u64;
            for (index, &byte) in chunk.iter().enumerate() {
                shifted += ((byte >> 1) as u64) << (7 * (7 - index));
            }
            let bytes = shifted.to_be_bytes();
            for &byte in bytes.iter().skip(1) {
                ret.push(byte);
            }
            ret
        })
        .collect();
    vec
}

fn parity_is_correct(mut byte: u8) -> bool {
    let parity_odd = u32::from((byte % 2) == 1);
    let mut odd_bits = 0;
    for _ in 0usize..7 {
        byte >>= 1;
        if byte % 2 == 1 {
            odd_bits += 1
        }
    }
    (odd_bits % 2) == parity_odd
}

#[test]
fn test_correct_parity() {
    assert!(parity_is_correct(0b1011_0010));
    assert!(parity_is_correct(0b0000_0000));
    assert!(parity_is_correct(0b1111_1111));
}
