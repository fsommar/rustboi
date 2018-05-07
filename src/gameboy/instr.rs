use self::Instruction::*;
use super::{cpu::*, Ctx, GameBoy};
use std::{convert::TryFrom};

macro_rules! instructions {
    ($($opcode:expr , $instruction:ident , { cycles: $cycles:expr, sp: $sp:expr } ; )*) => {
        #[derive(Debug, Copy, Clone)]
        pub(crate) enum Instruction {
            $($instruction),*
        }

        impl Instruction {
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
    0x00, NOP       , { cycles: 1, sp: 1 };
    0x01, LdBCImm   , { cycles: 1, sp: 1 };
    0x02, LdBCAddrA , { cycles: 1, sp: 1 };
}


trait WithContext {
    fn with<T, A: Argument<T>>(&self, arg: A, func: fn(T) -> Effect) -> Effect;
}

impl<C: Ctx> WithContext for C {
    fn with<T, A: Argument<T>>(&self, arg: A, func: fn(T) -> Effect) -> Effect {
        arg.apply(self, func)
    }
}

impl Instruction {
    fn test<C: Ctx>(&self, ctx: &mut C) {
        use self::{Effect::*, R16::*, R8::*};
        let e: Effect = match self {
            LdBCImm => ctx.with(Immediate16, |x| Load16(BC, x)),
            LdBCAddrA => ctx.with(Lookup(&A), |x| Load16(BC, x.into())),
            NOP => NOOP
        };
        // e.apply(ctx);
    }
}

trait Argument<T> {
    fn get(&self, &Ctx) -> T;
    
    fn apply(&self, ctx: &Ctx, func: fn(T) -> Effect) -> Effect {
        func(self.get(ctx))
    }
}
struct Immediate;
struct Immediate16;
struct Lookup<'a, T: 'a>(&'a Argument<T>);

impl Argument<u8> for Immediate {
    fn get(&self, ctx: &Ctx) -> u8 {
        ctx.read_u8_imm(0)
    }
}

impl Argument<u16> for Immediate16 {
    fn get(&self,ctx: &Ctx) -> u16 {
        ctx.read_u16_imm()
    }
}

impl<'a> Argument<u8> for Lookup<'a, u16> {
    fn get(&self, ctx: &Ctx) -> u8 {
        let addr = self.0.get(ctx);
        ctx.read_u8(addr)
    }
}

impl<'a> Argument<u8> for Lookup<'a, u8> {
    fn get(&self, ctx: &Ctx) -> u8 {
        let addr = self.0.get(ctx) as u16;
        ctx.read_u8(addr)
    }
}

impl<T, TA, U, UA> Argument<(TA, UA)> for (T, U)
    where T: Argument<TA>, U: Argument<UA> {
    fn get(&self, ctx: &Ctx) -> (TA, UA) {
        (self.0.get(ctx), self.1.get(ctx))
    }
}

impl<T, TA, U, UA, V, VA> Argument<(TA, UA, VA)> for (T, U, V)
    where T: Argument<TA>, U: Argument<UA>, V: Argument<VA> {
    fn get(&self, ctx: &Ctx) -> (TA, UA, VA) {
        (self.0.get(ctx), self.1.get(ctx), self.2.get(ctx))
    }
}

impl Argument<u8> for R8 {
    fn get(&self, ctx: &Ctx) -> u8 {
        match self {
            A => ctx.reg().a(),
        }
    }
}

enum R8 {
    A,
}

enum R16 {
    BC,
}

enum Effect {
    NOOP,
    Load(R8, u8),
    Load16(R16, u16),
}

trait ApplyArgument<T> {
    fn apply(&self, ctx: &mut Ctx, func: fn(T) -> Effect);
}