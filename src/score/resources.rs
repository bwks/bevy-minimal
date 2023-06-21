use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerOneScore {
    pub value: u32,
}

#[derive(Resource, Default)]
pub struct PlayerTwoScore {
    pub value: u32,
}

pub fn update_score(player_one_score: Res<PlayerOneScore>, player_two_score: Res<PlayerTwoScore>) {
    if player_one_score.is_changed() {
        println!("Player 1 Score: {}", player_one_score.value);
    }
    if player_two_score.is_changed() {
        println!("Player 2 Score: {}", player_two_score.value);
    }
}
