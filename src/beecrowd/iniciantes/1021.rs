
use std::io;
use std::collections::BTreeMap;

const NOTES: [f32; 6] = [100_f32, 50_f32, 20_f32, 10_f32, 5_f32, 2_f32];

const COINS: [f32; 6] = [1.0_f32, 0.50_f32, 0.25_f32, 0.10_f32, 0.05_f32, 0.01_f32];

fn main (){
    let mut imput = String::new();
    io::stdin().read_line(&mut imput).unwrap();

    let n: f32 = imput.trim().parse().unwrap();

    let mut result_notes: BTreeMap<i32, i32> = BTreeMap::new();
    let mut result_coins: BTreeMap<usize, i32> = BTreeMap::new();

    let mut acc = n;
    
    acc = decompose_notes(acc, &mut result_notes);
    acc = decompose_coins(acc, &mut result_coins);
    
    show_notes(result_notes);
    show_coins(result_coins, COINS);
}


fn decompose_notes(mut acc: f32, result_notes: &mut BTreeMap<i32, i32>) -> f32{
    for note in NOTES.iter() {
        let (value, quantity) = decompose(acc, *note);
        
        result_notes.insert(*note as i32, quantity);

        acc = value;
    }
    return acc;
}

fn decompose_coins(mut acc: f32, result_coins: &mut BTreeMap<usize,i32>) -> f32{

    for coin_index in 0..COINS.len() {
        let coin: f32 = COINS[coin_index];
        let (value, quantity) = decompose(acc, coin);
        result_coins.insert(coin_index, quantity);

        acc = value;
    }

    return acc;
}

fn decompose(mut value: f32, note: f32) -> (f32, i32) {
    let mut quantity = 0;
    loop {
        if value < note {
            break;
        }

        value -= note;
        quantity += 1;
    }

    return (value, quantity);
}
fn show_notes(result_notes: BTreeMap<i32, i32>){
    println!("NOTAS:");
    for (note, quantity) in result_notes.iter().rev(){
        println!("{} nota(s) de R$ {},00", quantity, note);
    }
}
fn show_coins(result_coins: BTreeMap<usize, i32>, coins_var: [f32; 6]){
    println!("MOEDAS:");
    for (coin_index, quantity) in result_coins.iter(){
        println!("{} moeda(s) de R$ {:.2}", quantity, coins_var[*coin_index]);
    }
}