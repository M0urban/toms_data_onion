use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::u64;
mod base85;
mod steps;
use base85::base85;
use steps::{step1, step2, step3, step4, step5, step6};

//195693

fn main() {
    let start = get_file("onion.txt");
    let step1_base = base85(start.as_slice());
    dump_to_file(step1_base.as_slice(), "layers/step1.txt");
    let step2_base = step1::run_step(step1_base.as_slice());
    drop(step1_base);
    dump_to_file(step2_base.as_slice(), "layers/step2.txt");
    let step3_base = step2::run_step(step2_base.as_slice());
    drop(step2_base);
    dump_to_file(step3_base.as_slice(), "layers/step3.txt");
    let step4_base = step3::run_step(step3_base.as_slice());
    drop(step3_base);
    dump_to_file(step4_base.as_slice(), "layers/step4.txt");
    let step5_base = step4::run_step(step4_base.as_slice());
    drop(step4_base);
    dump_to_file(step5_base.as_slice(), "layers/step5.txt");
    let step6_base = step5::run_step(step5_base.as_slice());
    drop(step5_base);
    dump_to_file(step6_base.as_slice(), "layers/step6.txt");
    let core = step6::run_step(step6_base.as_slice());
    drop(step6_base);
    dump_to_file(core.as_slice(), "layers/core.txt");
}

fn get_file(path: &str) -> Vec<u8> {
    let f = File::open(path).unwrap();
    let mut vec = Vec::with_capacity(f.metadata().unwrap().len().try_into().unwrap());
    let mut reader = BufReader::new(f);
    reader.read_to_end(&mut vec).unwrap();
    vec
}

fn dump_to_file(data: &[u8], path: &str) {
    let mut f = File::create(path).unwrap();
    f.write_all(data).unwrap();
}
