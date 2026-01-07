fn main() {
    let file = midi::File::new();

    println!("bpm: {}", file.bpm);
}
