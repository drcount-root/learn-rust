# Things to know about rust & about variables

0. 💡 `fn main()`: This is the main function, and it is the entry point of any Rust program. `main` must be defined only once in the value namespace of the module.

1. 💡 A variable can be used only if it has been initialized.
2. 💡 A variable which is uninitialized but used, gives ERROR !
3. 💡 A variable which is uninitialized but also unused, only a Warning !

4. 💡 Declare a variable x of type i32 (32-bit signed integer) and assign the value 5 to it. example: `let x: i32 = 5;`

5. 💡 `assert_eq!` macro is used to check if the value of x is equal to 5 as per the example: `assert_eq!(x, 5);`

6. 💡 `print!`: text is printed to the console
7. 💡 same as `print!` but a newline is appended
8. 💡 `print!` is a macro that prints text to the console

9. 💡 `let mut x: i32 = 1;`: This line declares a mutable variable named x of type i32 (32-bit signed integer) and initializes it with the value 1. The mut keyword indicates that the variable can be changed.

10. 💡 `x += 2;` This line increments the value of x by 2. If the value of x is 1, then after this line executes, the value of x becomes 3.
