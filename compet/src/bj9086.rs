/*
문자열을 입력으로 주면 문자열의 첫 글자와 마지막 글자를 출력하는 프로그램을 작성하시오.
입력의 첫 줄에는 테스트 케이스의 개수 T(1 ≤ T ≤ 10)가 주어진다.
각 테스트 케이스는 한 줄에 하나의 문자열이 주어진다.
문자열은 알파벳 A~Z 대문자로 이루어지며 알파벳 사이에 공백은 없으며 문자열의 길이는 1000보다 작다.
*/

use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let x: u32 = input.trim().parse().expect("not a int");
    for _ in 0..x {
        input = String::new();
        io::stdin().read_line(&mut input)?;
        let case:String = input.trim().parse().expect("not a string");
        let first = case.chars().next().unwrap();
        let last = case.chars().last().unwrap();
        print!("{first}");
        println!("{last}");
    }
    Ok(())
}
