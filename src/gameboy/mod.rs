mod cpu;
mod instr;
mod mem;

use self::cpu::*;

#[derive(Debug, Default)]
pub(crate) struct GameBoy {
    cpu: cpu::CPU,
    mmu: mem::MMU,
}

impl GameBoy {
    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch();
            self.advance_pc(1);
            self.execute(opcode).unwrap_or_else(|e| panic!(e));
        }
    }

    fn fetch(&self) -> u8 {
        self.mmu.read_u8(self.cpu.register.pc())
    }

    fn advance_pc(&mut self, steps: u8) {
        let pc: &mut u16 = &mut self.cpu.register.pc();
        *pc += steps as u16;
    }

    fn execute(&mut self, opcode: u8) -> Result<(), String> {
        let steps = instr::execute(opcode, self)?;
        self.advance_pc(steps);
        Ok(())
    }
}
