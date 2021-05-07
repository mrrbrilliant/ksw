use walkdir::WalkDir;
use parselnk::Lnk;
use std::path::{Path, PathBuf};
use std::fs::File;

fn main() {

    let start_dir = Path::new("C:\\ProgramData\\Microsoft\\Windows\\Start Menu");
    let mut lnk_files: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(start_dir).follow_links(true) {
        if entry.as_ref().unwrap().path().is_file() {
            let pb = PathBuf::from(entry.as_ref().unwrap().path());
            if pb.file_name().unwrap() != "desktop.ini" {
                lnk_files.push(pb);
            }
        }
    }

    for lnk_file in lnk_files.iter() {
        println!("{:?}", lnk_file);
        let mut file = File::open(lnk_file).unwrap();
        let lnk = Lnk::new(&mut file);
        match lnk {
            Ok(l) => {
                println!("==================================================");
                println!("{:#?}", l.link_info.local_base_path);
                println!("==================================================");
            }
            Err(e) => {
                println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
                eprintln!("{:?}", e);
                println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
            }
        }
    }
}
