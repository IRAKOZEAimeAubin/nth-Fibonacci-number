use std::io;

fn main() {
    println!("nth Fibonacci number");
    println!("Find the nth Fibonacci number");

    loop {
        println!("Enter the position of the Fibonacci number:");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n == 1 {
            println!(
                "The {}st Fibonacci number using iteration is: {}",
                n,
                fibonacci_iterative(n)
            );
            continue;
        } else if n == 2 {
            println!(
                "The {}nd Fibonacci number using iteration is: {}",
                n,
                fibonacci_iterative(n)
            );
            continue;
        } else if n == 3 {
            println!(
                "The {}rd Fibonacci number using iteration is: {}",
                n,
                fibonacci_iterative(n)
            );
            continue;
        } else {
            println!(
                "The {}th Fibonacci number using iteration is: {}",
                n,
                fibonacci_iterative(n)
            );
            continue;
        }
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut x: u64 = 0;
    let mut y: u64 = 1;
    let mut z: u64;

    for _ in 2..=n {
        z = x + y;
        x = y;
        y = z;
    }

    y
}
