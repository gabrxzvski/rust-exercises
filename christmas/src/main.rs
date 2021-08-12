fn main() {
    let string_count = [
        "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
        "eleventh", "twelfth",
    ];

    let string_phrase = [
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    println!(
        "On the first day of Christmas,\nmy true love sent to me\nA partridge in a pear tree.\n"
    );

    let mut counter = 0;

    for item in string_count {
        println!("On the {} day of Christmas,\nmy true love sent to me", item);
        counter += 1;
        for index in 0..counter {
            println!("{}", string_phrase[index]);
        }
        println!("And a partridge in a pear tree.\n");
    }
}
