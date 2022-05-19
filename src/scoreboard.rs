use serde::Deserialize;

#[derive(Deserialize)]
pub struct Scoreboard {
    pub games: Vec<Game>,
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
    #[serde(rename = "gameDuration")]
    pub game_duration: GameDuration,
    pub watch: Watch,
}

#[derive(Deserialize)]
pub struct Watch {
    pub broadcast: Broadcast,
}

#[derive(Deserialize)]
pub struct Broadcast {
    pub broadcasters: Broadcasters,
}

#[derive(Deserialize)]
pub struct Broadcasters {
    pub national: Vec<BroadcastDetails>,
    #[serde(rename = "vTeam")]
    pub v_team: Vec<BroadcastDetails>,
    #[serde(rename = "hTeam")]
    pub h_team: Vec<BroadcastDetails>,
}

#[derive(Deserialize, Clone)]
pub struct BroadcastDetails {
    #[serde(rename = "shortName")]
    pub short_name: String,
}

#[derive(Deserialize)]
pub struct GameDuration {
    pub hours: String,
    pub minutes: String,
}

#[derive(Deserialize)]
pub struct Team {
    #[serde(rename = "teamId")]
    pub team_id: String,
}
