use super::{Groups, RelationshipPair, StudentId, Violation};

#[derive(serde::Deserialize)]
pub struct CreateGroupsRequest {
    pub num_groups: usize,
    pub students: Vec<StudentId>,
    pub relationship_pairs: Vec<RelationshipPair>,
}

impl CreateGroupsRequest {
    pub fn into_parts(self) -> (usize, Vec<StudentId>, Vec<RelationshipPair>) {
        (self.num_groups, self.students, self.relationship_pairs)
    }
}

#[derive(serde::Serialize)]
pub struct CreateGroupsResponse {
    pub groups: Groups,
    pub violations: Vec<Violation>,
    pub objective: f64,
}
