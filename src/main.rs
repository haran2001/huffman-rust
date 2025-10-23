use bit_vec::BitVec;
use std::fs;
use std::collections::HashMap;

fn integer_to_binary_vector(mut n: u32) -> BitVec {
    if n == 0 {
        let mut bv = BitVec::new();
        bv.push(false);
        return bv;
    }

    let mut bits = Vec::new();
    while n > 0 {
        bits.push(n % 2 == 1);
        n /= 2;
    }
    bits.reverse();

    let mut binary_vector = BitVec::new();
    for bit in bits {
        binary_vector.push(bit);
    }
    binary_vector
}

pub fn create_encoding_table(tokens: &Vec<char>) -> HashMap<char, BitVec> {
    let mut encoding_table = HashMap::new();
    let mut count = 0;
    for token in tokens {
        encoding_table.insert(*token, integer_to_binary_vector(count));
        count += 1;
    }
    encoding_table
}

pub fn create_decoding_table(tokens: &Vec<char>) -> HashMap<BitVec, char> {
    let mut decoding_table = HashMap::new();
    let mut count = 0;
    for token in tokens {
        decoding_table.insert(integer_to_binary_vector(count), *token);
        count += 1;
    }
    decoding_table
}

pub fn read_data(filename: &str) -> String {
    let data = fs::read_to_string(filename).expect("Should be able to read hosts file");
    data
}

pub fn parse_data(data: &str) -> Vec<char> {
    let mut tokens = Vec::new();
    for char in data.chars() {
        tokens.push(char);
    }
    tokens
}

pub fn compress(token: char, encoding_table: &HashMap<char, BitVec>) -> BitVec {
    encoding_table.get(&token).unwrap().clone()
}

pub fn decompress(token: &BitVec, decoding_table: &HashMap<BitVec, char>) -> char {
    decoding_table.get(token).unwrap().clone()
}

fn main() {
    let data = read_data("data.txt");
    let tokens = parse_data(&data);
    let encoding_table = create_encoding_table(&tokens);
    let decoding_table = create_decoding_table(&tokens);
    let mut compressed_data = BitVec::new();

    for token in tokens {
        let compressed = compress(token, &encoding_table);
        for bit in compressed.iter() {
            compressed_data.push(bit);
        }
    }

    println!("Compressed data: {:?}", compressed_data);
    // Write compressed data to file
    fs::write("compressed.bin", compressed_data.to_bytes()).expect("Unable to write file");
    println!("Compressed data written to compressed.bin");

    // let decompressed_data = BitVec::new();
    // for bit in compressed_data.iter() {
    //     let decompressed = decompress(&bit, &decoding_table);
    //     decompressed_data.push(decompressed);
    // }

    // println!("Decompressed data: {:?}", decompressed_data);
}
