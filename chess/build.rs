#![feature(coverage_attribute)]

use std::io::Write;
use std::time::Instant;

mod generators;

fn main() {
    if std::path::Path::new("generated.rs").exists() {
        return;
    };

    let mut file = std::fs::File::create("generated.rs");
    if let Ok(ref mut file) = file {
        let i = Instant::now();
        generators::init();
        generators::write(file);
        let duration = Instant::now() - i;
        writeln!(file, "// Generation time {:#?}", duration).unwrap();
    }
}
