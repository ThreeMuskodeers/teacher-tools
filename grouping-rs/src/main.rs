use grouping::backtracking::generate_random_constraints;
use rand::seq::SliceRandom;
fn main() {
    let mut students: Vec<usize> = (1..=500).collect();
    let n_groups = 17;
    for i in 0..1000 {
        students.shuffle(&mut rand::thread_rng());
        let restrictions = generate_random_constraints(&students, 70);
        let mut groups = vec![Vec::new(); n_groups];
        let success = grouping::backtracking::assign_students(
            &students,
            &restrictions,
            &mut groups,
            n_groups,
            0,
        );
        if !success {
            println!("Failed to assign students to groups");
            println!("Constraints: {:?}", restrictions);
        }
        if i % 10 == 0 {
            // println!("Iteration: {:?}", groups);
        }
    }
}
