use entity::entity::*;

trait EntityFactoryTrait {
    fn new(entity_type: EntityType) -> Entity;
}

struct EntityFactory {}

impl EntityFactoryTrait for EntityFactory {
    fn new(entity_type: EntityType) -> Entity {
        let mut entity = Entity::new(entity_type);
        entity.set_state(&new_player_default_state());
        entity
    }
}

fn new_player_default_state () -> EntityState {
    let mut state = EntityState::new();
    state.health = 100.0;
    state
}

#[cfg(test)]
mod entity_factory_test {
    use super::*;
    
    #[test]
    fn can_create_player_with_defaults() {
        let player = EntityFactory::new(EntityType::Player);
        assert_eq!(player.get_type(), &EntityType::Player);
        assert_eq!(player.get_state().health, 100.0);
    }
}
