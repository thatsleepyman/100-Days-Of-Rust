fn main() {
    let nemo = "hello Nemo nem N3mo nemo Nemo! nemo Nemo !";

    let nemo_count = nemo.split_whitespace()
        .filter(|word| *word == "Nemo")
        .count();

    println!("count of Nemo: {}", nemo_count);
}
