mod read_data;

fn main() {
    if let Err(e) = read_data::main() {
        eprintln!("Error: {}", e);
    }
}