use std::io;

fn main() {
    println!("Enter index of the fibbonaci number you want to get: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: u32 = input.trim().parse().expect("Please enter a number!");

    println!(
        "The fibbonaci number at index {} is {}",
        index,
        fibbonaci(index)
    );
}

fn fibbonaci(index: u32) -> u32 {
    if (index == 0) || (index == 1) {
        return index;
    }
    return fibbonaci(index - 1) + fibbonaci(index - 2);
}
