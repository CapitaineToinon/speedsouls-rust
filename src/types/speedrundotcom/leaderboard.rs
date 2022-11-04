use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Leaderboard {
    pub runs: Vec<LeaderboardRun>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct LeaderboardRun {
    pub place: u32,
    pub run: LeaderboardRunRun,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct LeaderboardRunRun {
    pub id: String,
    pub weblink: String,
}
