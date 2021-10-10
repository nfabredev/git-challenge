use flate2::read::ZlibDecoder;
use flate2::read::ZlibEncoder;
use flate2::Compression;
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        init();
    } else if args[1] == "cat-file" {
        let blob_sha = &args[3];
        cat_file(blob_sha);
    } else if args[1] == "hash-object" {
        let file_name = &args[3];
        hash_object(file_name);
    } else {
        println!("unknown command: {}", args[1]);
    }
}

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    println!("Initialized git directory")
}

fn cat_file(blob_sha: &String) {
    // first 2 chars of blob path are its directory name
    let object_path = blob_sha.split_at(2);

    let mut path = ".git/objects/".to_string();

    path.push_str(&object_path.0.to_string()); // directory name
    path.push_str(&"/".to_string()); // path separator
    path.push_str(&object_path.1.to_string()); // filename

    let blob = fs::File::open(path).expect("Unable to open");

    let mut content = ZlibDecoder::new(&blob);
    let mut buffer = String::new();
    content.read_to_string(&mut buffer).unwrap();
    // git ojects have a header separated by a NULL codepoint
    // https://git-scm.com/book/en/v2/Git-Internals-Git-Objects#_object_storage
    let null_ascii_codepoint = '\x00';
    // that we discard
    let result = buffer.split(null_ascii_codepoint).collect::<Vec<&str>>()[1];
    print!("{}", result);
}

fn hash_object(file_name: &String) {
    let original = fs::File::open(file_name).expect("Unable to open");
    let mut original_reader = io::BufReader::new(original);

    let data = encode_file(&mut original_reader).expect("Failed to encode file");

    let header = "blob \x00".as_bytes().to_vec();

    // FIXME: does this really prepends the header to the data ?
    let object: Vec<u8> = data.into_iter().chain(header.into_iter()).collect();

    let encoded = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("1234567890")  // TODO compute hash name
        .expect("Failed to write file");

    let mut encoded_writer = io::BufWriter::new(encoded);
    encoded_writer
        .write_all(&object)
        .expect("Failed to write encoded file");
}

fn encode_file(file: &mut dyn io::Read) -> io::Result<Vec<u8>> {
    let mut encoded = ZlibEncoder::new(file, Compression::fast());
    let mut buffer = Vec::new();
    encoded.read_to_end(&mut buffer)?;
    return Ok(buffer);
}

// fn ls(dir_path: String) {
//     let paths = fs::read_dir(dir_path).expect("Unable to open");

//     for path in paths {
//         println!("Name: {}", path.as_ref().unwrap().path().display());
//         ls(path.unwrap().path().display().to_string());
//     }
// }
