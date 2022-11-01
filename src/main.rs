fn main() {
    reduction("4/6");
}

fn reduction(arg: &str) {
    let v: Vec<&str> = arg.split('/').collect();
    let a: u32 = v[0].parse().unwrap();
    let b: u32 = v[1].parse().unwrap();
    let greatest_common_divisor: u32 = gcd(a,b);
    let numerator = 4 / greatest_common_divisor;
    let denominator = 6 / greatest_common_divisor;
    println!("{}/{}", numerator, denominator);
}

fn gcd(num_1: u32, num_2: u32) -> u32 {
    if num_2 == 0 {
        num_1
    } else {
        gcd(num_2, num_1 % num_2)
    }
}