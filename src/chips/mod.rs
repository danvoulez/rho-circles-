pub mod normalize;
pub mod validate;
pub mod policy;
pub mod compile;
pub mod exec;

pub use normalize::normalize;
pub use validate::validate;
pub use policy::policy_eval;
pub use compile::compile;
pub use exec::exec;

/// Base transistor opcodes
pub const OPCODE_NORMALIZE: u8 = 2;
pub const OPCODE_VALIDATE: u8 = 3;
pub const OPCODE_POLICY_EVAL: u8 = 4;
pub const OPCODE_COMPILE: u8 = 5;
pub const OPCODE_EXEC: u8 = 6;
