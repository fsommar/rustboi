use self::Instruction::*;
use super::{cpu::*, Ctx, GameBoy};
use std::{convert::TryFrom};

macro_rules! instructions {
    ($($opcode:expr , $instruction:ident , $func:expr , { cycles: $cycles:expr, sp: $sp:expr } ; )*) => {
        #[derive(Debug, Copy, Clone)]
        pub(crate) enum Instruction {
            $($instruction),*
        }

        impl Instruction {
            fn apply(&self, ctx: &mut Ctx) -> u8 {
                match self {$(
                    $instruction => { $func(ctx); $sp },
                )*}
            }

            fn opcode(&self) -> u8 {
                match self {$(
                    $instruction => $opcode,
                )*}
            }
        }

        impl TryFrom<u8> for Instruction {
            type Error = String;

            fn try_from(value: u8) -> Result<Instruction, String> {
                match value {
                    $(
                        $opcode => Ok($instruction),
                    )*
                    _ => Err(format!("Opcode `{:x}` isn't mapped yet", value)),
                }
            }
        }
    }
}

instructions! {
    0x00, NOP       , nop          , { cycles: 1, sp: 1 };
    0x01, LdBCImm   , ld_bc_imm    , { cycles: 1, sp: 1 };
    0x02, LdBCAddrA , ld_bc_addr_a , { cycles: 1, sp: 1 };
}

fn nop(_: &mut Ctx) {}

fn ld_bc_imm(ctx: &mut Ctx) {
    *ctx.reg_mut().bc() = ctx.read_u16_imm();
}

fn ld_bc_addr_a(ctx: &mut Ctx) {
    let value = ctx.read_u8(ctx.reg().bc());
    *ctx.reg_mut().a() = value;
}
