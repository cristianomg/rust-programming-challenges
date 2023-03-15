fn main(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let hours: i32 = input.trim().parse().unwrap();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let velocity: i32 = input.trim().parse().unwrap();

    let distance = hours * velocity;

    let consumed_fuel = distance as f64 / 12.0;

    println!("{:.3}", consumed_fuel);
}