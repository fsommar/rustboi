mod flag;
mod register;

use self::register::*;

#[derive(Debug, Default)]
pub struct CPU {
    register: Register,
}