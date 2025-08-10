mod validator;
mod reader;

fn main() {
    let test = reader::read_parakeet_file(
        "test.🦜");
    let lines = test.lines();
    let thing: Vec<&str> = lines.into_iter().collect();
    dbg!(&test);

}
