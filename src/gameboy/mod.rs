mod cpu;

#[derive(Debug, Default)]
pub(crate) struct GameBoy {
    cpu: cpu::CPU,
}