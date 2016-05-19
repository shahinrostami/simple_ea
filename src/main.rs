extern crate rand;
mod zdt1;
mod gaussian_mutation;
use rand::{random, Closed01};
use zdt1::zdt1;
use gaussian_mutation::gaussian_mutation;

#[derive(Copy, Clone, Debug)]
struct Solution {
    parameters: [f32; 30],
    objectives: [f32; 2],
}

fn initialise_population(initial_population: bool) -> [Solution; 100] {
    let mut population = [Solution {
        parameters: [0_f32; 30],
        objectives: [0_f32; 2],
    }; 100];

    if initial_population {
        for solution in &mut population[..] {
            for parameter in &mut solution.parameters[..] {
                *parameter = random::<Closed01<f32>>().0;
            }

            solution.objectives = zdt1(solution.parameters);
        }
    }

    return population;
}

fn main() {
    let generations = 10;
    let mutation_rate = 0.2_f32;
    let mut parent_population = initialise_population(true);
    let mut offspring_population = initialise_population(false);

    for gen in 0..generations {
        for solution in &mut parent_population[..] {
            solution.parameters = gaussian_mutation(solution.parameters, mutation_rate);
            solution.objectives = zdt1(solution.parameters);
        }
    }
}
