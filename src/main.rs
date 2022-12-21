mod custom_grid;

use crate::custom_grid::GridArray;
use itertools_num::linspace;
use ndarray::prelude::*;

fn main() {
    println!("CUSTOM GRIDARRAY EXPT");
    let mut test = GridArray::<f64>::new();
    println!("Initialised GridArray\n{:?}", test);
    test.push(1.01);
    test.push(2.45);
    println! {"Pushed 2 values\n{:?}", test};
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
    let input_vec: Vec<f64> = itertools_num::linspace::<f64>(0., 20.48, 16 * 3 * 3).collect();
    let mut input_array = Array3::<f64>::zeros((16, 3, 3));
    let test_ndarray = Array::from_shape_vec((16, 3, 3), input_vec).unwrap();
    let test_ndarray2 = test_ndarray.clone();
    let test_ndarray3 = test_ndarray.clone();

    println!("{}", test_ndarray);
    println!("{}", input_array);
    ndarray::Zip::from(input_array.axis_iter_mut(Axis(0)))
        .and(test_ndarray.axis_iter(Axis(0)))
        .and(test_ndarray2.axis_iter(Axis(0)))
        .and(test_ndarray3.axis_iter(Axis(0)))
        .for_each(|mut inp, a, b, c|{
            // Want to perform a matrix multiplication
            // A * B * C where all matrices are NxN matrices
            // and save the result in inp - preallocated array
            let mat = (a.dot(&b)).dot(&c);
            inp.assign(&mat);
        }

        );
    println!("{}", input_array);
    /*
    let input_array = ndarray::Zip::from(test_ndarray.axis_iter(Axis(0)))
    .and(test_ndarray2.axis_iter(Axis(0)))
    .and(test_ndarray3.axis_iter(Axis(0)))
    .map_assign_into(input_array.view_mut(), |a, b, c| a.dot(&b).dot(&c));
     */
}
