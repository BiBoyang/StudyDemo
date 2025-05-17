fn main() {
    println!("Hello, world!");
    
    let a:i32 = 5;
    
    println!("The value of a is {}", a);
    
    
    let t:bool = true;
    let f = false;   
    println!("The value of t is {t},{f}", );
    
    let tup = (500, 6.4, "hello",true);
    print!("{}",tup.0);
    
    let arr = [1,2,3,4,5];
    
    println!(" {}",arr[0]);
    
    //函数
    anther();
    
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("{}",y);
}

fn anther() {
    println!("Hello, Boyang!");
}
