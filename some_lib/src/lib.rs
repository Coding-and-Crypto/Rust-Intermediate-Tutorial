
pub mod some_lib_functions;

pub fn whats_up() {
    println!("What's up?");
}

// Macros

// Actually creates other code in program
// Does this at compile time
// Can do things functions can't

// Functions have to declare number/type of params
// Macros do not

// Macros are tougher to read

// This is what the vec! macro looks like:
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(($x));
            )*
            temp_vec
        }
    };
}

// Let's break it down
// Make this macro available elsewhere
#[macro_export]
// Establish the name of the macro [vec!]
macro_rules! vec {
    // Matches any Rust expression [$x:expr]
    // Allows one or more of them [,*]
    ( $( $x:expr ),* ) => {
        {
            // Create a temporary empty vector
            let mut temp_vec = Vec::new();
            // Generate this command for all expressions
            $(
                temp_vec.push(($x));
            )*
            temp_vec
        }
    };
}

// The resulting generated code:
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}