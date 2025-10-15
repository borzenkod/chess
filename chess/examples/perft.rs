use std::{time::Instant};

use chess::{START_POS, Chessboard, perft as g_perft};

#[cfg(not(clippy))]
const fn _perft(d: u32) -> usize {
    let mut cb = BOARD;
    g_perft(&mut cb, d)
}

#[cfg(clippy)]
const fn _perft(_: u32) -> usize {
    let mut cb = BOARD;
    g_perft(&mut cb, 0)
}

const FEN: &str = START_POS;
const BOARD: Chessboard = match Chessboard::from_fen(FEN) {
    Ok(v) => v,
    Err(_) => unreachable!()
};
const COMPUTE_TO: usize = 3;
#[allow(long_running_const_eval)]
const PERFT: [usize; COMPUTE_TO + 1] = {
    let mut arr = [0usize; COMPUTE_TO + 1];
    let mut index = 0;
    loop {
        if index > COMPUTE_TO {
            break;
        }
        arr[index] = _perft(index as u32);
        index += 1;
    }
    arr
};

fn main() {
    println!("Runtime:");
    for depth in 1..=COMPUTE_TO {
        let now = Instant::now();
        let perft = _perft(depth as u32);
        println!(" perft @depth {} is #{}", depth, perft);
        let elapsed = now.elapsed();
        println!(" Elapsed: {:?}", elapsed);
    }
    println!("Compile-time:");
    for depth in 1..=COMPUTE_TO {
        let now = Instant::now();
        let perft = PERFT[depth];
        println!(" perft @depth {} is #{}", depth, perft);
        let elapsed = now.elapsed();
        println!(" Elapsed: {:?}", elapsed);
    }
}
