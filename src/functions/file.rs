pub fn write_file(file_name: &str, contents: &str) {
    std::fs::write(file_name, contents).unwrap();
}

pub fn file_count(dir: &str) -> i32 {
    let base = std::path::Path::new(dir);

    let mut count = 0;

    for _entry in std::fs::read_dir(base).unwrap() {
        count += 1;
    }

    count
}

pub fn target_file(file_name: &str) -> String {
    let contents = std::fs::read_to_string(file_name).unwrap();
    contents
}
