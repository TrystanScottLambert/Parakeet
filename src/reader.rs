use std::fs;


 pub fn read_parakeet_file(file_name: &str) -> String {
    match fs::read_to_string(file_name) {
        Ok(val) => val,
        Err(e) => panic!("Not able to read file because of error {e}")
    }
 }


