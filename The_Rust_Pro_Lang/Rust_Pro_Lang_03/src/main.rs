fn main() {
    // 测试斐波那契数列计算
    println!("第10个斐波那契数: {}", fibonacci(10));
    println!("第20个斐波那契数: {}", fibonacci(20));
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
    let mut i = 2;

    while i <= n {
        result = a + b;
        a = b;
        b = result;
        i += 1;
    }

    result
}

// fn fibonacci(n: u32) -> u64 {
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     }
//
//     let mut a = 0;
//     let mut b = 1;
//     let mut result = 0;
//
//     for _ in 2..=n {
//         result = a + b;
//         a = b;
//         b = result;
//     }
//
//     result
// }