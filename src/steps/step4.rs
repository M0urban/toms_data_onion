use super::super::base85;
use std::net::Ipv4Addr;

pub fn run_step(payload: &[u8]) -> Vec<u8> {
    let encode = base85(payload);
    step4_extension(encode.as_slice())
}

fn step4_extension(payload: &[u8]) -> Vec<u8> {
    let mut ret = Vec::new();
    let mut packet_start: usize = 0;
    let must_source = Ipv4Addr::new(10, 1, 1, 10);
    let must_dest = Ipv4Addr::new(10, 1, 1, 200);
    let dest_port: u16 = 42069;
    while payload.len() - packet_start > 28 {
        let len = get_total_length_ip(&payload[packet_start..]);
        let (source, dest) = get_source_dest_ip(&payload[packet_start..]);
        let (_, d_port) = get_ports_udp(&payload[packet_start..]);
        let ip_correct = checksum_correct_ip_header(&payload[packet_start..]);
        let udp_correct = checksum_correct_udp_header(
            &payload[packet_start..packet_start + len],
            must_source,
            must_dest,
            0x11,
        );
        if ip_correct
            && udp_correct
            && must_source == source
            && must_dest == dest
            && d_port == dest_port
        {
            ret.extend_from_slice(&payload[packet_start + 28..packet_start + len]);
        }
        packet_start += len;
    }

    ret
}

fn get_total_length_ip(payload: &[u8]) -> usize {
    u16::from_be_bytes(payload[2..4].try_into().unwrap())
        .try_into()
        .unwrap()
}

fn get_source_dest_ip(payload: &[u8]) -> (Ipv4Addr, Ipv4Addr) {
    let src_addr: [u8; 4] = payload[12..16].try_into().unwrap();
    let dest_addr: [u8; 4] = payload[16..20].try_into().unwrap();
    (Ipv4Addr::from(src_addr), Ipv4Addr::from(dest_addr))
}

fn checksum_correct_ip_header(payload: &[u8]) -> bool {
    let mut checksum: u16 = 0;
    //pre checksum field
    ones_complement_sum(&mut checksum, &payload[0..10]);

    //post checksum field
    ones_complement_sum(&mut checksum, &payload[12..20]);

    //ones complement
    !checksum == u16::from_be_bytes(payload[10..12].try_into().unwrap())
}

fn get_ports_udp(payload: &[u8]) -> (u16, u16) {
    (
        u16::from_be_bytes(payload[20..22].try_into().unwrap()),
        u16::from_be_bytes(payload[22..24].try_into().unwrap()),
    )
}

fn checksum_correct_udp_header(
    payload: &[u8],
    src_ip: Ipv4Addr,
    dest_ip: Ipv4Addr,
    protocol_byte: u8,
) -> bool {
    let mut checksum_check: u16 = 0;
    let checksum_header = u16::from_be_bytes(payload[26..28].try_into().unwrap());
    if checksum_header == 0xFFFF {
        return false;
    }
    let ports = get_ports_udp(payload);

    let mut header = Vec::<u8>::with_capacity(20);
    header.extend_from_slice(src_ip.octets().as_slice());
    header.extend_from_slice(dest_ip.octets().as_slice());
    header.push(0x0);
    header.push(protocol_byte);
    header.extend(&payload[24..26]);
    header.extend(ports.0.to_be_bytes());
    header.extend(ports.1.to_be_bytes());
    header.extend(&payload[24..26]);
    ones_complement_sum(&mut checksum_check, header.as_slice());

    ones_complement_sum(&mut checksum_check, &payload[28..]);

    !checksum_check == checksum_header
}

fn ones_complement_sum(sum: &mut u16, payload: &[u8]) {
    let iter = payload.chunks_exact(2);
    let remainder = iter.remainder();
    for word in iter {
        let (new, overflow) = sum.overflowing_add(u16::from_be_bytes(word.try_into().unwrap()));
        *sum = new;
        if overflow {
            *sum += 1
        }
    }
    if remainder.len() == 1 {
        let final_word = [remainder[0], 0];
        let (new, overflow) = sum.overflowing_add(u16::from_be_bytes(final_word));
        *sum = new;
        if overflow {
            *sum += 1
        }
    }
}
