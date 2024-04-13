use super::{RelationshipPair, Solution, Violation};

pub(crate) fn get_violations(
    solution: &Solution,
    restrictions: &[RelationshipPair],
) -> Vec<Violation> {
    restrictions
        .iter()
        .filter(|restriction| {
            solution[&restriction.first_student_id] == solution[&restriction.second_student_id]
        })
        .cloned()
        .map(|restriction| Violation {
            relationship_pair: restriction.clone(),
            group: solution[&restriction.first_student_id].clone(),
        })
        .collect()
}
