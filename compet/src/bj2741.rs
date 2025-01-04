// use std::io::{self, BufRead, Write};

// fn main() {
//     let mut buffer = String::new();
//     let stdin = io::stdin();
//     let mut handle = stdin.lock();
//     handle.read_line(&mut buffer).unwrap();
//     let jeongsu = buffer
//         .trim()
//         .parse::<u32>()
//         .unwrap();
//     let stdout = io::stdout();
//     let mut handle2 = stdout.lock();
//     for i in 1..=jeongsu {
//         handle2.write_fmt(format_args!("{}\n", i)).unwrap();
//     }
//     handle2.flush().unwrap();
// }

use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    handle.read_line(&mut buffer).unwrap();
    let jeongsu: u32 = buffer.trim().parse().unwrap();

    let stdout = io::stdout();
    let mut handle2 = stdout.lock();
    let mut output = String::with_capacity(jeongsu as usize * 2);

    for i in 1..=jeongsu {
        output.push_str(&i.to_string());
        output.push('\n');
    }
    handle2.write_all(output.as_bytes()).unwrap();
}
