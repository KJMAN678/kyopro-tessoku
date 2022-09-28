use::std::io;

#[allow(unused_must_use)]
fn main() {

    let (n, x) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace().map(|i| i.parse::<i32>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut list = String::new();
    io::stdin().read_line(&mut list);
    let a_str: Vec<&str> = list.split_whitespace().collect();
    let a = a_str.iter().map(|i| i.parse().unwrap()).collect::<Vec<i32>>();

    let mut answer = false;

    for _i in 0..n {
        if a[_i as usize] == x {
            answer = true;
        }
    }

    // 出力
    if answer {
        println!("Yes");
    } else {
        println!("No")
    }
}