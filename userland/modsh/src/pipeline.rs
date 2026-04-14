//! Pipeline handling
//!
//! Structured output via IPC (no byte streams).

/// Pipeline stage
pub struct Stage {
    pub command: &'static str,
    pub args: &'static [&'static str],
}

/// Pipeline execution
pub struct Pipeline {
    stages: [Option<Stage>; 4],
    count: usize,
}

impl Pipeline {
    pub const fn new() -> Self {
        Self {
            stages: [None, None, None, None],
            count: 0,
        }
    }

    pub fn add(&mut self, stage: Stage) {
        if self.count < 4 {
            self.stages[self.count] = Some(stage);
            self.count += 1;
        }
    }

    pub fn execute(&self) -> i32 {
        // TODO: Create IPC channels between stages
        // TODO: Spawn each stage process
        // TODO: Route structured data through channels
        0
    }
}
