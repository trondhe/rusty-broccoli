#[derive(Debug, Copy, Clone)]
enum EntityType {
	Player,
	Weapon,
	Object,
	Ammo,
}

#[derive(Debug, Copy, Clone)]
struct Position {
	x: f32,
	y: f32,
	z: f32,
}

impl Position {
	pub fn new(x: f32, y: f32, z: f32) -> Position {
		Position { x, y, z }
    }

    pub fn empty() -> Position {
        Position { x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[derive(Debug, Copy, Clone)]
struct EntityStatus {
	health: f32,
	weight: f32,
	armor: f32,
	position: Position,
}

impl EntityStatus {
	pub fn new(health: f32, weight: f32, armor: f32) -> EntityStatus {
		EntityStatus {
			health,
			weight,
			armor,
            position: Position::empty()
        }
    }
}

trait EntityTrait {
	fn set_position(&mut self, x: f32, y: f32, z: f32);
    fn get_position(&self) -> &Position;
}

#[derive(Debug, Copy, Clone)]
struct Entity {
	itype: EntityType,
    status: EntityStatus,
    // inventory: Inventory,
}

impl Entity {
    pub fn new(itype: EntityType) -> Entity {
        let status = EntityStatus::new(0.0, 0.0, 0.0);
        Entity { itype, status, }
    }
}

impl EntityTrait for Entity {
    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        let new_position = Position::new(x, y, z);
        self.status.position = new_position;
    }

    fn get_position(&self) -> &Position {
        &self.status.position
    }
}

#[derive(Debug, Copy, Clone)]
struct Player {
	entity: Entity,
}

impl Player {
	fn new() -> Player {
		let entity = Entity::new(EntityType::Player);
		Player { entity, }
    }
}

impl EntityTrait for Player {
	fn set_position(&mut self, x: f32, y: f32, z: f32) {
		&self.entity.set_position(x, y, z);
    }

    fn get_position(&self) -> &Position {
        &self.entity.get_position()
    }
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn can_create_player_object() {
        let player = Player::new();
    }
}

fn main() {
    println!("Hello, world!");
}
