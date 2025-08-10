// Module to validate that a file is actually a parakeet file.

use core::panic;
use std::collections::HashSet;

pub const VALID_EXTENSIONS: [&str;3] = [".prkt", ".parakeet" , ".🦜"];
pub const VALID_ID_TYPES: [&str; 2] = ["int", "string"];
pub const VALID_GENERAL_TYPES: [&str; 3] = ["int", "float", "string"];


pub fn is_valid_extension(file_name: &str) -> bool {
    let index = file_name.find('.');
    let idx = match index {
        Some(idx) => idx,
        _ => return false
    };
    
    let (_, extension) = file_name.split_at(idx);
    VALID_EXTENSIONS.contains(&extension)
}

pub fn is_id_type_valid(file_contents: String) -> bool {
    let lines: Vec<&str> = file_contents.lines().into_iter().collect();
    let id_type =  match lines.get(0) {
        Some(value) => value,
        _ => return true // If empty then it's fine.
    };
    VALID_ID_TYPES.contains(id_type)
}

pub fn get_ids(file_contents: String) -> Vec<String> {
    let mut new_line_counter = 0;
    let mut ids = Vec::new();
        
    for line in file_contents.lines() {
        
        if new_line_counter == 1 {
            let colon_idx = match line.find(':') {
                Some(idx) => idx,
                _ => panic!("Missing colon in id field: {line}")
            };
            let (value, _) = line.split_at(colon_idx);
            ids.push(value.to_string());
            new_line_counter = 0;
        }

        if line.is_empty() {
            new_line_counter += 1
        }
    }
    ids
}

 pub fn are_ids_unique(ids: Vec<String>) -> bool {
    let mut seen = HashSet::new();
    for id in ids {
        if !seen.insert(id.clone()) {
            return false;
        }
    }
    true
}

fn are_ids_ints(ids: Vec<String>) -> bool {
    for id in ids {
        match id.parse::<i32>() {
            Ok(_) => (),
            Err(_) => return false
        };
    }
    true
}



#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    #[test]
    fn extensions() {
        let good_prkt: &str = "hello.prkt";
        let good_parakeet: &str = "hello.parakeet";
        let good_parrot: &str = "hello.🦜";
        let bad_1: &str = "hello.txt";
        let bad_2: &str = "hello";
        let bad_3: &str = "hello.prkt.txt";

        assert!(is_valid_extension(good_prkt));
        assert!(is_valid_extension(good_parakeet));
        assert!(is_valid_extension(good_parrot));
        assert!(!is_valid_extension(bad_1));
        assert!(!is_valid_extension(bad_2));
        assert!(!is_valid_extension(bad_3));
        
    }

    #[test]
    fn getting_ids() {
        let test_contents = String::from("int:\n\tname:string\n\n1:\n\tname: Trystan\n\n2:\n\tname:poop\n");
        let res = get_ids(test_contents);
        let ans = ["1", "2"];
        for (r, a) in zip(res, ans) {
            assert_eq!(r, a)
        }
    }

    #[test]
    fn ids_are_ints() {
        let ids_bad = vec!["1".to_string(), "test".to_string()];
        let ids_good = vec!["1".to_string(), "2".to_string()];
        assert!(!are_ids_ints(ids_bad));
        assert!(are_ids_ints(ids_good));
    }

    #[test]
    fn unique_ids() {
        let ids_bad = vec!["1".to_string(), "1".to_string(), "2".to_string()];
        let ids_good = vec!["1".to_string(), "2".to_string()];
        assert!(!are_ids_unique(ids_bad));
        assert!(are_ids_unique(ids_good));
    }
}
