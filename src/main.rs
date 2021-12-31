use std::{collections::HashMap};
fn main() {
    println!("The first fibonacci number is: {}", nth_fibonacci(0));
    println!("The second fibonacci number is: {}", nth_fibonacci(1));
    println!("The third fibonacci number is: {}", nth_fibonacci(2));
    println!("The fourth fibonacci number is: {}", nth_fibonacci(3));
    println!("The fifth fibonacci number is: {}", nth_fibonacci(4));
    println!("The sixth fibonacci number is: {}", nth_fibonacci(5));
    println!("The sixth fibonacci number is: {}", nth_fibonacci(400))
}

fn nth_fibonacci(num: u32) -> u32 {
    let mut cache: HashMap<u32, u32> = HashMap::new();

    fn fib(num: u32, cache: &mut HashMap<u32, u32>) -> u32 {
        // formula Fn = Fn − 1 + Fn − 2 , where n > 1

        if num == 0 || num == 1 {
            return num;
        }

        match cache.get(&num) {
            Some(num) => return *num,
            None => cache.insert(num, num),
        };

        fib(num - 1, cache) + fib(num - 2, cache)
    }

    fib(num, &mut cache)
}
