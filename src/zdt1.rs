/// ZDT1 bi-objective test function
///
/// Evaluates solution parameters using the ZDT1 [1] synthetic test
/// test function to produce two objective values.
///
/// [1 ]E. Zitzler, K. Deb, and L. Thiele. Comparison of Multiobjective
/// Evolutionary Algorithms: Empirical Results. Evolutionary
/// Computation, 8(2):173-195, 2000
pub fn zdt1(parameters: [f32; 30]) -> [f32; 2] {

    // objective function 1
    let f1 = parameters[0];
    // objective function 2
    let mut g = 1_f32;

    // g(x)
    for i in 1..parameters.len() {
        g = g + ((9_f32 / (parameters.len() as f32 - 1_f32)) * parameters[i]);
    }

    // h(f1, x)
    let h = 1_f32 - (f1 / g).sqrt();

    // f2(x)
    let f2 = g * h;

    return [f1, f2];
}
