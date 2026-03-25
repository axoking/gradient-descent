/// Lineare Beispielfunktion. p ist hier der y-Achsenabschnitt
pub fn linear_example_forward(x: f64, p: f64) -> f64 {
	x * 0.64 + p
}

/// Ableitung der linearen Beispielfunktion, abhängig vom Parameter
pub fn linear_example_derivative(_: f64, _: f64) -> f64 {
	1.0
}
