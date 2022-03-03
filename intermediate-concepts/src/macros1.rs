
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

// Make this macro available elsewhere
#[macro_export]
// Establish the name of the macro
macro_rules! vec {
    // Match any Rust expression [$x:expr]
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

{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec
}