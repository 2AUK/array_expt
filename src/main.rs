mod custom_grid;

use crate::custom_grid::GridArray;

fn main() {
    let mut test = GridArray::<f64>::new();
    println!("Initialised GridArray\n{:?}", test);
    test.push(1.01);
    test.push(2.45);
    println!{"Pushed 2 values\n{:?}", test};
    let last_val = test.pop().unwrap();
    println!("Popped value: {}\nGridArray: {:?}", last_val, test);
    let mut other_test = GridArray::<f64>::new();
    other_test.push(2.0);
    other_test.push(3.0);
    test.push(12.015);
    test[1] = 2.0;
    println!("Adding {:?} and {:?}", test, other_test);
    println!("{:?}", test + other_test);
}
