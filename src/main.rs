use gradient_descent::{*, linear_example::*};

fn main() {
    let batch = vec![(1.0, 3.0), (3.0, 4.0), (8.0, 6.0)];
    let p = gradient_descent(
        &(linear_example_forward as ParamFunction),
        &(linear_example_derivative as ParamFunction),
        &batch,
        0.0,
        0.3,
        0.001,
        1000
    ).unwrap();

    println!("{p}");
    println!("Cost: {}", cost(&(linear_example_forward as ParamFunction), &batch, p));
}
