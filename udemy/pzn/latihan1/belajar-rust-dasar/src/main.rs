fn main() {
    println!("Hello, world!");
    println!("Hello, yasir!");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Muhamad Yasir Noval";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Muhamad Yasir Noval";
    println!("Hello {}", name);

    name = "Budi Nugraha";
    println!("Hello {}", name);
}

#[test]
fn test_static_typing() {
    let mut name = "Muhamad Yasir Noval";
    println!("Hello {}", name);

    name = "10";
    println!("Hello {}", name);
}

#[test]
fn test_shadowing() {
    let name = "Muhamad Yasir Noval";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

/*
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
*/
#[test]
fn test_comment() {
    // ini komentar
    println!("Hello"); // ini komentar lagi
}

#[test]
fn test_explicit() {
    let age: i32 = 20;
    println!("{}", age);
}

#[test]
fn test_number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn test_number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn test_numeric_operator() {
    let a = 10;
    let b = 10;

    let c = a * b;
    println!("{}", c);

    let d = a / b;
    println!("{}", d);

    let e = a + b;
    println!("{}", e);
}

#[test]
fn test_augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn test_boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn test_comparison() {
    let a = 10;
    let b = 20;

    let result: bool = a > b;
    println!("{}", result);
}

#[test]
fn test_boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai_akhir: bool = nilai_akhir >= 75;

    let lulus: bool = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);
}

#[test]
fn test_char_type() {
    let char1: char = 'a';
    let char2: char = 'b';

    println!("{} {}", char1, char2);
}

#[test]
fn test_tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a, b, c) = data;

    println!("{} {} {}", a, b, c);

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result: () = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}

#[test]
fn test_array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);
}

#[test]
fn test_two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);

    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);

    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2]);
}

const MAXIMUM: i32 = 100;

#[test]
fn test_constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}