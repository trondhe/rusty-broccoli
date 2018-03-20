#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]
mod entity;
mod job_handler;
mod examples;
mod threadpool;
mod win;
extern crate winit;
use win::interface;

fn main() {
    interface::start_event_loop();
}
