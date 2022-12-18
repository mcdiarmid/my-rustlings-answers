// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.


macro_rules! my_macro {  // Macros must be defined prior to useage in same file
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro!();
}