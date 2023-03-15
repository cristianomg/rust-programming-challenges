
use std::io;
use std::collections::BTreeMap;

const NOTES: [f64; 6] = [100_f64, 50_f64, 20_f64, 10_f64, 5_f64, 2_f64];

const COINS: [f64; 6] = [1.0_f64, 0.50_f64, 0.25_f64, 0.10_f64, 0.05_f64, 0.01_f64];

fn main (){
    let mut imput = String::new();
    io::stdin().read_line(&mut imput).unwrap();

    let n: f64 = imput.trim().parse().unwrap();

    let mut result_notes: BTreeMap<i32, i32> = BTreeMap::new();
    let mut result_coins: BTreeMap<usize, i32> = BTreeMap::new();

    let mut acc = n;
    
    acc = decompose_notes(acc, &mut result_notes);
    _ = decompose_coins(acc, &mut result_coins);
    
    show_notes(result_notes);
    show_coins(result_coins, COINS);
}


fn decompose_notes(mut acc: f64, result_notes: &mut BTreeMap<i32, i32>) -> f64{
    for note in NOTES.iter() {
        let (value, quantity) = decompose(acc, *note);
        
        result_notes.insert(*note as i32, quantity);

        acc = value;
    }
    return acc;
}

fn decompose_coins(mut acc: f64, result_coins: &mut BTreeMap<usize,i32>) -> f64{

    for coin_index in 0..COINS.len() {
        let coin: f64 = COINS[coin_index];
        let (value, quantity) = decompose(acc, coin);
        result_coins.insert(coin_index, quantity);

        acc = value;
    }

    return acc;
}

fn decompose(mut value: f64, note: f64) -> (f64, i32) {
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
fn show_coins(result_coins: BTreeMap<usize, i32>, coins_var: [f64; 6]){
    println!("MOEDAS:");
    for (coin_index, quantity) in result_coins.iter(){
        println!("{} moeda(s) de R$ {:.2}", quantity, coins_var[*coin_index]);
    }
}