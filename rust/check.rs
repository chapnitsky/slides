use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

// from all the md files extract the list of included files
// from the examples/ directory list all the files
// make sure each file is included and is only included once

// Run every rs file, if there is an out file compare the results.

fn main() {
    //get_md_files()
    let md_files = get_md_files();
    println!("{:?}", md_files);
    let examples = get_examples();
    println!("{:?}", examples);
}

fn get_examples() -> Vec<PathBuf> {
    let mut examples = vec![];
    let path = Path::new("examples");
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let dirname = entry.path();
            if dirname.is_dir() {
                //println!("dir");
                for filepath in dirname.read_dir().expect("read_dir call failed") {
                    if let Ok(filepath) = filepath {
                        // println!("{:?}", filepath);
                        let filename = filepath.path();
                        examples.push(filename);
                    }
                }
            } else {
                println!("'ERROR: {:?}' is not a directory", dirname);
                exit(1);
            }
        }
    }
    return examples
}

fn get_md_files() -> Vec<PathBuf> {
    let mut md_files = vec![];
    let path = Path::new(".");
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let filename = entry.path();
            // println!("{:?}", filename); //.as_path());
            let extension = filename.extension();
            if extension != None {
                if extension.unwrap() == "md" {
                    // println!("{:?}", filename);
                    //println!("{}", filename);
                    md_files.push(filename);
                }
                //println!("{:?}", extension.unwrap())
            }
        }
    }
    return md_files
}