pub(crate) mod flag;
mod register;

pub(crate) use self::register::*;

#[derive(Debug, Default)]
pub struct CPU {
    pub(crate) register: Register,
}