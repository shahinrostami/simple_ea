/// Returns true if the candidate dominates or equal the parent
pub fn dominates(offspring_objectives: [f32; 2], parent_objectives: [f32; 2]) -> bool {
    if (offspring_objectives[0] <= parent_objectives[0]) &&
       (offspring_objectives[1] <= parent_objectives[1]) {
        return true;
    }
    return false;
}
