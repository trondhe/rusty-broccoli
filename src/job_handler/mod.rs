trait JobHandlerTrait {
    fn new() -> JobHandler;
    fn push(&mut self);
}

struct JobHandler {}

impl JobHandlerTrait for JobHandler {
    fn new() -> JobHandler {
        JobHandler {
        }
    }

    fn push(&mut self) {}
}


#[cfg(test)]
mod job_handler_test {
    use super::*;

    #[test]
    fn can_create() {
        let job_handler = JobHandler::new();
    }

    fn do_damage() {

    }

    fn on_damage() {

    }
    
    // #[test]
    // fn can_add_job_to_queue() {
    //     let mut job_handler = JobHandler::new();
    //     job_handler.push({
    //         w: &do_damage, 
    //         c: &on_damage 
    //     }, JobHandlerPriority::Normal);
    // }

    // fn worker() {
    //     // fetch work from queue
    //     let f = pop();
        
    //     let c = f.callback;
    //     let w = f.work;
        
    //     spawn(w, c);

    // }
}