/*
The math teacher gave the students the unknown x to find from the
equality: a*x = b-c. The numbers a, b and c are natural numbers and are such
that the solution x is an integer. Znaiko attended the computer science school
and tried to make a program, but had difficulty.
Help him by writing an equation program that finds the unknown x.
The first line of the standard input is given three natural numbers a, b, c.
At the standard output, the program must output a natural number x such
that the given equality is satisfied.
*/

use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let a: u64 = input.trim().parse().expect("not a int");
    input = String::new();
    io::stdin().read_line(&mut input)?;
    let b: u64 = input.trim().parse().expect("not a int");
    input = String::new();
    io::stdin().read_line(&mut input)?;
    let c: u64 = input.trim().parse().expect("not a int");
    let x: u64 = (b-c)/a;
    println!("{x}");
    Ok(())
}
