use serde::Serialize;

#[derive(Serialize)]
pub struct SkipVotes {
    pub votes: u32,
}
