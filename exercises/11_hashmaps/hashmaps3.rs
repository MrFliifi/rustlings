// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;


#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    let mut scores = HashMap::new();

    for line in results.lines() {
        let mut it = line.split(',');

        let team_1 = it.next().unwrap();
        let team_2 = it.next().unwrap();
        let goals_1: u8 = it.next().unwrap().parse().unwrap();
        let goals_2: u8 = it.next().unwrap().parse().unwrap();

        let team1_entry: &mut TeamScores = scores.entry(team_1).or_default();
        team1_entry.goals_scored += goals_1;
        team1_entry.goals_conceded += goals_2;

        let team2_entry: &mut TeamScores = scores.entry(team_2).or_default();
        team2_entry.goals_scored += goals_2;
        team2_entry.goals_conceded += goals_1;
    }

    scores
}


fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
