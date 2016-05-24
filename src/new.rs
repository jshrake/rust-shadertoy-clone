fn new_image(name: &str) {
    println!("creating image shader {}", name);
}

pub fn execute(kind: &str, name: &str) {
    match kind {
        "image" => new_image(name),
        _ => unreachable!(),
    }
}
