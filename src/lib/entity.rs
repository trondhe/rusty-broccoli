pub trait EntityTrait {
	fn new() -> Entity;
	fn get_state(&self) -> &EntityState;
	fn set_state(&mut self, state: &EntityState);
	fn get_position(&self) -> &Position;
	fn set_position(&mut self, position: &Position);
}

pub trait EntityStateTrait {
	fn new() -> EntityState;
}

pub trait PositionTrait {
	fn new() -> Position;
}

////////////////////////////////////

#[derive(PartialEq, Debug)]
pub struct Entity {
	state: EntityState,
}

impl EntityTrait for Entity {
	fn new() -> Entity {
		Entity {
			state: EntityState::new(),
		}
	}

	fn get_state(&self) -> &EntityState {
		&self.state
	}

	fn set_state(&mut self, state: &EntityState) {
		self.state = state.clone();
	}

	fn get_position(&self) -> &Position {
		&self.state.position
	}

	fn set_position(&mut self, position: &Position) {
		self.state.position = position.clone();
	}
}

#[derive(PartialEq, Debug, Clone)]
pub struct EntityState {
	health: f32,
	position: Position,
}

impl EntityStateTrait for EntityState {
	fn new() -> EntityState {
		EntityState {
			health: 0.0,
			position: Position::new(),
		}
	}
}

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
	x: f32,
	y: f32,
	z: f32,
}

impl PositionTrait for Position {
	fn new() -> Position {
		Position {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}
}

#[cfg(test)]
mod entity_test {
	use super::*;

	fn make_position(x: f32, y: f32, z: f32) -> Position {
		let mut position = Position::new();
		
		position.x = x;
		position.y = y;
		position.z = z;
		
		position
	}

	#[test]
	fn new_entity_has_zero_state() {
		let entity = Entity::new();
		let state = entity.get_state();
		assert_eq!(state, &EntityState::new());
	}

	#[test]
	fn new_entity_state_defaults_to_zero() {
		let state = EntityState::new();
		assert_eq!(state.health, 0.0);
		assert_eq!(state.position, Position::new());
	}

	#[test]
	fn can_update_entity_state() {
		let mut entity = Entity::new();
		let mut state = EntityState::new();
		state.health = 100.0;
		state.position = Position::new();
		
		entity.set_state(&state);
		assert_eq!(entity.get_state(), &state);
	}

	#[test]
	fn new_position_defaults_to_zero() {
		let position = Position::new();
		assert_eq!(position.x, 0.0);
		assert_eq!(position.y, 0.0);
		assert_eq!(position.z, 0.0);
	}

	#[test]
	fn can_get_entity_position() {
		let entity = Entity::new();
		assert_eq!(entity.get_position(), &Position::new());
	}

	#[test]
	fn can_update_entity_position() {
		let position = make_position(1.0, 2.0, 3.0);
		let mut entity = Entity::new();
		entity.set_position(&position);
		assert_eq!(entity.get_position(), &position);
	}
}



// fn setup<F>(tf: F)
// where F: Fn (&TestFixture) {

// 	let o = TestFixture { name: String::from ("Initialised") };
// 	tf (&o);
// }

// | o | {
// assert_eq!(o.get_state(), &EntityState::empty());
// }