use std::io;

fn main() {
    let mut n = String::new();

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut counter = 1;
    while counter < n {
        c = a + b;
        a = b;
        b = c;

        counter += 1;
    }

    println!("{}", c);
}
