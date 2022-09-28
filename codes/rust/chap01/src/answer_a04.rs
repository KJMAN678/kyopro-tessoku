use::std::io;

fn main() {

    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).ok();
    let n: i32 = n_str.trim().parse().expect("Error!");
    
    for x in (0..10).rev() {
        let wari: i32 = 2_i32.pow(x);
        print!("{}", (n / wari) % 2);
    }

    println!("");

}