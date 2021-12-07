use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub fn read_input<P, U>(path: P) -> impl Iterator<Item = U>
where
    P: AsRef<Path>,
    U: FromStr,
    U::Err: std::fmt::Debug,
{
    read_input_with_parse(path, |line| line.parse().unwrap())
}

pub fn read_input_with_parse<P, U>(
    path: P,
    mut parse: impl FnMut(String) -> U,
) -> impl Iterator<Item = U>
where
    P: AsRef<Path>,
    U: FromStr,
    U::Err: std::fmt::Debug,
{
    let input = BufReader::new(File::open(path).unwrap());

    input.lines().map(move |line| parse(line.unwrap()))
}
