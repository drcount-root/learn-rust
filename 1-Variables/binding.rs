// Learnings

// `fn main()`: This is the main function, and it is the entry point of any Rust program. `main` must be defined only once in the value namespace of the module.

// 💡 A variable can be used only if it has been initialized.

// 💡 A variable which is uninitialized but used, gives ERROR !

// 💡 A variable which is uninitialized but also unused, only a Warning !

// 💡 Declare a variable x of type i32 (32-bit signed integer) and assign the value 5 to it. example: let x: i32 = 5;

// 💡 assert_eq! macro is used to check if the value of x is equal to 5 as per the example: assert_eq!(x, 5);

// 💡 print!: text is printed to the console
// 💡 println!: same as print! but a newline is appended
// 💡 print! is a macro that prints text to the console

// ------------------------------------------------------

// 🌟🌟🌟🌟🌟 Fix the error - 1 🌟🌟🌟🌟🌟

// fn main() {
//     let x: i32;
//     let y: i32;

//     assert_eq!(x, 5);
//     println!("Success!");
// }

// ------------------------------------------------------

// it gives a warning: unused variable: `y`.
// but compiled with that warning
// so i removed the variable y declaration

// Declare a variable x of type i32 (32-bit signed integer) and assign the value 5 to it.

// Use the assert_eq! macro to check if the value of x is equal to 5.

// If the assertion passes (i.e., x is equal to 5), print "Success!" to the console.

fn one_func() {
    let x: i32 = 5;

    assert_eq!(x, 5);

    println!("✅ Success! one");
}

// 🌟🌟🌟🌟🌟 Fix the error - 2 🌟🌟🌟🌟🌟

// fn main() {
//     let __ __ = 1;
//     __ += 2;

//     assert_eq!(x, 3);
//     println!("Success!");
// }

// ------------------------------------------------------

// gave the error - cannot find value `x` in this scope

// let mut x: i32 = 1;: This line declares a mutable variable named x of type i32 (32-bit signed integer) and initializes it with the value 1. The mut keyword indicates that the variable can be changed.

// x += 2;: This line increments the value of x by 2. After this line executes, the value of x becomes 3.

fn two_func() {
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("✅ Success! Two");
}

fn analyzer_function() {
    one_func();
    two_func();
}

// ------------------------------------------------------

// Advance concept : Error handling

fn one_function() -> Result<(), &'static str> {
    let x: i32 = 5;

    if x != 5 {
        return Err("Error in one_function");
    }

    println!("✅ Success! one");
    Ok(())
}

fn two_function() -> Result<(), &'static str> {
    let mut x: i32 = 1;
    x += 2;

    if x != 3 {
        return Err("Error in two_function");
    }

    println!("✅ Success! Two");
    Ok(())
}

fn main() {
    analyzer_function();

    if let Err(err) = one_function() {
        println!("❌ Error in one_function: {}", err);
    }

    if let Err(err) = two_function() {
        println!("❌ Error in two_function: {}", err);
    }
}
