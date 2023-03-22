use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Pop;

use iced_x86::Register;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::state::registers::SupportedRegister;

impl Axecutor {
    pub(crate) fn mnemonic_pop(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Pop);

        match i.code() {
            Pop_r16 => self.instr_pop_r16(i),
            Pop_r32 => self.instr_pop_r32(i),
            Pop_r64 => self.instr_pop_r64(i),
            Pop_rm16 => self.instr_pop_rm16(i),
            Pop_rm32 => self.instr_pop_rm32(i),
            Pop_rm64 => self.instr_pop_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Pop", i.code()),
        }
    }

    /// POP r16
    ///
    /// o16 58+rw
    fn instr_pop_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_r16);

        let reg: SupportedRegister = i.op0_register().into();
        let rsp = self.reg_read_64(Register::RSP.into())? + 2;

        let value = self.mem_read_16(rsp)?;
        self.reg_write_16(reg, value)?;

        self.reg_write_64(Register::RSP.into(), rsp)?;

        Ok(())
    }

    /// POP r32
    ///
    /// o32 58+rd
    fn instr_pop_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_r32);

        fatal_error!("There's no prefix for encoding this in 64-bit x86-64 (see Intel manual)");
    }

    /// POP r64
    ///
    /// o64 58+ro
    fn instr_pop_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_r64);

        let reg: SupportedRegister = i.op0_register().into();
        let rsp = self.reg_read_64(Register::RSP.into())? + 8;

        let value = self.mem_read_64(rsp)?;
        self.reg_write_64(reg, value)?;

        self.reg_write_64(Register::RSP.into(), rsp)?;

        Ok(())
    }

    /// POP r/m16
    ///
    /// o16 8F /0
    fn instr_pop_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_rm16);

        opcode_unimplemented!("instr_pop_rm16 for Pop")
    }

    /// POP r/m32
    ///
    /// o32 8F /0
    fn instr_pop_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_rm32);

        opcode_unimplemented!("instr_pop_rm32 for Pop")
    }

    /// POP r/m64
    ///
    /// o64 8F /0
    fn instr_pop_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_rm64);

        opcode_unimplemented!("instr_pop_rm64 for Pop")
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_reg_value, ax_test, init_mem_value, write_reg_value};
    use iced_x86::Register::*;

    // pop bx
    ax_test![pop_bx; 0x66, 0x5B;
        |a: &mut Axecutor| {
            // Setup stack
            write_reg_value!(q; a; RSP; 0x1000-2);
            init_mem_value!(w; a; 0x1000; 0x1234u16);

            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1234u64);

            assert_reg_value!(q; a; RSP; 0x1000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // pop rbx
    ax_test![pop_rbx; 0x5B;
        |a: &mut Axecutor| {
            // Setup stack
            write_reg_value!(q; a; RSP; 0x1000-8);
            init_mem_value!(q; a; 0x1000; 0x1234567890ABCDEFu64);

            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1234567890ABCDEFu64);

            assert_reg_value!(q; a; RSP; 0x1000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
