pub fn base85(payload: &[u8]) -> Vec<u8> {
    let mut ret = trim_delimiter(payload);

    ret.retain(|x| !u8::is_ascii_whitespace(x));

    let stuff = ret
        .chunks(5)
        .flat_map(|chunk| {
            let mut chunk_sum: u64 = 0;
            let slice_len: u32 = chunk.len().try_into().unwrap();
            for (ind, byte) in (0u32..slice_len).rev().zip(chunk.iter()) {
                chunk_sum += ((*byte - 33u8) as u64) * 85u64.pow(ind);
            }
            let mut decoded: Vec<u8> = vec![0; 4];
            (0usize..4).rev().for_each(|num| {
                decoded[num] = (chunk_sum % 256).try_into().unwrap();
                chunk_sum /= 256;
            });
            decoded
        })
        .collect::<Vec<u8>>();
    stuff
}

fn trim_delimiter(payload: &[u8]) -> Vec<u8> {
    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;

    for (index, byte) in payload.windows(2).enumerate() {
        if byte[0] == b'<' && byte[1] == b'~' {
            start = Some(index + 2);
            break;
        }
    }
    let Some(start) = start else {
        panic!("no start found")
    };

    for (index, byte) in payload.windows(2).enumerate().skip(start) {
        if byte[0] == b'~' && byte[1] == b'>' {
            end = Some(index + 2);
            break;
        }
    }
    let Some(end) = end else {
        panic!("no start found")
    };
    payload[start..end].to_vec()
}
