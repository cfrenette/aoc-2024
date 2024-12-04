use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};

pub fn read_input_by_line<P: AsRef<Path>>(path: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_input_to_string<P: AsRef<Path>>(path: P, buf: &mut String) -> io::Result<usize> {
    let mut file = File::open(path)?;
    file.read_to_string(buf)
}
