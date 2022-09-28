use::std::io;

fn main() {

    // 入力
    let (n, k) = {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace().map(|i| i.parse::<i32>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut answer :i32 = 0;

    for x in 1..=n {
        for y in 1..=n {
            let z :i32 = k - x - y;
            if z >= 1 && z <= n {
                answer += 1;
            }
        }
    }

    println!("{}", answer);
}