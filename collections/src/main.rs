fn main() {
    let mut v = vec![1, 2, 3, 4, 5];


    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let does_not_exist = v.get(100);

    match does_not_exist {
        Some(third) => println!("The 100th element is {}", third),
        None => println!("There is no 100th element."),
    }

    v.push(6);
    
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }
    
}
