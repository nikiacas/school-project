use std::fs;
fn main() {
    let path = "/home/user/example.txt";
    fs::write(path, "Hello, world!").expect("Unable to write file");
}
