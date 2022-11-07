fn main() {
    let mut v = (10, 20);
    let r = &mut v; /* 从可变借用可变 */
    let r0 = &mut r.0; /* 从可变借用可变 */
    let r1 = r.1; /* 从可变借用共享，但是和可变不重叠 */
    *r0 = 11;
    println!("r1 = {}", r1);
}