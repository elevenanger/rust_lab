mod array_and_slice {

    /// 数组模式匹配：
    fn hsl_to_rgb(hsl: [u8; 3]) -> [u8; 3] {
        match hsl {
            [_, _, 0] => [0, 0, 0],
            [_, _, 255] => [255, 255, 255],
            [r, g, b] => [r, g, b]
        }
    }

    #[test]
    fn test_hsl_to_rgb() {
        println!("{:?}", hsl_to_rgb([10, 10, 255]));
    }

    /// 切片模式匹配：
    fn greet_people(names: &[&str]) {
        match names {
            [] => println!("Hello, Nobody."),
            [a] => println!("Hello, {}.", a),
            [a, b] => println!("Hello, {} and {}.", a, b),
            [a, .., b] => println!("Hello ,everyone from {} to {}", a, b)
        }
    }

    #[test]
    fn test_greet_people() {
        greet_people(&[]);
        greet_people(&["Zhang"]);
        greet_people(&["Li"]);
        greet_people(&["Niu", "Ma"]);
        greet_people(&["Zhao", "Qian", "Sun"]);
    }
}