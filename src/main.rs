mod custom_grid;

use crate::custom_grid::GridArray;
use ndarray::prelude::*;
use itertools_num::linspace;

fn main() {
    println!("CUSTOM GRIDARRAY EXPT");
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
    println!("\nNDARRAY EXPT");
    let input_vec: Vec<f64> = linspace::<f64>(0., 20.48, 16*3*3).collect();
    println!("{:?}", input_vec);
    let mut input_array = Array3::<f64>::zeros((16, 3, 3));
    let test_ndarray = Array::from_shape_vec((16, 3, 3), input_vec).unwrap();
    let ndarray2 = test_ndarray.clone();
    let ndarray3 = test_ndarray.clone();
    ndarray::Zip::from(input_array.axis_iter_mut(Axis(0)))
        .and(test_ndarray.axis_iter(Axis(0)))
        .and(ndarray2.axis_iter(Axis(0)))
        .and(ndarray3.axis_iter(Axis(0)))
        .for_each(
            |inp, a, b, c|
            inp = a.view()
        )
}
