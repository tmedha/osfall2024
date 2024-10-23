// src/mlfq.rs

#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize, // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,
    num_levels: usize,
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Exercise 1: Queue Management
    pub fn add_process(&mut self, process: Process) {
        let priority = {
            if process.priority < self.num_levels {
                process.priority
            } else {
                // Lowest priority available
                self.num_levels - 1
            }
        };
        // Push the process into the queue
        self.queues[priority].push(process);
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
        // Fetch the time quantum
        let t = self.time_quanta[queue_index];
        // Ensure that there is at least one process
        if self.queues[queue_index].len() > 0 {
            let mut p = self.queues[queue_index].remove(0);
            if p.remaining_time > t {
                // p will stay after executing this time quantum
                p.remaining_time -= t;
                p.total_executed_time += t;
                self.current_time += t;
                if queue_index == self.num_levels - 1 {
                    // Already in least priority queue
                    self.queues[queue_index].push(p);
                } else {
                    // Push to the queue with lesser priority
                    self.queues[queue_index + 1].push(p);
                }
            } else {
                // p will finish executing
                p.total_executed_time += p.remaining_time;
                self.current_time += p.remaining_time;
                p.remaining_time = 0;
            }
        }
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // For every queue with lesser priority than the highest
        for i in 1..self.num_levels {
            while self.queues[i].len() > 0 {
                // Empty this queue and move all processes to the queue with the highest priority
                let mut p = self.queues[i].remove(0);
                self.queues[0].push(p);
            }
        }
    }

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);

        let process1 = Process {
            id: 1,
            priority: 0,
            remaining_time: 10,
            total_executed_time: 0,
        };
        let process2 = Process {
            id: 2,
            priority: 1,
            remaining_time: 5,
            total_executed_time: 0,
        };
        let process3 = Process {
            id: 3,
            priority: 5,
            remaining_time: 8,
            total_executed_time: 0,
        };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process {
            id: 1,
            priority: 0,
            remaining_time: 5,
            total_executed_time: 0,
        });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process {
            id: 1,
            priority: 1,
            remaining_time: 5,
            total_executed_time: 3,
        });
        mlfq.queues[2].push(Process {
            id: 2,
            priority: 2,
            remaining_time: 3,
            total_executed_time: 7,
        });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process {
            id: 1,
            priority: 1,
            remaining_time: 5,
            total_executed_time: 3,
        });

        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}
