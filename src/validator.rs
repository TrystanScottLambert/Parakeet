// Module to validate that a file is actually a parakeet file.

// Check file extension

pub const VALID_EXTENSIONS: [&str;3] = [".prkt", ".parakeet" , ".🦜"];

pub fn is_valid_extension(file_name: &str) -> bool {
    let index = file_name.find('.');
    let idx = match index {
        Some(idx) => idx,
        _ => return false
    };
    
    let (_, extension) = file_name.split_at(idx);
    VALID_EXTENSIONS.contains(&extension)
}

#[cfg(test)]
mod tests {
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
}
