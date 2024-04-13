use tracing::{error, trace};

use super::{RelationshipPair, Solution};

#[tracing::instrument]
pub(crate) fn objective(
    solution: &Solution,
    num_groups: usize,
    restrictions: &[RelationshipPair],
) -> Result<f64, anyhow::Error> {
    let mut num_violations = 0f64;
    for restriction in restrictions {
        if solution.get(&restriction.first_student_id).is_none()
            || solution.get(&restriction.second_student_id).is_none()
        {
            error!(
                first_student_id = restriction.first_student_id.0,
                second_student_id = restriction.second_student_id.0,
                "Invalid student id in relationship pair",
            );
            return Err(anyhow::anyhow!("Invalid student id in relationship pair"));
        }
        if solution[&restriction.first_student_id] == solution[&restriction.second_student_id] {
            num_violations += 1.0;
        }
    }

    let mut group_sizes = vec![0; num_groups];
    for group in solution.values() {
        group_sizes[**group] += 1;
    }

    let mean_size = group_sizes.iter().map(|&size| size as f64).sum::<f64>() / num_groups as f64;
    let variance = group_sizes
        .iter()
        .map(|&size| (size as f64 - mean_size).powi(2))
        .sum::<f64>()
        / num_groups as f64;

    let score = variance + (2.0 * num_violations);
    trace!(
        "Objective function: variance: {}, violations: {}, score: {}",
        variance,
        num_violations,
        score
    );
    Ok(score)
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::annealing::{GroupId, StudentId};

    use super::*;

    fn generate_random_restriction_pairs(
        num_pairs: usize,
        num_students: usize,
    ) -> Vec<RelationshipPair> {
        let mut rng = rand::thread_rng();
        (0..num_pairs)
            .map(|_| {
                //generate two different student ids
                let first_student_id = rng.gen_range(1..num_students).into();
                let mut second_student_id = rng.gen_range(1..num_students).into();
                while first_student_id == second_student_id {
                    second_student_id = rng.gen_range(1..num_students).into();
                }

                RelationshipPair {
                    first_student_id,
                    second_student_id,
                }
            })
            .collect()
    }

    #[test]
    fn test_objective_function() {
        let mut solution = Solution::new();
        solution.insert(StudentId(1), GroupId(0));
        solution.insert(StudentId(2), GroupId(0));
        solution.insert(StudentId(3), GroupId(1));
        solution.insert(StudentId(4), GroupId(1));
        solution.insert(StudentId(5), GroupId(2));
        solution.insert(StudentId(6), GroupId(2));

        let restrictions = vec![
            RelationshipPair {
                first_student_id: 1.into(),
                second_student_id: 2.into(),
            },
            RelationshipPair {
                first_student_id: 3.into(),
                second_student_id: 4.into(),
            },
            RelationshipPair {
                first_student_id: 5.into(),
                second_student_id: 6.into(),
            },
        ];

        let result = objective(&solution, 3, &restrictions).unwrap();
        assert_eq!(result, 6.0);
    }
}
