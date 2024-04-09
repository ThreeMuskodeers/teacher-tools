use rand::Rng;

use crate::annealing::{GroupId, Solution, StudentId};

pub fn make_move(solution: &Solution, num_groups: usize) -> anyhow::Result<Solution> {
    if num_groups == 1 {
        return Ok(solution.clone());
    }

    if num_groups == 0 {
        return Err(anyhow::anyhow!("num_groups must be greater than 0"));
    }
    // calculate the group sizes
    let mut group_sizes = vec![0; num_groups];
    for &group in solution.values() {
        group_sizes[group as usize] += 1;
    }
    // if group sizes are equal, swap students between groups
    // FIXME: This doesn't take into account mandatory uneven group sizes
    // Ex. 22 students 4 groups, 5, 5, 6, 6
    if group_sizes.iter().all(|&size| size == group_sizes[0]) {
        Ok(swap_students_between_groups(solution, num_groups))
    } else {
        Ok(move_from_large_group(solution, num_groups))
    }
}
fn swap_students_between_groups(solution: &Solution, num_groups: usize) -> Solution {
    let mut new_solution = solution.clone();
    let mut rng = rand::thread_rng();

    // Randomly select two different groups
    let group1 = rng.gen_range(0..num_groups);
    let mut group2 = rng.gen_range(0..num_groups);
    while group1 == group2 {
        group2 = rng.gen_range(0..num_groups);
    }

    let students_in_group1: Vec<StudentId> = new_solution
        .iter()
        .filter_map(|(&student, &group)| {
            if group as usize == group1 {
                Some(student)
            } else {
                None
            }
        })
        .collect();

    let students_in_group2: Vec<StudentId> = new_solution
        .iter()
        .filter_map(|(&student, &group)| {
            if group as usize == group2 {
                Some(student)
            } else {
                None
            }
        })
        .collect();

    // Ensure both groups have at least one student to swap
    if !students_in_group1.is_empty() && !students_in_group2.is_empty() {
        let student_from_group1 = students_in_group1[rng.gen_range(0..students_in_group1.len())];
        let student_from_group2 = students_in_group2[rng.gen_range(0..students_in_group2.len())];

        // Swap the students between the two groups
        new_solution.insert(student_from_group1, group2 as GroupId);
        new_solution.insert(student_from_group2, group1 as GroupId);
    }

    new_solution
}
fn move_from_large_group(solution: &Solution, num_groups: usize) -> Solution {
    let mut group_sizes = vec![0; num_groups];
    for &group in solution.values() {
        group_sizes[group as usize] += 1;
    }

    let (largest_group, smallest_group) = identify_groups(&group_sizes);

    let students_in_largest_group: Vec<StudentId> = solution
        .iter()
        .filter_map(|(&student, &group)| {
            if group == largest_group {
                Some(student)
            } else {
                None
            }
        })
        .collect();

    let mut new_solution = solution.clone();
    let mut rng = rand::thread_rng();
    let random_student =
        students_in_largest_group[rng.gen_range(0..students_in_largest_group.len())];
    new_solution.insert(random_student, smallest_group);

    new_solution
}

fn identify_groups(group_sizes: &[i32]) -> (GroupId, GroupId) {
    let mut largest_group = 0;
    let mut smallest_group = 0;
    for (i, &size) in group_sizes.iter().enumerate() {
        if size > group_sizes[largest_group] {
            largest_group = i;
        }
        if size < group_sizes[smallest_group] {
            smallest_group = i;
        }
    }
    (largest_group as GroupId, smallest_group as GroupId)
}
