use rand::prelude::*;
use rayon::prelude::*;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel::<(i64, Vec<u8>)>();
    let reader_thread = std::thread::spawn(move || {
        let mut buf: Box<Vec<_>> = Box::new(Vec::with_capacity(100_000));
        loop {
            coz::scope!("reading channel");
            if let Some(res) = rx.recv().ok() {
                buf.push(res);
            } else {
                break;
            }
        }
        buf.sort_unstable_by_key(|&(idx, _)| -idx);
        coz::begin!("flushing channel");
        for (_i, l) in buf.into_iter() {
            println!(
                "{}",
                std::str::from_utf8(&l).expect("failed to restringify")
            );
        }
        coz::end!("flushing channel");
    });

    let height = 2000;
    let lines = (0..=(height - 1)).rev().collect::<Vec<_>>();
    lines.par_iter().for_each_with(tx, |tx, &j| {
        let mut rng = rand::thread_rng();
        let mut buf = Vec::with_capacity(100_000);
        coz::begin!("scanline");
        for _ in 0..100_000 {
            buf.extend(format!("{}", rng.gen::<f64>()).into_bytes());
        }
        tx.send((j, buf)).unwrap();
        coz::end!("scanline");
    });

    reader_thread.join().unwrap();
    println!("Hello, world!");
}
