/*
This file contains a very simple example for using gradient descent.
We want to find the best y-intercept for a linear function with a given slope (here 0.64)
so that it approximately matches a given batch of points
*/

pub fn linear_intercept_forward(x: f64, p: f64) -> f64 {
	x * 0.64 + p
}

pub fn linear_intercept_derivative(x: f64, p: f64) -> f64 {
	1.0
}


/*
The same example, but this time the intercept is fixed to 2 and we want to find the best slope
*/

pub fn linear_slope_forward(x: f64, p: f64) -> f64 {
    p * x + 2
}

pub fn linear_slope_derivative(x: f64, p: f64) -> f64 {
    x
}

// Next up I should implement optimizing both the slope and y-intercept at once
