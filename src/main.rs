use engine;

fn main() {
    let input_file = std::env::args()
        .nth(1)
        .expect("Please supply an input file");

    match engine::process_transactions(&input_file) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("An error occurred: {}.", err.to_string())
        }
    }
}
