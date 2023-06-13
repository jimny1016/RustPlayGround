fn main() {
    let result = fibonacci(30);
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    println!("{b}！");
    for x in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
        println!("{b}！");
    }
    b
}