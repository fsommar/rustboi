mod cpu;
mod instr;
mod mem;

#[derive(Debug, Default)]
pub(crate) struct GameBoy {
    pub(crate) cpu: cpu::CPU,
    pub(crate) mmu: mem::MMU,
}

impl GameBoy {
    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch();
            println!("Fetched 0x{:02X?}", opcode);
            if opcode == 0 {
                panic!("Something went wrong!");
            }
            self.execute(opcode).unwrap_or_else(|e| panic!(e));
        }
    }

    fn fetch(&self) -> u8 {
        println!("{}", self.cpu.register);
        self.mmu.read_u8(*self.cpu.register.pc)
    }

    fn advance_pc(&mut self, steps: u8) {
        let pc: &mut u16 = &mut self.cpu.register.pc;
        *pc += u16::from(steps);
    }

    fn execute(&mut self, opcode: u8) -> Result<(), failure::Error> {
        let steps = instr::execute(opcode, self)?;
        println!("PC += {}", steps);
        self.advance_pc(steps);
        Ok(())
    }
}
