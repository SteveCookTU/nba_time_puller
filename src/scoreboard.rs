use serde::Deserialize;

#[derive(Deserialize)]
pub struct Scoreboard {
    pub games: Vec<Game>
}

#[derive(Deserialize)]
pub struct Game {
    #[serde(rename = "startTimeUTC")]
    pub start_time_utc: String,
    #[serde(rename = "endTimeUTC")]
    pub end_time_utc: Option<String>,
    #[serde(rename = "vTeam")]
    pub v_team: Team,
    #[serde(rename = "hTeam")]
    pub h_team: Team,
}

#[derive(Deserialize)]
pub struct Team {
    #[serde(rename = "teamId")]
    pub team_id: String,
}
