/*
This file contains a very simple example for using gradient descent.
We want to find the best y-intercept for a linear function with a given slope (here 0.64)
so that it approximately matches a given batch of points
*/

pub fn linear_example_forward(x: f64, p: f64) -> f64 {
	x * 0.64 + p
}

pub fn linear_example_derivative(_: f64, _: f64) -> f64 {
	1.0
}
