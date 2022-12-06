use csv::ReaderBuilder;
use std::fs;
use std::path::Path;

pub fn read_file_to_string(src_file: &str, read_file: &str) -> String {
    fs::read_to_string(Path::new(src_file).with_file_name(read_file)).unwrap()
}

pub fn read_file_to_struct<T: serde::de::DeserializeOwned>(
    src_file: &str,
    read_file: &str,
) -> Vec<T> {
    ReaderBuilder::new()
        .has_headers(false)
        .from_path(Path::new(src_file).with_file_name(read_file))
        .unwrap()
        .deserialize::<T>()
        .map(|item| item.unwrap())
        .collect()
}

pub fn read_str_to_struct_iter<T>(data: &str) -> impl Iterator<Item = T> + '_
where
    T: std::str::FromStr<Err = ()>,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    data.lines().map(|line| line.parse::<T>().unwrap())
}

pub fn group_lines_iter(raw: &str) -> impl Iterator<Item = &str> {
    raw.split("\n\n").filter(|&x| !x.trim().is_empty())
}

pub fn str_split_half(s: &str) -> (&str, &str) {
    s.split_at(s.len() / 2)
}

pub fn get_re_name<T>(caps: &regex::Captures, name: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    caps.name(name).unwrap().as_str().parse::<T>().unwrap()
}
