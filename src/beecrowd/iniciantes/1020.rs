const DAYS_IN_YEAR: i32 = 365;
const DAYS_IN_MONTH: i32 = 30;

fn main(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();


    let years = get_years(n);

    let month = get_months(n - (years * DAYS_IN_YEAR));

    let days = n - (years * DAYS_IN_YEAR) - (month * DAYS_IN_MONTH);

    println!("{} ano(s)", years);
    println!("{} mes(es)", month);
    println!("{} dia(s)", days);

}

fn get_years(value: i32) -> i32 {
    if value >= DAYS_IN_YEAR {
        return value / DAYS_IN_YEAR;
    }
    return 0;
}

fn get_months(value: i32) -> i32 {
    if value >= DAYS_IN_MONTH {
        return value / DAYS_IN_MONTH;
    }
    return 0;
}
