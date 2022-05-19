use crate::scoreboard::Scoreboard;
use crate::team::Team;
use crate::timezone::Timezone;
use time::format_description::well_known;
use time::{OffsetDateTime, UtcOffset};

mod scoreboard;
mod team;
pub mod timezone;

pub async fn get_nba_times(date: &str, timezone: Timezone) -> Vec<String> {
    let json_raw = reqwest::get(format!(
        "https://data.nba.net/prod/v2/{}/scoreboard.json",
        date.replace('-', "")
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let mut output = Vec::new();

    let scoreboard = serde_json::from_str::<Scoreboard>(&json_raw).unwrap();
    for game in scoreboard.games {
        let start_time = OffsetDateTime::parse(&game.start_time_utc, &well_known::Rfc3339)
            .unwrap()
            .to_offset(UtcOffset::from_hms(timezone.into(), 0, 0).unwrap());
        let end_time_str = if let Some(end_time_str) = game.end_time_utc {
            let end_time = OffsetDateTime::parse(&end_time_str, &well_known::Rfc3339)
                .unwrap()
                .to_offset(UtcOffset::from_hms(timezone.into(), 0, 0).unwrap());
            format!(
                "{:0<2}:{:0<2} {}",
                end_time.hour(),
                end_time.minute(),
                timezone
            )
        } else {
            "N/A".to_string()
        };

        let away: Team = game
            .v_team
            .team_id
            .parse::<u32>()
            .unwrap()
            .try_into()
            .unwrap_or(Team::Unknown);
        let home: Team = game
            .h_team
            .team_id
            .parse::<u32>()
            .unwrap()
            .try_into()
            .unwrap_or(Team::Unknown);

        if away == Team::Unknown || home == Team::Unknown {
            continue;
        }

        output.push(format!(
            "{} at {},{},{},{}",
            away,
            home,
            date,
            format_args!(
                "{:0<2}:{:0<2} {}",
                start_time.hour(),
                start_time.minute(),
                timezone
            ),
            end_time_str
        ));
    }

    output
}
