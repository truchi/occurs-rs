mod occurs;
mod sorted_occurs;

use glob::glob;
use occurs::*;
use sorted_occurs::*;
use std::{
    env,
    fmt::{self, Debug, Display, Formatter},
    fs::read_to_string,
    ops::{Add, AddAssign, Index, IndexMut},
};

const MAX: usize = 127;

fn main() {
    let path = env::args().nth(1).expect("Usage: cargo r <glob>");
    let mut occurs = Occurs::default();
    let mut file_count = 0;

    for entry in glob(&path).expect("Failed to read glob pattern") {
        let path = entry.expect("Failed to unwrap entry");
        let content = read_to_string(path).expect("Failed to read file");

        occurs += &content[..];
        file_count += 1;
    }

    println!("{} ({} files)", path, file_count);
    println!("{}", SortedOccurs::from(occurs));
}
