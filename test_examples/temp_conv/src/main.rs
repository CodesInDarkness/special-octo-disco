use std::io; // standard library the input/output library

fn main() {
    println!("What Fehrenheit degree to you want to know in Celsius?");

    let mut feh = String::new();
    io::stdin()
        .read_line(&mut feh)
        .expect("Failed to read line");

    let feh: u32 = feh.trim().parse().expect("Please enter a number!");

    let cel = (feh - 32) * 5 / 9;

    println!("{feh} Fehrenheit converts to {cel} Celsius");
}
