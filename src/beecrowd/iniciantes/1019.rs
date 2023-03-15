const HOURS: i32 = 60 * 60;
const MINUTES: i32 = 60;

fn main(){
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    let hours = get_hours(n);
    let minutes = get_minutes(n - (hours * HOURS));
    let seconds = n - (hours * HOURS) - (minutes * MINUTES);

    println!("{}:{}:{}", hours, minutes, seconds);

}

fn get_hours(value : i32) -> i32 {
    if value >= HOURS {
        return value / HOURS;
    }
    return 0;
}

fn get_minutes(value : i32) -> i32 {
    if value >= MINUTES {
        return value / MINUTES;
    }

    return 0;
}