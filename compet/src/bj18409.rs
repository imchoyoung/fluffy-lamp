/*
長さ N の英小文字からなる文字列 S が与えられる．S のうち母音字の個数，
つまり a，i，u，e，o の個数の総和を求めよ．
입력
入力は以下の形式で標準入力から与えられる．
N
S
출력
S のうち母音字の個数，つまり a，i，u，e，o の個数の総和を出力せよ．
제한
    1 ≦ N ≦ 50.
    S は長さ N の文字列である．
    S の各文字は英小文字である．
*/
use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Input is not a  integer");

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let s = s.trim(); // Remove any trailing newline or whitespace
    assert!(s.len() == n, "Input line must have {n} characters");

    let vowels = ['a', 'i', 'u', 'e', 'o'];
    let count = s
        .chars()
        .filter(|c| vowels.contains(c)) // Filter vowels
        .count(); // Count the filtered characters

    println!("{}", count);
    Ok(())
}
