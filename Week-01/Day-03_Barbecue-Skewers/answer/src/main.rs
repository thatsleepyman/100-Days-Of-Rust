fn main() {
    let bbq_vecs: (Vec<&str>, Vec<&str>) = (
        vec![
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ],
        vec![
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----"
        ],
    );

    println!("{:?}", bbq_vecs);

    for vecs in bbq_vecs.iter(Vec::&str, Vec::&str) {
        println!("{:?}", vecs);
    }
}
