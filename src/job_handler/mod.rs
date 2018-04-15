use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::RwLock;

use threadpool::Message;
use threadpool::ThreadPool;
use gamestate::GameState;

pub trait JobHandlerTrait {
    fn new() -> JobHandler;
    fn set_threadpool(&mut self, pool_size: &usize);
    fn get_sender(&self) -> Arc<mpsc::Sender<Message>>;
    fn set_gamestate(&mut self, gamestate: Arc<RwLock<GameState>>);
}

pub struct JobHandler {
    pool: ThreadPool,
    gamestate: Option<Arc<RwLock<GameState>>>,
}

impl JobHandlerTrait for JobHandler {
    fn new() -> JobHandler {
        JobHandler {
            pool: ThreadPool::new(1),
            gamestate: None,
        }
    }

    fn set_threadpool(&mut self, pool_size: &usize) {
        self.pool = ThreadPool::new(*pool_size)
    }

    fn get_sender(&self) -> Arc<mpsc::Sender<Message>> {
        self.pool.get_sender()
    }

    fn set_gamestate(&mut self, gamestate: Arc<RwLock<GameState>>) {
        self.gamestate = Some(gamestate);
    }
}

#[cfg(test)]
mod job_handler_test {
    use super::*;

    #[test]
    fn can_create() {
        let job_handler = JobHandler::new();
    }

    #[test]
    fn can_set_threadpool() {
        let pool_size: usize = 10;
        let mut job_handler = JobHandler::new();
        job_handler.set_threadpool(&pool_size);
        assert_eq!(job_handler.pool.pool_size(), pool_size);
    }
}
