use crate::config::*;
use rand::prelude::*;
use rand::seq::SliceRandom;

pub struct Weights {
    pub normal: u16,
    pub shuffle: u16,
    pub sorted: u16,
    pub repeat: u16,
    pub replace: u16,
    pub to_string: u16,
    pub noise: u16,
    pub nop: u16,
}

impl Weights {
    pub fn to_slice(&self) -> [u16; 8] {
        [
            self.normal,
            self.shuffle,
            self.sorted,
            self.repeat,
            self.replace,
            self.to_string,
            self.noise,
            self.nop,
        ]
    }
}

pub fn split_file(buffer: &[u8]) -> (&[u8], &[u8], &[u8]) {
    let header = &buffer[0..HEADER_LEN as usize];
    let data = &buffer[HEADER_LEN as usize..buffer.len() - FOOTER_LEN as usize];
    let footer = &buffer[buffer.len() - FOOTER_LEN as usize..];

    (header, data, footer)
}

pub fn build_tiff(header: &[u8], data: &[u8], footer: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(header);
    out.extend_from_slice(data);
    out.extend_from_slice(footer);
    out
}

pub fn write_to_buff(chunk: &[u8], out: &mut Vec<u8>) {
    out.extend_from_slice(chunk);
}

pub fn repeat(chunk: &[u8], out: &mut Vec<u8>) {
    let mut rng = thread_rng();
    for _ in 0..rng.gen_range(0..8) {
        out.extend_from_slice(chunk)
    }
}

pub fn add_noise(chunk: &[u8], out: &mut Vec<u8>) {
    for byte in chunk {
        if random() {
            out.push(random::<u8>())
        } else {
            out.push(*byte)
        }
    }
}

pub fn chunk_to_string(chunk: &[u8], out: &mut Vec<u8>) {
    let mut rng = thread_rng();
    for byte in chunk {
        out.extend_from_slice(PHRASES.choose(&mut rng).unwrap().as_bytes());
    }
}

pub fn luma(v: Vec<u8>) -> f64 {
    if v.len() != 3 {
        return random::<f64>();
    }

    (v[0] as f64 + v[1] as f64 + v[2] as f64) / 3.0
}

pub fn sorted_chunk(chunk: &[u8], out: &mut Vec<u8>) {
    let mut sorted: Vec<u8> = chunk.to_vec();
    sorted.sort();
    out.append(&mut sorted);
}

pub fn shuffle_chunk(chunk: &[u8], out: &mut Vec<u8>) {
    let mut shuffled = chunk.to_vec();
    shuffled.shuffle(&mut thread_rng());
    out.extend_from_slice(&shuffled);
}

pub fn replace_with_chunk(chunk: &[u8], out: &mut Vec<u8>) {
    
    let mut rng = thread_rng();
    let start = rng.gen_range(0..out.len() - chunk.len());
    let end = start + chunk.len();
    if chunk.len() == 0 || out.len() == 0 || end > out.len() {return}
    out[start..end].copy_from_slice(chunk);
}

pub fn nop(_: &[u8], _: &mut Vec<u8>) {}
