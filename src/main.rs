use std::ops::Add;

#[derive(Debug)]

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    // println!("Hello, world!");
    // println!("{:?}", Point { x: 1, y: 2 } + Point { x: 2, y: 3 });

    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 'a实际表示的是生命周期更短的那一个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 被引用的对象的生命周期需 >= 引用对象
struct ImportantExcerpt<'a> {
    part: &'a str
}
