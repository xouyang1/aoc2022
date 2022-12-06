# Advent Of Code 2022
![](https://rustacean.net/more-crabby-things/rustdocs.png)

Newborn rustacean hard at work
https://adventofcode.com/2022

|Q | Input | Part 1 | Part2 | Concepts Learned
|--- | --- | --- | --- | --- |
| 1 | 1ms | <1ms |  1ms | Data ETL: `std::path::{Path, PathBuf}`, `std::fs::{canonicalize, read, read_to_string}`, `std::env::current_dir`,  <br/> Data Structures: `std::string::String` vs str, `std::vec::Vec`, `std::collections::BinaryHeap`, <br/> Traits: `std::iter::Iterator`, <br/> Closures, <br/> Iterators: `std::collections::binary_heap::IntoIterSorted`, `std::iter::{Filter, Take}`, `std::slice::Iter`, `std::str::{Lines, Map, Split}` <br/> Metrics: `std::time::Instant`, <br/> Tests |
| 2 | 4ms | <1ms |  4ms | Data ETL: `serde::{Deserialize, Deserializer}`, `csv::ReaderBuilder` <br/> Casting: `as`, <br/> Data Structures: array, `core::array::from_fn`, <br/> Traits and Trait Bounds: `std::convert::From`, `num_derive::FromPrimitive`, `std::iter::IntoIterator`, `where`, impl Trait return <br/> Iterators: `std::iter::Zip`, <br> | 
| 3 | <1ms | 3ms | 6ms | Data Structures: `std::collections::HashSet`, <br/> Iterators: `std::slice::Chunks`, `std::str::Chars` |
| 4 | <1ms | 2ms |  2ms | Data Structures: tuple, `std::ops::Range`, <br/> Closures: as function parameter, <br/> Iterators: `itertools::tuple_impl::Tuples`|
| 5 | <1ms | 700ms |  700ms | Data ETL: `regex::{Captures, Regex}`, <br/> Data Structures: `std::vec::Vec` as stack, <br/> Traits: `std::str::FromStr`, impl Trait parameter, pointer to trait on heap with `dyn` and `std::boxed::Box` <br/> Iterators: `std::iter::{Enumerate, FilterMap, Rev}`, `std::slice::IterMut`, `std::str::SplitWhiteSpace`, <br/> Lifetime and Bounds |
| 6 | <1ms | 1ms |  2ms | Data ETL: `regex::{Captures, Regex}`, <br/> Data Structures: `std::vec::Vec` as stack, <br/> Traits: `std::str::FromStr`, impl Trait parameter, pointer to trait on heap with `dyn` and `std::boxed::Box` <br/> Iterators: `std::iter::{Enumerate, FilterMap, Rev}`, `std::slice::IterMut`, `std::str::SplitWhiteSpace`, <br/> Lifetime and Bounds |
| 6 | <1ms | 1ms |  2ms | Data Structures: `std::collections::{HashMap, hash_map::Entry}`, <br/> Iterators: `std::iter::Inspect`, `std::str::CharIndices` |
