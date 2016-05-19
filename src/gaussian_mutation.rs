/// Performs gaussian mutation on solution parameters
///
/// Performs gaussian mutation on real-valued solution parameters,
/// currently hard-coded for 30 problem variables for the ZDT1
/// synthetic test function.
extern crate rand;
use rand::{random, thread_rng, Rng};

// bounds hard-coded for ZDT1 i.e. (0,1)
pub fn gaussian_mutation(mut parameters: [f32; 30], mutation_rate: f32) -> [f32; 30] {
    let std = 1_f32 - 0_f32 / 10_f32;
    for parameter in &mut parameters[..] {
        if random::<f32>() <= mutation_rate {
            let mutation = thread_rng().gen_range(-1.0f32, 1.0f32) * std;
            *parameter = *parameter + mutation;

            // Enforce bounds
            *parameter = f32::max(*parameter, 0_f32);
            *parameter = f32::min(*parameter, 1_f32);
        }
    }
    return parameters;
}
