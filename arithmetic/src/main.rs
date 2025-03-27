fn main() {
    println!("result: {}", interproduct(120, 100, 248));
    let a: i64  = 120;
    let b: i64  = 100;
    let c: i64  = 248;
    println!("result: {}", (a * b).saturating_add(b * c).saturating_add(c * a));

    let max = u8::MAX; // 255
    let wrapped = max.wrapping_add(1); // Wraps around to 0

    println!("255 + 1 with wrapping_add: {}", wrapped); // Output: 0

    let checked = max.checked_add(1);

    println!("255 + 1 with checked_add: {:?}", checked); // Output: None
    println!("255 + 1 with checked_add: {:?}", checked.unwrap_or(u8::MAX)); // Output: None

}
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}
