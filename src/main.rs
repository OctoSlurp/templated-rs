mod templater;

fn main() {
    // TODO: Error Handling
    let file_path = std::env::args().nth(1).unwrap();
    let file = std::fs::read_to_string(file_path).unwrap();

    templater::format_template(file);
}
