#![coverage(off)]

use std::fs::File;
use std::io::Write;

const DEBUG: bool = true;

macro_rules! compiled {
    ($f:expr, $name:ident, $bitboard:expr) => {
        writer::write_table($f, $bitboard, stringify!($name));
    };
}

pub fn write_table<T: Table>(f: &mut std::fs::File, bitboard: &T, name: &str) {
    write!(f, "static {}: {} = ", name, bitboard.content_type()).unwrap();
    if DEBUG {
        writeln!(f).unwrap();
    }
    bitboard.write_table(f, 1);
    writeln!(f, ";").unwrap();
}

pub(super) trait Table {
    fn write_table(&self, f: &mut File, ident: usize);
    fn content_type(&self) -> String;
}

fn get_ident(ident: usize) -> String {
    if DEBUG {
        "  ".repeat(ident).to_string()
    } else {
        "".to_string()
    }
}

impl Table for Magic {
    fn write_table(&self, f: &mut File, ident: usize) {
        let ident = get_ident(ident);
        write!(f, "{}Magic::new(", ident).unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
        write!(f, "{}Bitboard::from_u64({}), ", ident, self.mask.as_u64()).unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
        write!(f, "{}{}, ", ident, self.magic).unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
        write!(f, "{}{}, ", ident, self.shift).unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
        write!(f, "{}{}", ident, self.offset).unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
        write!(f, "{})", ident).unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
    }

    fn content_type(&self) -> String {
        "Magic".to_string()
    }
}

impl Table for Bitboard {
    fn write_table(&self, f: &mut File, ident: usize) {
        let ident = get_ident(ident);
        if DEBUG {
            for line in self.to_string().split('\n') {
                writeln!(f, "{}// {}", ident, line).unwrap();
            }
        }
        write!(f, "{}Bitboard::from_u64({})", ident, self.as_u64()).unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
    }

    fn content_type(&self) -> String {
        "Bitboard".to_string()
    }
}

impl<T: Table> Table for Vec<T> {
    fn write_table(&self, f: &mut File, i: usize) {
        let ident = get_ident(i);
        if DEBUG {
            writeln!(
                f,
                "{ident}// Writing vector with {} and L {}",
                self.content_type(),
                self.len()
            )
            .unwrap();
        }
        write!(f, "{ident}[").unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
        for (index, table) in self.iter().enumerate() {
            table.write_table(f, i + 1);
            if index != self.len() - 1 {
                write!(f, ",").unwrap();
                if DEBUG {
                    writeln!(f).unwrap();
                }
            }
        }
        write!(f, "{ident}]").unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
    }

    fn content_type(&self) -> String {
        format!("[{}; {}]", self[0].content_type(), self.len())
    }
}

impl<T: Table, const L: usize> Table for [T; L] {
    fn write_table(&self, f: &mut File, i: usize) {
        let ident = get_ident(i);
        if DEBUG {
            writeln!(
                f,
                "{ident}// Writting table with type {} and L {}",
                self.content_type(),
                L
            )
            .unwrap();
        }
        write!(f, "{ident}[").unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
        for (index, table) in self.iter().enumerate() {
            table.write_table(f, i + 1);
            if index != L - 1 {
                write!(f, ",").unwrap();
                if DEBUG {
                    writeln!(f).unwrap();
                }
            }
        }
        write!(f, "{ident}]").unwrap();
        if DEBUG {
            writeln!(f).unwrap();
        }
    }

    fn content_type(&self) -> String {
        format!("[{}; {}]", self[0].content_type(), self.len())
    }
}

pub(crate) use compiled;
use types::{Bitboard, Magic};
