use gradient_descent::{linear_example::*, *};

fn main() {
    let batch = vec![(1.0, 3.0), (3.0, 4.0), (8.0, 6.0)];

    let p = gradient_descent(
        &(linear_intercept_forward as ParamFunction),
        &(linear_intercept_derivative as ParamFunction),
        &batch,
        0.0,
        0.3,
        0.001,
        5000,
    )
    .unwrap();
    println!("Best y-intercept for slope 0.64: {p}");
    println!(
        "Cost: {}\n",
        cost(&(linear_intercept_forward as ParamFunction), &batch, p)
    );

    let p = gradient_descent(
        &(linear_slope_forward as ParamFunction),
        &(linear_slope_derivative as ParamFunction),
        &batch,
        0.0,
        0.01, // slope optimiztation needs a smaller learning rate than intercept optimzation, otherwise it fails
        0.001,
        5000,
    )
    .unwrap();
    println!("Best slope for y-intercept 2: {p}");
    println!(
        "Cost: {}",
        cost(&(linear_slope_forward as ParamFunction), &batch, p)
    );
}
