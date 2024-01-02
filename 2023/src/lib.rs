
pub fn load_input(day: u32) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    String::from_utf8(std::fs::read(path).unwrap()).unwrap()
}

pub fn parse_iter<'a, T>(iter: impl Iterator<Item = &'a str>) -> Vec<T>
where
    T: core::str::FromStr,
    T::Err: core::fmt::Debug,
{
    iter.filter(|s| !s.is_empty())
        .map(|v| v.parse().unwrap())
        .collect()
}

pub mod range_map;
