fn main (){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let distance : i32 = input.trim().parse().unwrap();

    let time = distance * 2;

    println!("{} minutos", time);
}