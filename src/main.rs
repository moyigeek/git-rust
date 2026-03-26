#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use flate2::read::ZlibDecoder;
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
        "cat-file"=>{
            if args.len()==4 && args[2]=="-p" {
                let file_path = format!(".git/objects/{}/{}", &args[3][0..2], &args[3][2..]);
                eprintln!("Reading file: {}", file_path);
                let file_content = fs::read(file_path).unwrap();
                let mut decoder = ZlibDecoder::new(&file_content[..]);
                let mut file_content=String::new();
                decoder.read_to_string(&mut file_content).unwrap();
                
                print!("{}", file_content.iter().map(|&b| b as char).collect::<String>());
            } else {
                println!("Usage: cat-file <object>");
            }
        }
        _ => println!("unknown command: {}", args[1]),
    }
}
