//! Sandbox for active development and experimentation
//!
//! This module is safe for LLM modification. Code here can be freely
//! changed without affecting validated modules.

pub mod experiments;
pub mod housekeep;
pub mod sndinfo;

#[cfg(test)]
mod tests {
    #[test]
    fn sandbox_test() {
        // Sandbox tests
    }
}
