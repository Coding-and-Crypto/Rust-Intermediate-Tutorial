use std::collections::HashMap;

#[macro_export]
macro_rules! make_map {
    ( $k:expr, $v:expr ) => {
        {
            println!("Key: {}", $k);
            println!("Value: {}", $v);
            let mut map = HashMap::new();
            map.insert($k, $v);
            map
        }
    };
}

#[macro_export]
macro_rules! make_map_2 {
    ( $( ( $k:expr, $v:expr ) ),* ) => {
        {
            let mut map = HashMap::new();
            $(
                println!("Key: {}", $k);
                println!("Value: {}", $v);
                map.insert($k, $v);
            )*
            map
        }
    };
}


fn main() {
    let int_map: HashMap<i32, i32> = make_map!(1, 3);
    println!("{:#?}", int_map);
    let str_map: HashMap<&str, &str> = make_map!("green", "go");
    println!("{:#?}", str_map);

    let int_map_2: HashMap<i32, i32> = make_map_2![
        (1, 3),
        (2, -1),
        (3, 5)
    ];
    println!("{:#?}", int_map_2);
    let str_map_2: HashMap<&str, &str> = make_map_2![
        ("green", "go"),
        ("yellow", "slow"),
        ("red", "stop")
    ];
    println!("{:#?}", str_map_2);
}