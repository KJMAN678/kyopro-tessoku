use::std::io;

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).ok();
    let n: i32 = n_str.trim().parse().expect("Error!");
    println!("{}", n * n);
}