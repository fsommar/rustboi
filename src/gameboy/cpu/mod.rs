pub(crate) mod flag;
pub(crate) mod register;

#[derive(Debug, Default)]
pub struct CPU {
    pub(crate) register: register::Register,
}
