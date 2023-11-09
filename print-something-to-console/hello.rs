fn main() {
    // print!: text is printed to the console
    // println!: same as print! but a newline is appended
    // print! is a macro that prints text to the console
    println!("This text is printed using rust lang");
    // output - This text is printed using rust lang

    // In general, the `{}` will be automatically replaced with any arguments. These will be stringified.
    // Positional arguments can be used. Specifying an integer inside `{}` determines which additional argument will be replaced. Arguments start at 0 immediately after the format string.
    println!("Hey there, {} here", "Snowden");
    // output - Hey there, Snowden here

    println!("Hi {0} how are you? - I'm fine {1}", "Snowden", "Subham");
    // output - Hi Snowden how are you? - I'm fine Subham

    println!("January have {} days in total", 31);
    // output - January have 31 days in total

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    // output - the quick brown fox jumps over the lazy dog

    println!("{number} is a number", number = 10);
    // output - 10 is a number

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{0} < {1}", number, width); // 1 < 5
}
