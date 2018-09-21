use super::models::Player;

impl Player {
    pub fn calculate_activity_index(&self) -> f32 {
        (self.time_played as f32).powf(0.333) + (self.rounds_played as f32)
    }

    pub fn calculate_ptfo_index(&self) -> f32 {
        let flags = ((self.flag_captures as f32) + (self.flag_defends as f32)) / self.rounds_played as f32;
        let score = self.gamemode_score * 0.15;
        flags + score
    }

    pub fn calculate_teamwork_index(&self) -> f32 {
        let individual_teamwork_score: f32 = (self.revives + self.repairs + self.heals) as f32 / self.rounds_played as f32;
        let squad_score = self.squad_score / self.rounds_played as f32;

        individual_teamwork_score + squad_score
    }
}

#[cfg(test)]
mod test {
    extern crate assert_approx_eq;

    use super::*;

    const DIFF: f32 = 3.2e-5f32;

    fn dummy_player() -> Player {
        Player {
            id: 1,
            cohort_id: 1,
            name: "dummy".to_string(),
            rank: 120,
            revives: 15.0,
            repairs: 17.0,
            heals: 178.0,
            rounds_played: 432,
            squad_score: 105987.0,
            flag_captures: 1053,
            flag_defends: 823,
            gamemode_score: 128471.0,
            time_played: 144400,
        }
    }

    #[test]
    fn it_calculates_the_activity_index() {
        // (time played)^0.333 + rounds played
        let player = dummy_player();

        let activity_index = player.calculate_activity_index();

        assert_approx_eq!(484.25596538165266, activity_index, DIFF);
    }

    #[test]
    fn it_calculates_the_ptfo_index() {
        // ((flag captures + flag defends) / rounds played)) + (0.15 * game mode score) / (400 * ln(game mode score))
        let player = dummy_player();

        let ptfo_index = player.calculate_ptfo_index();

        assert_approx_eq!(19274.9925925926, ptfo_index, DIFF);
    }

    #[test]
    fn it_calculates_the_teamwork_index() {
        // ((revives + repairs + heals) / rounds played) + (squad score / rounds played)
        let player = dummy_player();

        let teamwork_index = player.calculate_teamwork_index();

        assert_approx_eq!(245.8263888889, teamwork_index, DIFF);
    }
}