pub mod lib;
pub mod ast;
pub mod fsm;
pub mod codegen;
// pub mod parse_tests;

// Re-export FSM-related items so they are available at crate root
// pub use fsm::{FSM, Transition, TransitionEvent, build_fsm};