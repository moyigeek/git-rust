use crate::algorithm::sha1;
use flate2::read::{ZlibDecoder, ZlibEncoder};
use std::env;
use std::fs;
use std::io::Read;

mod algorithm;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            println!("init");
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        "cat-file" => {
            if args.len() == 4 && args[2] == "-p" {
                let file_path = format!(".git/objects/{}/{}", &args[3][0..2], &args[3][2..]);
                eprintln!("Reading file: {}", file_path);
                let file_content = fs::read(file_path).unwrap();
                let mut decoder = ZlibDecoder::new(&file_content[..]);
                let mut file_content = String::new();
                decoder.read_to_string(&mut file_content).unwrap();
                // 只要\0 后面的内容
                let file_content = file_content.splitn(2, '\0').nth(1).unwrap();
                print!("{}", file_content);
            } else {
                println!("Usage: cat-file <object>");
            }
        }
        "hash-object" => {
            if args.len() == 3 {
                let file_path = &args[2];
                let content = read_file(file_path);
                let mut sha1_encoder = sha1::Sha1::new();
                println!("{}", sha1_encoder.hash(content.as_bytes()));
            } else if args.len() == 4 && args[2] == "-w" {
                let file_path = &args[3];
                let content = read_file(file_path);
                save_blob(file_path, &content);
            }
        }
        _ => println!("unknown command: {}", args[1]),
    }
}

fn read_file(file_path: &str) -> String {
    let file_content = fs::read_to_string(file_path).unwrap();
    file_content
}

fn write_file(file_path: &str, content: &str) {
    fs::write(file_path, content).unwrap();
}

fn save_blob(file_path: &str, content: &str) {
    let file_name = file_path.split("/").last().unwrap();
    let file_name_hash = algorithm::sha1::Sha1::new().hash(file_name.as_bytes());
    let file_name_hash_dir = format!(".git/objects/{}/", &file_name_hash[0..2]);
    let file_name_hash_file = format!(
        ".git/objects/{}/{}",
        &file_name_hash[0..2],
        &file_name_hash[2..]
    );
    let content = format!("blob {}\0{}", content.len(), content);
    let mut zlib_encoder = ZlibEncoder::new(content.as_bytes(), flate2::Compression::default());
    let mut zlib_content = Vec::new();
    zlib_encoder.read_to_end(&mut zlib_content).unwrap();
    fs::create_dir_all(&file_name_hash_dir).unwrap();
    fs::write(&file_name_hash_file, zlib_content).unwrap();
}

#[test]
fn test_sha1() {
    let mut sha1 = algorithm::sha1::Sha1::new();
    let message = "Hello, World!";
    let expected_hash = "0a0a9f2a6772942557ab5355d76af442f8f65e01";
    let actual_hash = sha1.hash(message.as_bytes());
    assert_eq!(actual_hash, expected_hash);
}
