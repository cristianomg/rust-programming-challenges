use std::collections::BTreeMap;
fn main(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    let mut results: BTreeMap<i32, i32> = BTreeMap::new();

    let notes : [i32; 7] = [100, 50, 20, 10, 5, 2, 1];
    let mut acc = n;

    for note in notes.iter(){
        let (quantity, rest) = decompose(acc, *note);

        results.insert(*note, quantity);
        
        acc = rest;
    }
    println!("{}", n);
    for (note, quantity) in results.iter().rev(){
        println!("{} nota(s) de R$ {},00", quantity, note);
    }

}

fn decompose(mut value: i32, note: i32) -> (i32, i32){
    let mut qtd = 0;
    loop{
        if value < note{
            break;
        }
        value -= note;
        qtd += 1;

    }
    return (qtd,value);
}