#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]
mod entity;
mod examples;
mod gamestate;
mod graphics;
mod job_handler;
mod threadpool;
mod win;

extern crate gfx_backend_vulkan as backend;
extern crate gfx_hal as hal;
extern crate winit;

use graphics::*;
use win::window_core::{WindowConfig, WindowCore};

use gamestate::GameState;
use job_handler::{JobHandler, JobHandlerTrait};

use std::thread::sleep;
use std::time::Duration;

use std::mem;
use std::sync::Arc;

fn main() {
    let gamestate = GameState::new();

    let mut handler = JobHandler::new();
    handler.set_gamestate(gamestate.clone());
    handler.set_threadpool(&10);

    let sender = handler.get_sender();

    let mut window_core = WindowCore::new(
        gamestate.clone(),
        sender.clone(),
        WindowConfig::new("RUSTY BROCCOLI", 800, 600),
    );

    loop {
        let gs2 = gamestate.clone();
        let job = Box::new(move || {
            let state = gs2.read().unwrap();
            println!("{:?}", state.keyboard_state.key_map.get("key_a"));
        });
        sender.send(threadpool::Message::NewJob(job)).unwrap();
        sleep(Duration::from_secs(1));
        if window_core.poll_event_loop() {
            return;
        }
    }
}
