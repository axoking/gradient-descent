pub mod linear_example;

/// A training batch is a list of tuples, first value is input and second is the desired output
pub type TrainingBatch = Vec<(f64, f64)>;
/// A param function is a function that depends on a second parameter besides x ( f(x,p) ).
pub type ParamFunction = fn(f64, f64) -> f64;

/// Calculates the error of a given training batch for a function that relies on one parameter
pub fn cost(f: &ParamFunction, batch: &TrainingBatch, p: f64) -> f64 {
	batch.iter()
		.map(|(a, b)| (b-f(*a, p)) * (b-f(*a, p)))
		.sum()
}

/// Derivative of the cost function (needs the derivative of the evaluation function)
pub fn cost_derivative(f: &ParamFunction, df: &ParamFunction, batch: &TrainingBatch, p: f64) -> f64 {
	batch.iter()
		.map(|(a, b)| -2.0*(b - f(*a, p)) * df(*a, p))
		.sum()
}

/// Performs gradient descent algorithm with the given function and its derivative on the given batch, and returns the optimal value for the parameter p
/// Returns Err if the maximum steps number is exceeded
pub fn gradient_descent(f: &ParamFunction, df: &ParamFunction, batch: &TrainingBatch, initial_p: f64, learning_rate: f64, min_adjust: f64, max_steps: usize)
-> Result<f64, &'static str> {
	let mut p = initial_p;
	for _ in 0..max_steps {
		let adjust = cost_derivative(f, df, &batch, p) * learning_rate;
		if adjust.abs() < min_adjust {
			return Ok(p)
		}
		p -= adjust;
	}

	Err("Maximum steps exceeded!")
}
