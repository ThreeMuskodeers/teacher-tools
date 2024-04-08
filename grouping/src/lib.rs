mod group_move;
mod temperature;
use rand::Rng;
use std::{collections::HashMap, fmt::Display};

type StudentId = i32;
type GroupId = i32;

#[derive(Debug, Clone)]
pub struct Student {
    id: StudentId,
    first_name: String,
    last_name: String,
}

#[derive(Debug, Clone)]
pub struct RelationshipPair {
    first_student_id: StudentId,
    second_student_id: StudentId,
}

type Solution = HashMap<StudentId, GroupId>;

fn objective(solution: &Solution, num_groups: usize, restrictions: &[RelationshipPair]) -> f64 {
    let mut num_violations = 0f64;
    for restriction in restrictions {
        if solution[&restriction.first_student_id] == solution[&restriction.second_student_id] {
            num_violations += 1.0;
        }
    }

    let mut group_sizes = vec![0; num_groups];
    for &group in solution.values() {
        group_sizes[group as usize] += 1;
    }

    let mean_size = group_sizes.iter().map(|&size| size as f64).sum::<f64>() / num_groups as f64;
    let variance = group_sizes
        .iter()
        .map(|&size| (size as f64 - mean_size).powi(2))
        .sum::<f64>()
        / num_groups as f64;

    variance + (2.0 * num_violations)
}

pub fn simulated_annealing(
    students: &[Student],
    num_groups: usize,
    restrictions: &[RelationshipPair],
    max_temp: f64,
    min_temp: f64,
    steps: i32,
) -> anyhow::Result<(Solution, f64)> {
    let mut rng = rand::thread_rng();
    let mut solution: Solution = students
        .iter()
        .map(|student| (student.id, rng.gen_range(0..num_groups as GroupId)))
        .collect();

    let mut best_solution = solution.clone();
    let mut best_score = objective(&solution, num_groups, restrictions);

    for step in 0..steps {
        let temp = temperature::change_temp(max_temp, min_temp, steps, step)?;

        let new_solution = group_move::make_move(&solution, num_groups)?;
        let new_score = objective(&new_solution, num_groups, restrictions);

        if new_score < best_score || rng.gen::<f64>() < ((best_score - new_score) / temp).exp() {
            solution = new_solution;
            if new_score < best_score {
                best_solution = solution.clone();
                best_score = new_score;
            }
        }
    }

    Ok((best_solution, best_score))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_random_restriction_pairs(
        num_pairs: usize,
        num_students: usize,
    ) -> Vec<RelationshipPair> {
        let mut rng = rand::thread_rng();
        (0..num_pairs)
            .map(|_| {
                //generate two different student ids
                let first_student_id = rng.gen_range(1..num_students as StudentId);
                let mut second_student_id = rng.gen_range(1..num_students as StudentId);
                while first_student_id == second_student_id {
                    second_student_id = rng.gen_range(1..num_students as StudentId);
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
        solution.insert(1, 0);
        solution.insert(2, 0);
        solution.insert(3, 1);
        solution.insert(4, 1);
        solution.insert(5, 2);
        solution.insert(6, 2);

        let restrictions = vec![
            RelationshipPair {
                first_student_id: 1,
                second_student_id: 2,
            },
            RelationshipPair {
                first_student_id: 3,
                second_student_id: 4,
            },
            RelationshipPair {
                first_student_id: 5,
                second_student_id: 6,
            },
        ];

        let result = objective(&solution, 3, &restrictions);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_annealing() {
        let students = vec![
            Student {
                id: 1,
                first_name: "Alice".to_string(),
                last_name: "Smith".to_string(),
            },
            Student {
                id: 2,
                first_name: "Bob".to_string(),
                last_name: "Jones".to_string(),
            },
            Student {
                id: 3,
                first_name: "Charlie".to_string(),
                last_name: "Brown".to_string(),
            },
            Student {
                id: 4,
                first_name: "David".to_string(),
                last_name: "Lee".to_string(),
            },
            Student {
                id: 5,
                first_name: "Eve".to_string(),
                last_name: "Wong".to_string(),
            },
            Student {
                id: 6,
                first_name: "Frank".to_string(),
                last_name: "Chan".to_string(),
            },
            Student {
                id: 7,
                first_name: "Grace".to_string(),
                last_name: "Chen".to_string(),
            },
            Student {
                id: 8,
                first_name: "Helen".to_string(),
                last_name: "Cheng".to_string(),
            },
            Student {
                id: 9,
                first_name: "Ivy".to_string(),
                last_name: "Chu".to_string(),
            },
            Student {
                id: 10,
                first_name: "Jack".to_string(),
                last_name: "Chui".to_string(),
            },
            Student {
                id: 11,
                first_name: "Karl".to_string(),
                last_name: "Chow".to_string(),
            },
            Student {
                id: 12,
                first_name: "Lily".to_string(),
                last_name: "Chang".to_string(),
            },
            Student {
                id: 13,
                first_name: "Mandy".to_string(),
                last_name: "Chiu".to_string(),
            },
            Student {
                id: 14,
                first_name: "Nancy".to_string(),
                last_name: "Chen".to_string(),
            },
            Student {
                id: 15,
                first_name: "Oscar".to_string(),
                last_name: "Chen".to_string(),
            },
            Student {
                id: 16,
                first_name: "Peter".to_string(),
                last_name: "Chen".to_string(),
            },
            Student {
                id: 17,
                first_name: "Queenie".to_string(),
                last_name: "Chen".to_string(),
            },
            Student {
                id: 18,
                first_name: "Rachel".to_string(),
                last_name: "Chen".to_string(),
            },
            Student {
                id: 19,
                first_name: "Sandy".to_string(),
                last_name: "Chen".to_string(),
            },
            Student {
                id: 20,
                first_name: "Tom".to_string(),
                last_name: "Chen".to_string(),
            },
            Student {
                id: 21,
                first_name: "Uma".to_string(),
                last_name: "Chen".to_string(),
            },
        ];
        let restrictions = generate_random_restriction_pairs(15, 21);
        let num_groups = 3;
        let (solution, score) =
            simulated_annealing(&students, num_groups, &restrictions, 100.0, 0.1, 1000).unwrap();
        println!("Restrictions: {:?}", restrictions);
        println!("{:?}", create_group_list(&solution, num_groups));
        if score > 0.0 {
            println!("Violations:");
            for restriction in restrictions {
                if solution[&restriction.first_student_id]
                    == solution[&restriction.second_student_id]
                {
                    println!(
                        "Students {} {} in group {}",
                        restriction.first_student_id,
                        restriction.second_student_id,
                        solution[&restriction.first_student_id],
                    );
                }
            }
        }
        assert_eq!(score, 0.0);
    }

    fn create_group_list(solution: &Solution, num_groups: usize) -> Vec<Vec<StudentId>> {
        let mut groups: Vec<Vec<StudentId>> = vec![vec![]; num_groups];
        for (&student, &group) in solution {
            groups[group as usize].push(student);
        }
        groups
    }
}
