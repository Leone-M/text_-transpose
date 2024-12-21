    use std::fs;

fn main(){
    let parsed_file = match fs::read_to_string("tipa_array.txt") {
        Ok(string) => string,
        Err(_) => panic!("Error reading file!"),
    };
    println!("File parsed:\n{:?}", parsed_file);
    let first_vec: Vec<&str> = parsed_file.trim().split("\n").collect();
    let mut splited_vec: Vec<Vec<&str>> = Vec::new();
    // println!("{:?}", first_vec);
    for i in 0..first_vec.len() {
        splited_vec.push(first_vec[i].split(" ").collect());
    }
    println!("{:?}", splited_vec);
    let mut transposed_vec: Vec<Vec<&str>> = Vec::new();
    for _ in 0..splited_vec[0].len(){
        let empty_vec: Vec<&str> = vec![""; splited_vec.len()];
        transposed_vec.push(empty_vec);
    }
    for i in 0..splited_vec.len() {
        for j in 0..splited_vec[i].len() {
            transposed_vec[j][i] = splited_vec[i][j];
        }
    }
    println!("{:#?}", transposed_vec);
    let mut string_to_write: String = String::new();
    for i in 0..transposed_vec.len() {
        for j in 0..transposed_vec[i].len(){
            string_to_write.push_str(transposed_vec[i][j]);
            string_to_write.push_str(" ");
        }
        string_to_write = String::from(string_to_write.trim());
        string_to_write.push_str("\n");
    }
    println!("String to write:\n{}", string_to_write);
    fs::write("array_tipa.txt", string_to_write).expect("Failed to write a file!");
}