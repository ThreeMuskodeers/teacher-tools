pub mod analysis;
mod group_move;
pub mod http;
mod objective;
mod temperature;

use crate::annealing::objective::objective;
use rand::Rng;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};
use tracing::debug;

use self::analysis::get_violations;

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StudentId(usize);
impl std::fmt::Display for StudentId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Deref for StudentId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for StudentId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<usize> for StudentId {
    fn from(id: usize) -> Self {
        StudentId(id)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GroupId(usize);
impl std::fmt::Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Deref for GroupId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for GroupId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<usize> for GroupId {
    fn from(id: usize) -> Self {
        GroupId(id)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RelationshipPair {
    first_student_id: StudentId,
    second_student_id: StudentId,
}

pub type Solution = HashMap<StudentId, GroupId>;

pub type Groups = Vec<Vec<StudentId>>;

#[derive(Debug, serde::Serialize)]
pub struct Violation {
    pub relationship_pair: RelationshipPair,
    pub group: GroupId,
}

pub struct AnnealingResult {
    pub groups: Groups,
    pub violations: Vec<Violation>,
    pub objective: f64,
}

impl std::fmt::Display for AnnealingResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Objective: {}", self.objective)?;
        writeln!(f, "Violations:")?;
        for violation in &self.violations {
            writeln!(
                f,
                "Students {} and {} in group {}",
                violation.relationship_pair.first_student_id,
                violation.relationship_pair.second_student_id,
                violation.group
            )?;
        }
        writeln!(f, "Groups:")?;
        for (i, group) in self.groups.iter().enumerate() {
            writeln!(f, "Group {}: {:?}", i, group)?;
        }
        Ok(())
    }
}

#[tracing::instrument]
pub fn simulated_annealing(
    students: Vec<StudentId>,
    num_groups: usize,
    restrictions: &[RelationshipPair],
    max_temp: f64,
    min_temp: f64,
    steps: i32,
) -> anyhow::Result<AnnealingResult> {
    let mut rng = rand::thread_rng();
    let mut solution: Solution = students
        .into_iter()
        .map(|student| (student, rng.gen_range(0..num_groups).into()))
        .collect();

    let mut best_solution = solution.clone();
    let mut best_score = objective(&solution, num_groups, restrictions)?;

    for step in 0..steps {
        let temp = temperature::change_temp(max_temp, min_temp, steps, step)?;

        let new_solution = group_move::make_move(&solution, num_groups)?;
        let new_score = objective(&new_solution, num_groups, restrictions)?;

        if new_score < best_score || rng.gen::<f64>() < ((best_score - new_score) / temp).exp() {
            solution = new_solution;
            if new_score < best_score {
                best_solution = solution.clone();
                best_score = new_score;
            }
        }
    }
    let result = AnnealingResult {
        groups: create_group_list(&best_solution, num_groups),
        violations: get_violations(&best_solution, restrictions),
        objective: best_score,
    };
    debug!("Result: {}", result);
    Ok(result)
}

fn create_group_list(solution: &Solution, num_groups: usize) -> Vec<Vec<StudentId>> {
    let mut groups: Vec<Vec<StudentId>> = vec![vec![]; num_groups];
    for (student, group) in solution {
        groups[**group].push(student.clone());
    }
    groups
}

#[cfg(test)]
mod tests {
    use crate::annealing::{analysis::get_violations, group_move::all_equal};

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
    fn test_annealing() {
        let students: Vec<StudentId> = (0..21).map(StudentId).collect();
        for _ in 0..1000 {
            let restrictions = generate_random_restriction_pairs(5, 21);
            let num_groups = 4;
            let result =
                simulated_annealing(students.clone(), num_groups, &restrictions, 10.0, 0.1, 1000)
                    .unwrap();
            if !result.violations.is_empty() {
                println!("{:?}", result.groups);
                println!("Restrictions: {:?}", restrictions);
                println!("Violations:");
                for violation in result.violations {
                    println!(
                        "Students {} and {} in group {}",
                        violation.relationship_pair.first_student_id,
                        violation.relationship_pair.second_student_id,
                        violation.group
                    );
                }
            }
            let group_sizes = result
                .groups
                .iter()
                .map(|group| group.len())
                .collect::<Vec<_>>();
            assert!(all_equal(num_groups, students.len(), &group_sizes));
        }
    }
}
