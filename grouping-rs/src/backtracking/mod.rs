use rand::seq::SliceRandom;

pub type StudentId = usize;
pub type Students = Vec<usize>;
#[derive(Debug, Clone)]
pub struct RelationshipPair {
    first_student_id: StudentId,
    second_student_id: StudentId,
}

fn is_valid_assignment(
    student: StudentId,
    group: usize,
    constraints: &[RelationshipPair],
    groups: &[Vec<StudentId>],
    students_per_group: usize,
) -> bool {
    // Check if adding the student violates the even distribution
    if groups[group].len() >= students_per_group {
        return false;
    }
    let passes_criteria = constraints.iter().all(|constraint| {
        let first_student = constraint.first_student_id;
        let second_student = constraint.second_student_id;
        if student == first_student && groups[group].contains(&second_student) {
            return false;
        }
        if student == second_student && groups[group].contains(&first_student) {
            return false;
        }
        true
    });
    passes_criteria
}

pub fn assign_students(
    students: &[StudentId],
    constraints: &[RelationshipPair],
    groups: &mut Vec<Vec<StudentId>>,
    n_groups: usize,
    index: usize,
) -> bool {
    if index == students.len() {
        return true; // All students have been successfully assigned
    }
    let total_students = students.len();
    let base_students_per_group = total_students / n_groups;
    let mut extra_students_needed = total_students % n_groups;

    for group in 0..n_groups {
        // Determine the allowed number of students in this group
        let allowed_students_in_group = if extra_students_needed > 0 {
            extra_students_needed -= 1; // Account for an extra student in this group
            base_students_per_group + 1
        } else {
            base_students_per_group
        };
        if is_valid_assignment(
            students[index],
            group,
            constraints,
            groups,
            allowed_students_in_group,
        ) {
            groups[group].push(students[index]); // Tentatively assign student to this group
            if assign_students(students, constraints, groups, n_groups, index + 1) {
                return true; // Found a valid assignment for all students
            }
            groups[group].pop(); // Backtrack
        }
    }

    false // No valid assignment was found for this student
}

pub fn generate_random_constraints(
    students: &[StudentId],
    n_constraints: usize,
) -> Vec<RelationshipPair> {
    let mut rng = rand::thread_rng();
    (0..n_constraints)
        .map(|_| {
            // Generate two different student ids
            let first_student_id = *students.choose(&mut rng).unwrap();
            let mut second_student_id = *students.choose(&mut rng).unwrap();
            while first_student_id == second_student_id {
                second_student_id = *students.choose(&mut rng).unwrap();
            }

            RelationshipPair {
                first_student_id,
                second_student_id,
            }
        })
        .collect()
}

pub fn check_for_constraint_violations(
    groups: &[Vec<StudentId>],
    constraints: &Vec<RelationshipPair>,
) -> bool {
    let mut violated = false;
    for constraint in constraints {
        let first_student = constraint.first_student_id;
        let second_student = constraint.second_student_id;
        for group in groups {
            if group.contains(&first_student) && group.contains(&second_student) {
                violated = true;
                break;
            }
        }
    }
    violated
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rand::seq::SliceRandom;
//
//     #[test]
//     fn test_group_assignment() {
//         let students = vec![1, 2, 3, 4, 5, 6];
//         let n_groups = 3;
//         let n_constraints = 3;
//         let constraints = generate_random_constraints(&students, n_constraints);
//         let mut groups = vec![Vec::new(); n_groups];
//         let success = assign_students(&students, &constraints, &mut groups, n_groups, 0);
//         assert!(success);
//         assert!(!check_for_constraint_violations(&groups, &constraints));
//     }
//     #[test]
//     fn test_large_number_of_students() {
//         let mut students = (1..=100).collect::<Vec<_>>();
//         students.shuffle(&mut rand::thread_rng());
//
//         let constraints = generate_random_constraints(&students, 10);
//
//         let n_groups = 5;
//         let mut groups: Vec<Vec<usize>> = vec![Vec::new(); n_groups];
//
//         assert!(assign_students(
//             &students,
//             &constraints,
//             &mut groups,
//             n_groups,
//             0,
//         ));
//         assert!(!check_for_constraint_violations(&groups, &constraints));
//     }
// let mut students: Vec<usize> = (1..=500).collect();
// let n_groups = 17;
// for i in 0..1000 {
//     students.shuffle(&mut rand::thread_rng());
//     let restrictions = generate_random_constraints(&students, 70);
//     let mut groups = vec![Vec::new(); n_groups];
//     let success = grouping::backtracking::assign_students(
//         &students,
//         &restrictions,
//         &mut groups,
//         n_groups,
//         0,
//     );
//     if !success {
//         println!("Failed to assign students to groups");
//         println!("Constraints: {:?}", restrictions);
//     }
//     if i % 10 == 0 {
//         // println!("Iteration: {:?}", groups);
//     }
// }
// }
