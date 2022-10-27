// https://crates.io/crates/zstd

// mondial-3.0.xml
// Compress/Decompress level 1 ratio 10% :  3236 / 1227
// Compress/Decompress level 8 ratio  8% : 24666 /  965
// mondial-3.0-mini.xml
// Compress/Decompress level 1 ratio 14% :  2718 /  994
// Compress/Decompress level 8 ratio 11% : 20739 /  825
// mondial-3.0.json
// Compress/Decompress level 1 ratio  6% :  4597 / 1777
// Compress/Decompress level 8 ratio  4% : 37382 / 1348
// mondial-3.0-mini.json
// Compress/Decompress level 1 ratio 14% :  2725 / 1022
// Compress/Decompress level 8 ratio 11% : 21308 /  834

// mondial-3.0-mini.xml.lz4
// Compress/Decompress level 2 ratio 89% :   267 /  187
// Compress/Decompress level 8 ratio 88% :  4450 /  197
// mondial-3.0-mini.json.lz4
// Compress/Decompress level 0 ratio 89% :   296 /  189
// Compress/Decompress level 8 ratio 88% :  5326 /  201

use std::io;
use std::fs::File;
use std::path::{Path};//, PathBuf};
use std::time::{SystemTime};//, UNIX_EPOCH};

use zstd;

//const BUF_SIZE: usize = 16 * 1024 * 1024;

const LOOPS_COUNT: u32 = 1_000;

fn main() {
    let source = Path::new("/home/test/test-compressors/test-rust-zstd/assets/mondial-3.0.xml");
    //let source = Path::new("/home/test/test-compressors/test-rust-zstd/assets/mondial-3.0-mini.xml");
    //let source = Path::new("/home/test/test-compressors/test-rust-zstd/assets/mondial-3.0.json");
    //let source = Path::new("/home/test/test-compressors/test-rust-zstd/assets/mondial-3.0-mini.json");
    //let source = Path::new("/home/test/test-compressors/test-rust-lz4/assets/mondial-3.0-mini.xml.lz4");
    //let source = Path::new("/home/test/test-compressors/test-rust-lz4/assets/mondial-3.0-mini.json.lz4");
    let mut input_file = File::open(source).unwrap();
    //let mut input_buf: &[u8; BUF_SIZE] = &[0; BUF_SIZE];
    //let mut output_buf: &[u8; BUF_SIZE] = &[0; BUF_SIZE];
    let mut input_buf: Vec<u8> = vec![];
    let mut output_buf: Vec<u8> = vec![];
    io::copy(&mut input_file, &mut input_buf).unwrap();
    let source_data = input_buf.clone();
    
    //let mut _result: Result<()> = Ok(());

    for level in 0..25 {
        let start = SystemTime::now();
        let mut output_buf: Vec<u8> = vec![];
        for _ in 0..LOOPS_COUNT {
            output_buf = vec![];
            //output_buf = zstd::stream::copy_encode(input_buf.as_slice(), output_buf, level).unwrap();
            let mut encoder = zstd::stream::Encoder::new(output_buf, level).unwrap();
            io::copy(&mut input_buf.as_slice(), &mut encoder).unwrap();
            output_buf = encoder.finish().unwrap();
        }
        let ratio = 100.0 * output_buf.len() as f32 / source_data.len() as f32;
        //println!("{} {}", source_data.len(), output_buf.len());
        println!("Compress level {} ratio {} : {:?}", level, ratio, SystemTime::now().duration_since(start).unwrap());
        //println!("{:?}, {}", result, output_buf.len());

        let start = SystemTime::now();
        for _ in 0..LOOPS_COUNT {
            let mut input_buf: Vec<u8> = vec![];
            zstd::stream::copy_decode(output_buf.as_slice(), input_buf).unwrap();
        }
        println!("Decompress level {} : {:?}", level, SystemTime::now().duration_since(start).unwrap());
        //println!("{}", input_buf.len());
        //println!("{:?}", source_data == input_buf);
        assert_eq!(source_data, input_buf);
    }

}


// This function use the convenient `copy_encode` method
/*fn compress(level: i32) {
    zstd::stream::copy_encode(io::stdin(), io::stdout(), level).unwrap();
}

// This function does the same thing, directly using an `Encoder`:
fn compress_manually(level: i32) {
    let mut encoder = zstd::stream::Encoder::new(io::stdout(), level).unwrap();
    io::copy(&mut io::stdin(), &mut encoder).unwrap();
    encoder.finish().unwrap();
}

fn decompress() {
    zstd::stream::copy_decode(io::stdin(), io::stdout()).unwrap();
}*/

