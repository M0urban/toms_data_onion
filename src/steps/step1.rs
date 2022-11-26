use super::super::base85;

pub fn run_step(payload: &[u8]) -> Vec<u8> {
    let encode = base85(payload);
    step1_extension(encode.as_slice())
}

pub fn step1_extension(payload: &[u8]) -> Vec<u8> {
    payload
        .iter()
        .map(|byte| {
            let ret = byte ^ 0x55;
            ret.rotate_right(1)
        })
        .collect::<Vec<u8>>()
}
