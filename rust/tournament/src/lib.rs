use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Team {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

impl Team {
    fn add_play(&mut self, score: i8) {
        self.mp += 1;

        match score {
            1 => {
                self.w += 1;
                self.p += 3;
            }
            -1 => self.l += 1,
            _ => {
                self.d += 1;
                self.p += 1;
            }
        };
    }
}

pub fn tally(match_results: &str) -> String {
    let teams: HashMap<&str, Team> = match_results
        .lines()
        .map(|line| line.split(';').collect::<Vec<&str>>())
        .fold(HashMap::new(), |mut teams, play_info| {
            let score = match play_info[2] {
                "win" => 1,
                "loss" => -1,
                _ => 0,
            };

            teams
                .entry(play_info[0])
                .or_insert(Default::default())
                .add_play(score);
            teams
                .entry(play_info[1])
                .or_insert(Default::default())
                .add_play(-score);
            teams
        });

    let mut teams_score = Vec::from_iter(teams.iter());
    teams_score.sort_unstable_by(|(a_name, a_scores), (b_name, b_scores)| {
        match b_scores.p.cmp(&a_scores.p) {
            Ordering::Equal => a_name.cmp(b_name),
            other => other,
        }
    });

    teams_score.iter().fold(
        "Team                           | MP |  W |  D |  L |  P".to_string(),
        |board, (name, scores)| {
            board
                + &format!(
                    "\n{:31}|{:3} |{:3} |{:3} |{:3} |{:3}",
                    name, scores.mp, scores.w, scores.d, scores.l, scores.p
                )
        },
    )
}
