fn main() {
    println!("result: {}", interproduct(120, 100, 248));
    let a: i64  = 120;
    let b: i64  = 100;
    let c: i64  = 248;
    println!("result: {}", (a * b).saturating_add(b * c).saturating_add(c * a));
}
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}
