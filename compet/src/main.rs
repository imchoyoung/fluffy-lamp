mod bj9086;
mod bj2741;

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
    bj2741::main();
    bj9086::main();
    Ok(())
}
