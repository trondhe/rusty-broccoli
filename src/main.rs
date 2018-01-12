mod entity;
mod job_handler;
mod examples;

fn main() {
    use job_handler::*;

    use examples::multithreading::*;
    mt_test();

    // use entity::entity_factory::*;
    // let player = EntityFactory::new_player();
    // println!("{:?}", player);

    // while(true) {
    //     job_handler::work();
    // }

    // Process user input
    // Update game/physics/network
    // Render
}
