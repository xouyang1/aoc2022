# Advent Of Code 2022
![](https://rustacean.net/more-crabby-things/rustdocs.png)

Newborn rustacean hard at work
https://adventofcode.com/2022

|Q | Part 1 | Part2 | Concepts Learned
|--- | --- | --- | --- |
| 1 | 2ms |  2ms | Files: `std::path::{Path, PathBuf}`, `std::fs::{canonicalize, read, read_to_string}`, `std::env::current_dir`,  <br/> Data Structures: `std::string::String` vs str, `std::vec::Vec`, `std::collections::BinaryHeap`, <br/> Traits: `std::iter::Iterator`, <br/> Closures, <br/> Iterators: `std::collections::binary_heap::IntoIterSorted`, `std::iter::{Filter, Take}`, `std::slice::Iter`, `std::str::{Lines, Map, Split}` <br/> Metrics: `std::time::Instant`, <br/> Tests |
| 2 | 4ms |  4ms | Files: `serde::{Deserialize, Deserializer}`, `csv::ReaderBuilder` <br/> Casting: `as`, <br/> Data Structures: array, `core::array::from_fn`, <br/> Traits and Trait Bounds: `std::convert::From`, `num_derive::FromPrimitive`, `std::iter::IntoIterator`, `where`, <br/> Iterators: `std::iter::Zip` | 
| 3 | 4ms | 10ms | Data Structures: `std::collections::HashSet`, <br/> Iterators: `std::slice::Chunks`, `std::str::Chars` |
| 4 | 3ms |  5ms | Data Structures: tuple, `std::ops::Range`, <br/> Closures: as function parameter, <br/> Iterators: `itertools::tuple_impl::Tuples`|
