mod entity;
mod job_handler;
mod examples;
mod threadpool;

fn main() {
    use job_handler::*;

    use examples::multithreading::*;
    mt_test();

    // use entity::entity_factory::*;
    // let player = EntityFactory::new_player();
    // println!("{:?}", player);

    // while(true) {
    //     job_handler::delegate_work();
    // }

    // Process user input
    // Update game/physics/network
    // Render
}
