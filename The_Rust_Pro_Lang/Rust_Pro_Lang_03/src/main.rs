fn main() {

    
    let mut a = 0;
    let mut b = 1;
    let mut result = 0;
    let mut i = 2;

    let n = 1;
    
    while i <= n {
        if n == 0 {
            println!("0");
        } else if n == 1 {
            println!("1");

        }
        result = a + b;
        a = b;
        b = result;
        i += 1;
    }
    
    println!("{}", result);
    
    
}
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut result = 0;

    for _ in 2..=n {
        result = a + b;
        a = b;
        b = result;
    }

    result
}