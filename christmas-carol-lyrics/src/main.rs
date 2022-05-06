fn main() {
    let lines_by_day = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 1..13 {
        println!("On the {} day of Christmas my true love gave to me:", day);
        for line in lines_by_day.iter().take(day).rev() {
            println!("{}", line);
        }
        println!("");
    }
}
