fn main() {
    let mut x = 5;
    println!("x 的數值為：{x}");
    x = 6;
    println!("x 的數值為：{x}");


    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("x 在內部範圍的數值為：{x}");
    }

    println!("x 的數值為：{x}");
    another_function();
}

fn another_function() {
    println!("另一支函式。");
}