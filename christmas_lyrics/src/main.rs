fn main() {
    let days = [
        "first", "second", "third", "fourth",
        "fifth", "sixth", "seventh","eighth",
        "ninth", "tenth", "eleventh", "twelfth"
    ];

    let lines = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
        "ba-dum-bum-bum"
    ];
    for day in 0..12 {
        print!("On {} day of Christmas", days[day]);
        if day == 8 {println!("(mee, mee, mee, mee, mee)");}
        else {println!(", my true love gave to me");}
        let mut count = 0;
        for line in 0..(day + 1) {
            if count > 3   && count % 3 == 0 {print!("\n");}
            if count == 3 {print!("\n");}
            count += 1;
            let to_print = lines[day - line];
            if line == day && day != 0 { print!(" and {}", lines[0].to_lowercase()); }
            else {
                print!("{}", to_print);
                if line != day { print!(",");}
            }
        }
        print!("\n\n");
    }
}
