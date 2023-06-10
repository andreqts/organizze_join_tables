use std::env;
use std::fs;

fn list_files_in_dir(fpath: &str) -> Vec<String> {
    let mut vfiles: Vec<String> = Vec::new();
    let dirv = fs::read_dir(fpath);
    let dirv = match dirv {
        Ok(direc) => direc,
        Err(error) => panic!("Error opening dir '{}': '{:?}'", fpath, error),
    };
    for f in dirv {
        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("Error reading file name in folder '{}': '{:?}'", fpath, error),
        };
        vfiles.push(f.path().display().to_string());
    }
    vfiles
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {i} -> '{arg}'");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_dirs() {    
        let vf  = list_files_in_dir(".\\tests\\testdir");
        let expected_results = [ "f1.txt", "f2.txt", "f3.txt" ];
        for (i, f) in vf.iter().enumerate() {
            let expected_file = expected_results[i];
            assert!(f.contains(expected_file));
        }
    }
}