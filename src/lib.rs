use crate::scoreboard::Scoreboard;
use crate::team::Team;
use crate::timezone::Timezone;
use std::ops::Add;
use time::format_description::well_known;
use time::{Duration, OffsetDateTime, UtcOffset};

mod scoreboard;
pub mod team;
pub mod timezone;

pub async fn get_nba_times(date: &str, timezone: Timezone, team: Team) -> Vec<String> {
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
        let mut hours = 0;
        let mut minutes = 0;
        let start_time = OffsetDateTime::parse(&game.start_time_utc, &well_known::Rfc3339)
            .unwrap()
            .to_offset(UtcOffset::from_hms(timezone.into(), 0, 0).unwrap());
        let end_time_str =
            if !game.game_duration.hours.is_empty() && !game.game_duration.minutes.is_empty() {
                hours = game.game_duration.hours.parse::<i64>().unwrap();
                minutes = game.game_duration.minutes.parse::<i64>().unwrap();
                let end_time = start_time
                    .add(Duration::hours(hours))
                    .add(Duration::minutes(minutes));
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

        if away == Team::Unknown
            || home == Team::Unknown
            || !(home == team || away == team || team == Team::All)
        {
            continue;
        }

        let national_broadcasts = game
            .watch
            .broadcast
            .broadcasters
            .national
            .iter()
            .map(|bd| bd.short_name.clone())
            .collect::<Vec<String>>()
            .join(". ");
        let h_broadcasts = game
            .watch
            .broadcast
            .broadcasters
            .h_team
            .iter()
            .map(|bd| bd.short_name.clone())
            .collect::<Vec<String>>()
            .join(". ");
        let v_broadcasts = game
            .watch
            .broadcast
            .broadcasters
            .v_team
            .iter()
            .map(|bd| bd.short_name.clone())
            .collect::<Vec<String>>()
            .join(". ");

        let broadcasts = [national_broadcasts, h_broadcasts, v_broadcasts]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join(". ");

        output.push(format!(
            "{} at {},{},{},{},{},{}",
            away,
            home,
            date,
            format_args!("{}:{:0>2}", hours, minutes),
            format_args!(
                "{:0>2}:{:0>2} {}",
                start_time.hour(),
                start_time.minute(),
                timezone
            ),
            end_time_str,
            broadcasts
        ));
    }

    output
}
