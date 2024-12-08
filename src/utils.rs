pub fn read_file_to_lines(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file_contents = std::fs::read_to_string(filename)?.trim().to_owned();
    let mut lines: Vec<String> = Vec::new();

    for line in file_contents.split('\n') {
        lines.push(line.to_owned());
    }

    Ok(lines)
}

pub fn read_file_to_char_vec(filename: &str) -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    let file_contents = std::fs::read_to_string(filename)?.trim().to_owned();
    let mut lines: Vec<String> = Vec::new();

    for line in file_contents.split('\n') {
        lines.push(line.to_owned());
    }

    Ok(lines.into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect())
}
