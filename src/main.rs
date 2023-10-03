fn main() {
    let time = chrono::Local::now();
    println!("Date: {}", time.date_naive());
    println!("Time: {}", time);
}
