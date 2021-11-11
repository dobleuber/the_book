fn main() {
    let data = "initial contents";

    let s = data.to_string();

    println!("{}", s);

    // the method also works on a literal directly:
    let s = String::from("initial contents");

    println!("{}", s);

    let salutes = vec!["السلام عليكم",
        "Dobrý den",
        "Hello",
        "שָׁלוֹם",
        "नमस्ते",
        "こんにちは",
        "안녕하세요",
        "你好",
        "Olá",
        "Здравствуйте",
        "Hola",
    ];

    for s in salutes {
        let s = s.to_string();
        println!("{}", s);
    }

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("s3 is {}", s3);

    let mut hello = "Здравствуйте".to_string();

    let s = &hello[0..4];

    println!("{}", s);

    hello.push_str("!");

    println!("{}", hello);

    for c in hello.chars() {
        println!("{}", c);
    }
    
    for b in hello.bytes() {
        println!("{}", b);
    }
}
