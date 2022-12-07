/// rust 展开一个宏调用时，会逐项匹配每条规则的模式,
/// 如果没有可以匹配的规则则会报错。
macro_rules! vec_new {
    // 第一条规则
    // 这个规则有两个表达式
    // 第一个表达式后面有一个 ; 号跟着第二个表达式
    // 处理 vec_new![0, 10] 这样的调用
    ($elem: expr ; $n: expr) => {
        ::std::vec::from_elem($elem, $n)
    };
    // 第二条规则 处理 vec_new![1, 2, 3, 4] 这样的调用
    // $( $x: expr ), * 表达 0 个或者更多的表达式，以 , 分隔
    // 其中的每一项都与模式进行匹配
    // $( PATTERN ) , * 用来匹配任意逗号分割的列表，列表中的每一项都与 PATTERN 匹配
    ($( $x: expr ), *) => {
        // <i32>::into_vec(Box::new(1), Box::new(2), ...)
        <[_]>::into_vec(Box::new([ $ ( $x ), *]))
    };
    // 第三条规则
    // 用来支持尾部逗号，
    // 如果存在尾部逗号，重试没有它的情况,
    // 递归调用 vec_new! ，
    // 这一次将会匹配第二条规则
    ( $( $x: expr ), + ,) => {
        vec_new![ $( $x ), *]
    };
}

#[test]
fn test_macro() {

    let v = vec_new!(0; 10);
    println!("{:?}", v);

    let v = vec_new!(1, 2, 3);
    println!("{:?}", v);

    let v = vec_new![1, 2, 3,];
    println!("{:?}", v);

}

