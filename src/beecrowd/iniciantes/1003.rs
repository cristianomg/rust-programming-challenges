fn main(){
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let a: i32 = input.trim().parse().unwrap();

    input.clear();

    std::io::stdin().read_line(&mut input).unwrap();

    let b: i32 = input.trim().parse().unwrap();

    println!("SOMA = {}", a + b);
}