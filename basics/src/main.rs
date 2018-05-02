#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


fn main() {
    let x:i32 = 123;
    let y:u32 = 1_123;
    let z:f64 = 0.3434243;
    let a:&str = "New string";
    let b = "It's b string";

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("Hello, world!");
    println!("x={:#?} y={} z={}",x,y,z);
    println!("x={x} y={y} z={z}",x=x,y=y,z=z);
    println!("string = {}", a);
    println!("string = {}", b);
    println!("{:#?}", peter);
}
