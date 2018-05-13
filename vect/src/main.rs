fn main() {
    let _v: Vec<i32> = vec![1, 2, 3];

    let mut v_mut = Vec::new();

    v_mut.push(5);
    v_mut.push(6);
    v_mut.push(7);
    v_mut.push(8);

    let _does_not_exist = &_v[1];
    let _does_not_exist = _v.get(1);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", *i);
    }

    println!("v:{}", &v_mut[2]);
    println!("Hello, world!");

    enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
