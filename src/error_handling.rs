pub fn module_function(value: String) -> String {
    let thing = value + " added a string";
    return thing.to_string();
}

pub fn read_file(file_name: String) {
    let file = std::fs::read_to_string(file_name)
    .expect("unable to read the file to string");

    file.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value);
        } else {
            println!("Line not a number")
        }
    });

}