fn main() {
    let is_even = return_expression(10);
    println!("{:?}", is_even);
}

fn return_expression(num: i32) -> Option<String> {
    if num % 2 ==  0 {
        Some("Yes".to_string())
    } else {
        None
    }
}