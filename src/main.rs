#[allow(dead_code)]
pub struct Person {
    name: String,
    id: i32,
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.width
    }
}

fn area(r: &Rect) -> u32 {
    r.width * r.height
}

fn main() {
    let rect1 = Rect { width: 30, height: 50 };
    let rect2 = Rect { width: 10, height: 40 };
    let rect3 = Rect { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    /*
    // Struct with function and method calls
    let r = Rect { width: 30, height: 50 };
    println!("The area is {}", area(&r));
    println!("The area is {}", r.area());
    println!("{:?}", r);
    */
    /*
    // Creation of a Struct with constuctor
    let p = Person {
        name: String::from("Jacob"),
        id: 13425,
    };
    println!("name: {} id: {}", p.name, p.id);
    */
    /*
    // This is String Slicing
    let s1 = String::from("Hello world");
    let s2 = "World Hello";

    print!("{}, ", first_word(&s1));
    print!("{}!\n", first_word(&s2));
    */

    // Fibonacci sequence
    //println!("{}: {}", 20, fib(20));
}

#[allow(dead_code)]
fn fib(n: i32) -> i32 {
    match n {
        1 | 0 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[allow(dead_code)]
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}