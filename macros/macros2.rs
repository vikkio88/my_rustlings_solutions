// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// oh man Macros are scoped by position :(
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
