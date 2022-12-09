use std::fs;

pub fn read_file(file_name: &str) -> String {
    match fs::read_to_string(file_name) {
        Ok(source) => source,
        Err(why) => {
            println!("{}", why.to_string());
            panic!()
        }
    }
}
