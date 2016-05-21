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
    let generations = 1000;
    let mutation_rate = 0.2_f32;
    let mut parent_population = initialise_population(true);

    for gen in 0..generations {
        for i in 0..parent_population.len() {
            let mut candidate_solution = Solution {
                parameters: parent_population[i].parameters,
                objectives: parent_population[i].objectives,
            };

            candidate_solution.parameters = gaussian_mutation(parent_population[i].parameters,
                                                              mutation_rate);
            candidate_solution.objectives = zdt1(candidate_solution.parameters);

            if dominates(candidate_solution.objectives,
                         parent_population[i].objectives) {
                parent_population[i] = candidate_solution;
            }
        }
    }

    for i in 0..parent_population.len() {
        println!("{:?}", parent_population[i].objectives);
    }
}

fn dominates(offspring_objectives: [f32; 2], parent_objectives: [f32; 2]) -> bool {
    if (offspring_objectives[0] <= parent_objectives[0]) &&
       (offspring_objectives[1] <= parent_objectives[1]) {
        return true;
    }
    return false;
}
