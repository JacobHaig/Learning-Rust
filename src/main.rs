pub struct Person{
    name : String,
    id : i32
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

fn area(r : &Rect) -> u32 {
    r.width* r.height
}

fn main() {
    let r = Rect{width: 30 , height: 50 };
    println!("The area is {}", area(&r));
    println!("{:?}", r);

    let p = Person{
        name : String::from("Jacob"),
        id: 13425,
    };
    println!("name: {} id: {}", p.name, p.id);


    println!("{}: {}",20, fib(20));


    let s1= String::from("Hello world");
    let s2 = "World Hello";

    print!("{}, " , first_word(&s1));
    print!("{}!\n", first_word(&s2));
}

fn fib(n : i32) -> i32 {
    match n {
        1|0 => n,
        _ => fib(n-1) + fib(n-2),
    }
}

fn first_word(s : &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}