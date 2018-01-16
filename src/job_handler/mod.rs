use std::thread;

use threadpool::ThreadPool;

trait JobHandlerTrait {
    fn new() -> JobHandler;
    fn set_threadpool(&mut self, pool_size: &usize);
}

struct JobHandler {
    pool: ThreadPool,
}

impl JobHandlerTrait for JobHandler {
    fn new() -> JobHandler {
        JobHandler {
            pool: ThreadPool::new(1),
        }
    }

    fn set_threadpool(&mut self, pool_size: &usize) {
        self.pool = ThreadPool::new(*pool_size)
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
