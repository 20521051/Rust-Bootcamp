fn main() {
    println!("Hello, Rust Bootcamp by VBI Academy!");
    exercise1();
    exercise2();
    exercise3();
}
// Exercise 1
// Make it compile
fn exercise1() {
    // Use as many approaches as you can to make it work
    // solution 1
    let x = String::from("hello, world");
    let y = x;
    let z = y;
    println!("z: {}", z);
    // solution 2
    // let x = String::from("hello, world");
    // let y = &x;
    // let z = &x;
    // println!("z: {}", z);
}

// Exercise 2
// It don't have any error
// Make it compile
// Don't modify code in exercise2 function!
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
// Only modify the code below!
fn take_ownership(s: String) -> String {
    //println!("{}", s);
    s
}
// Exercise 3
// Make it compile
// Don't care about logic
fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();

    let mut additions: Vec<usize> = vec![0];

    println!("{:?}", values_number);

    while additions.len() > 0 {
        let mut addition: f64 = 0.0;

        // Sumar valores en additions
        for element_index in &additions {
            let addition_aux = values[*element_index];
            addition = addition_aux + addition;
        }
        println!("addition {}", addition);
        additions.clear();
    }
}

// // Exercise 4
// // Make it compile
// fn exercise4(value: u32) -> &'static str {
//     let str_value = value.to_string(); // Convert u32 to String
//     let str_ref: &str = &str_value; // Obtain a reference to the String
//     str_ref // Return the reference to the String
// }

// // Exercise 5
// // Make it compile
// use std::collections::HashMap;
// fn exercise5() {
//     let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

//     let key = 3;

//     let res = match my_map.get(&key) {
//         Some(child) => child,
//         None => {
//             let value = "3.0".to_string();
//             my_map.insert(key, value);
//             &value // HERE IT FAILS
//         }
//     };

//     println!("{}", res);
// }
