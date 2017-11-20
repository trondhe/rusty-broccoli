
use lib::entity::*;

trait PlayerTrait : EntityTrait {
    fn new() -> Player;
}

struct Player {}

// impl PlayerTrait for Player {
//     fn new() -> Player {
//         Player {}
//     }

//     fn get_state(&self) -> &EntityState {

//     }

// 	fn set_state(&mut self, state: &EntityState) {

//     }

// 	fn get_position(&self) -> &Position {

//     }

// 	fn set_position(&mut self, position: &Position) {

//     }
// }

#[cfg(test)]
mod player_test {
	use super::*;

	#[test]
	fn can_create_player() {
        // let player = Player::new();
    }

}