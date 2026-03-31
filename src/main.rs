use crate::algorithm::sha1;
use flate2::read::{ZlibDecoder, ZlibEncoder};
use std::env;
use std::fs;
use std::io::Read;
use std::str;
mod algorithm;

// struct TreeNode{
//     mode:String,
//     name:String,
//     hash:String,
// }
// impl TreeNode {
//     fn new(input:String)->Self{
//         let (title,blob)=split_content(input);
//         let iter=title.splitn(2,' ');
//         Self{
//             hash:blob
            
//         }
//     }
// }

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage
    let args: Vec<String> = env::args().collect();
    let num_arg=args.len();
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
            if num_arg == 4 && args[2] == "-p" {
                let file_path = get_object_path(args[3].clone());
                eprintln!("Reading file: {}", file_path);
                let file_content = fs::read(file_path).unwrap();
                let mut decoder = ZlibDecoder::new(&file_content[..]);
                let mut file_content = String::new();
                let _=decoder.read_to_string(&mut file_content).unwrap();
                // 只要\0 后面的内容
                let (_, file_content) = split_content(file_content);
                print!("{}", file_content);
            } else {
                println!("Usage: cat-file <object>");
            }
        }
        "hash-object" => {
            if num_arg == 3 {
                let file_path = &args[2];
                let content = read_file(file_path);
                let mut sha1_encoder = sha1::Sha1::new();
                println!("{}", sha1_encoder.hash(&content));
            } else if num_arg == 4 && args[2] == "-w" {
                let file_path = &args[3];
                let data=read_file(file_path);
                let content =  std::str::from_utf8(&data).unwrap();
                save_blob(&content);
            }
        }
        "ls-tree" => {
            if num_arg==3{
                let file_path=get_object_path(args[2].clone());
                let content=read_file(&file_path);
                let de_content=zlib_decode(&content);
                let (_,object_raw)=split_content(de_content);
                eprintln!("{}",object_raw);
            }
            if num_arg==4&&args[2]=="--name-only"{
                let file_path=get_object_path(args[3].clone());
                let content=read_file(&file_path);
                let de_content=zlib_decode(&content);
                let (_,object_raw)=split_content(de_content);
                eprintln!("{}",object_raw);

            }

        }
        _ => println!("unknown command: {}", args[1]),
    }
}

fn read_file(file_path: &str) -> Vec<u8> {
    let file_content = fs::read(file_path).unwrap();
    file_content
}

fn write_file(file_path: &str, content: &[u8]) {
    fs::write(file_path, content).unwrap();
}
fn split_content(content: String) -> (String, String) {
    let iter = content.split_once('\0').unwrap();
    (iter.0.into(), iter.1.into())
}

fn get_object_path(hash:String)->String{
    format!(".git/objects/{}/{}", &hash[0..2],&hash[2..])
}

fn zlib_decode(content:&[u8])->String{
    let mut decoder=ZlibDecoder::new(content);
    let mut decoded_content=String::new();
    let _ =decoder.read_to_string(&mut decoded_content);
    decoded_content
}

fn zlib_encode(content:String)->Vec<u8>{
    let mut encoder = ZlibEncoder::new(content.as_bytes(), flate2::Compression::default());
    let mut compressed_content = Vec::new();
    encoder.read_to_end(&mut compressed_content).unwrap();
    compressed_content
}

fn save_blob(content: &str) {
    let blob_content = format!("blob {}\0{}", content.len(), content);
    let hash = algorithm::sha1::Sha1::new().hash(blob_content.as_bytes());
    println!("{}", hash);
    let object_path = format!(".git/objects/{}/{}", &hash[0..2], &hash[2..]);
    eprintln!("Saving blob to: {}", object_path);
    fs::create_dir_all(format!(".git/objects/{}", &hash[0..2])).unwrap();
    let compressed_content=zlib_encode(blob_content);
    write_file(&object_path, &compressed_content);
    eprintln!("Saved blob to: {}", object_path);
}



#[test]
fn test_sha1() {
    let mut sha1 = algorithm::sha1::Sha1::new();
    let message = "Hello, World!";
    let expected_hash = "0a0a9f2a6772942557ab5355d76af442f8f65e01";
    let actual_hash = sha1.hash(message.as_bytes());
    assert_eq!(actual_hash, expected_hash);
}
