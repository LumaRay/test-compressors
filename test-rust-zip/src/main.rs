// https://crates.io/crates/zip
// 
// 

use std::time::{SystemTime};//, UNIX_EPOCH};

use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::ops::Range;

use zip;

//use zip::write::FileOptions;
use zip::CompressionMethod;

const LOOPS_COUNT: u32 = 1000;

const FILE_NAME: &str = "mondial-3.0.xml";
//const FILE_NAME: &str = "mondial-3.0.json";
//const FILE_NAME: &str = "mondial-3.0-mini.xml";
//const FILE_NAME: &str = "mondial-3.0-mini.json";

const COMPRESSION_METHODS: [(CompressionMethod, Range<i32>); 4] = [
    (CompressionMethod::Stored, 0..1), 
    (CompressionMethod::Zstd, -7..13),
//    (CompressionMethod::Zstd, -7..23),
    (CompressionMethod::Deflated, 0..10),
    (CompressionMethod::Bzip2, 1..6),
//    (CompressionMethod::Bzip2, 1..10),
]; 

fn main() {
    let mut f = File::open("/home/test/test-xml-json/data/".to_owned() + FILE_NAME).unwrap();
    let mut buf_to_pack: Vec<u8> = vec![];
    f.read_to_end(&mut buf_to_pack).unwrap();

    //let mut buf_packed = [0; 1024 * 1024];
    //let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf_packed[..]));
    for method in COMPRESSION_METHODS.iter() {
        for level in method.1.clone() {
            let mut buf_packed: Vec<u8> = vec![];

            let start = SystemTime::now();
	        for _ in 0..LOOPS_COUNT {
                buf_packed = vec![];
                let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf_packed));
                let options = zip::write::FileOptions::default()
                    .compression_method(method.0)
                    .compression_level(Some(level));
                zip.start_file(FILE_NAME, options).unwrap();
                zip.write_all(&buf_to_pack).unwrap();

                zip.finish().unwrap();
                //drop(zip);
            }
            let ratio = 100.0 * buf_packed.len() as f32 / buf_to_pack.len() as f32;
            println!("Compress method {:?} level {} ratio {} : {:?}", method.0, level, ratio, SystemTime::now().duration_since(start).unwrap());

            let mut buf_unpacked: Vec<u8> = vec![];
            let start = SystemTime::now();
	        for _ in 0..LOOPS_COUNT {
                buf_unpacked = vec![0; buf_to_pack.len()];
                //println!("{}", buf_unpacked.len());
                //let mut zip = zip::ZipReader::new(std::io::Cursor::new(&mut buf_unpacked));
                let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&mut buf_packed)).unwrap();
                let mut file = match archive.by_name(FILE_NAME) {
                    Ok(file) => file,
                    Err(..) => {
                        println!("File {FILE_NAME} not found");
                        return;// 2;
                    }
                };
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                //println!("{contents}");return;
                //println!("{}", contents.len());//return;
                buf_unpacked = contents.as_bytes().to_vec();
                /*for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();
                    let outpath = match file.enclosed_name() {
                        Some(path) => path.to_owned(),
                        None => continue,
                    };
                    std::io::copy(&mut file, &mut buf_unpacked).unwrap();
                    //println!("{outpath:?}");
                }*/
                /*let options = zip::read::FileOptions::default();
//                    .compression_method(method.0)
//                    .compression_level(Some(level));
                zip.start_file("mondial-3.0-mini.json", options).unwrap();
                zip.read(&buf_packed).unwrap();

                zip.finish().unwrap();*/
                //drop(zip);
            }
            //let ratio = 100.0 * buf_packed.len() as f32 / buf_to_pack.len() as f32;
            println!("Decompress method {:?} level {} : {:?}", method.0, level, SystemTime::now().duration_since(start).unwrap());
            //println!("{} {}", buf_to_pack.len(), buf_unpacked.len());
            assert_eq!(buf_to_pack, buf_unpacked);
        }
    }
	//println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
    //println!("Hello, world! {}", buf_packed.len());
}
