// https://crates.io/crates/lz4

// mondial-3.0.xml
// Compress/Decompress level 2 ratio 16% :  2238 /  871
// Compress/Decompress level 3 ratio 12% : 10424 /  762
// mondial-3.0-mini.xml
// Compress/Decompress level 2 ratio 21% :  1875 /  605
// Compress/Decompress level 3 ratio 16% :  8816 /  575
// Compress/Decompress level 4 ratio 16% : 10867 /  538
// mondial-3.0.json
// Compress/Decompress level 2 ratio 10% :  3715 / 1734
// Compress/Decompress level 3 ratio  7% : 18857 / 1308
// mondial-3.0-mini.json
// Compress/Decompress level 2 ratio 22% :  1962 /  639
// Compress/Decompress level 3 ratio 17% :  9844 /  580

//extern crate lz4;

//use std::env;
use std::fs::File;
use std::io::{self, Result};
use std::path::{Path};//, PathBuf};

use lz4::{Decoder, EncoderBuilder};

use std::time::{SystemTime};//, UNIX_EPOCH};

//const BUF_SIZE: usize = 16 * 1024 * 1024;

const LOOPS_COUNT: u32 = 1_000;

fn main() {
    //let source = Path::new("/home/test/test-compressors/test-rust-lz4/assets/mondial-3.0.xml");
    //let source = Path::new("/home/test/test-compressors/test-rust-lz4/assets/mondial-3.0-mini.xml");
    //let source = Path::new("/home/test/test-compressors/test-rust-lz4/assets/mondial-3.0.json");
    let source = Path::new("/home/test/test-compressors/test-rust-lz4/assets/mondial-3.0-mini.json");
    let mut input_file = File::open(source).unwrap();
    //let mut input_buf: &[u8; BUF_SIZE] = &[0; BUF_SIZE];
    //let mut output_buf: &[u8; BUF_SIZE] = &[0; BUF_SIZE];
    let mut input_buf: Vec<u8> = vec![];
    let mut output_buf: Vec<u8> = vec![];
    io::copy(&mut input_file, &mut input_buf).unwrap();
    let source_data = input_buf.clone();
    
    let mut _result: Result<()> = Ok(());

    for level in 0..12 {
        let start = SystemTime::now();
        let mut output_buf: Vec<u8> = vec![];
        for _ in 0..LOOPS_COUNT {
            output_buf = vec![];
            let mut encoder = EncoderBuilder::new().level(level).build(output_buf).unwrap();
            io::copy(&mut input_buf.as_slice(), &mut encoder).unwrap();
            (output_buf, _result) = encoder.finish();
        }
        let ratio = 100.0 * output_buf.len() as f32 / source_data.len() as f32;
        //println!("{} {}", source_data.len(), output_buf.len());
        println!("Compress level {} ratio {} : {:?}", level, ratio, SystemTime::now().duration_since(start).unwrap());
        //println!("{:?}, {}", result, output_buf.len());

        let start = SystemTime::now();
        for _ in 0..LOOPS_COUNT {
            let mut input_buf: Vec<u8> = vec![];
            let mut decoder = Decoder::new(output_buf.as_slice()).unwrap();
            io::copy(&mut decoder, &mut input_buf).unwrap();
        }
        println!("Decompress level {} : {:?}", level, SystemTime::now().duration_since(start).unwrap());
        //println!("{}", input_buf.len());
        //println!("{:?}", source_data == input_buf);
        assert_eq!(source_data, input_buf);
    }

}

/*fn main1() {
    println!("LZ4 version: {}", lz4::version());

    for path in env::args().skip(1).map(PathBuf::from) {
        if let Some("lz4") = path.extension().and_then(|e| e.to_str()) {
            decompress(&path, &path.with_extension("")).unwrap();
        } else {
            compress(&path, &path.with_extension("lz4")).unwrap();
        }
    }
}

fn compress(source: &Path, destination: &Path) -> Result<()> {
    println!("Compressing: {} -> {}", source.display(), destination.display());

    let mut input_file = File::open(source)?;
    let output_file = File::create(destination)?;
    let mut encoder = EncoderBuilder::new()
        .level(4)
        .build(output_file)?;
    io::copy(&mut input_file, &mut encoder)?;
    let (_output, result) = encoder.finish();
    result
}

fn decompress(source: &Path, destination: &Path) -> Result<()> {
    println!("Decompressing: {} -> {}", source.display(), destination.display());

    let input_file = File::open(source)?;
    let mut decoder = Decoder::new(input_file)?;
    let mut output_file = File::create(destination)?;
    io::copy(&mut decoder, &mut output_file)?;

    Ok(())
}*/
