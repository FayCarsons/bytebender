mod utils;
mod config;

use rand::distributions::{Uniform, Distribution, WeightedIndex};
use rand::prelude::*;
use arbitrary_chunks::*;
use std::fs::{remove_file, File};
use std::io::{Read, Write};
use utils::*;
use crate::config::{CHUNK_SIZE, NUM_CHUNKS, OUTPUT_PATH, WEIGHTS, INPUT_PATH};

fn main() {
    remove_file(OUTPUT_PATH).unwrap_or_default();
    let mut file = File::open(INPUT_PATH).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    println!("File size: {}", buffer.len());
    println!("file beginning: {:?}", buffer.get(0..10));

    let (header, data, footer) = split_file(&buffer);

    let mut rng = rand::thread_rng();
    
    let chunk_range = Uniform::new(128,CHUNK_SIZE);
    let chunk_sizes: Vec<usize> = (0..NUM_CHUNKS).map(|_| rng.sample(chunk_range)).collect();
    let chunks = data.arbitrary_chunks(&chunk_sizes); 

    let mut out_data = Vec::new();

    let choices = [
        write_to_buff,
        shuffle_chunk,
        sorted_chunk,
        repeat,
        replace_with_chunk,
        chunk_to_string,
        add_noise,
        nop,
    ];
    let weights = WEIGHTS.to_slice();
    let dist = WeightedIndex::new(&weights).unwrap();

    chunks.for_each(|chunk| {
        let choice = choices[dist.sample(&mut rng)];
        choice(chunk, &mut out_data);
    });
    
    let out = build_tiff(header, &out_data, footer);

    println!("out size: {}", out.len());
    println!("original size: {}", buffer.len());

    let mut file = File::create(OUTPUT_PATH).unwrap();
    file.write(&out).unwrap();
}