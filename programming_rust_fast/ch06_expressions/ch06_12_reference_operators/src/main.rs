fn main() {
    reference_operator();
}

/// 引用运算符：
/// - & 共享引用
/// - &mut 可变引用
/// 解引用运算符 * ：
/// 获取引用指向的值，
/// 当使用 . 运算符的时候，不需要 * 运算符，
/// 因为使用 . 运算符访问域或者方法是访问某个值的一部分，
/// 只有在需要读写整个值的时候才需要使用 * 运算符。
fn reference_operator() {
    let mut wong = build_personal_file("Wong".to_string(), 20);
    desc_person(&wong);
    change_name(&mut wong, "Wong Er");
    desc_person(&wong);
    change_age(&mut wong, 22);
    desc_person(&wong);

    let num = &mut 10;
    *num = 20;
    println!("num => {}", *num);
}

struct Person {name: String, age: u8}

fn build_personal_file(name: String, age: u8) -> Person {
    Person {name, age}
}

fn desc_person(person: &Person) {
    println!("Person : name = {}, age = {}", person.name, person.age);
}

fn change_name(person: &mut Person, new_name: &str) {
    person.name = new_name.to_string();
}

fn change_age(person: &mut Person, age: u8) {
    person.age = age;
}