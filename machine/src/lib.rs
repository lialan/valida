#![no_std]

// TODO: Convert memory from big endian to little endian

extern crate alloc;
extern crate self as valida_machine;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub use crate::core::Word;
pub use chip::{
    BusArgument, Chip, Interaction, InteractionType, PermutationPublicInput, ValidaAirBuilder,
};

pub use p3_field::{AbstractField, ExtensionField, Field, PrimeField, PrimeField32, PrimeField64};

pub mod __internal;
pub mod chip;
pub mod config;
pub mod core;
pub mod proof;

pub const OPERAND_ELEMENTS: usize = 5;
pub const INSTRUCTION_ELEMENTS: usize = OPERAND_ELEMENTS + 1;
pub const CPU_MEMORY_CHANNELS: usize = 3;
pub const MEMORY_CELL_BYTES: usize = 4;
pub const LOOKUP_DEGREE_BOUND: usize = 3;

pub trait Instruction<M: Machine> {
    const OPCODE: u32;

    fn execute(state: &mut M, ops: Operands<i32>);
}

pub struct InstructionWord<F> {
    pub opcode: u32,
    pub operands: Operands<F>,
}

#[derive(Copy, Clone, Default)]
pub struct Operands<F>(pub [F; 5]);

impl<F: Copy> Operands<F> {
    pub fn a(&self) -> F {
        self.0[0]
    }
    pub fn b(&self) -> F {
        self.0[1]
    }
    pub fn c(&self) -> F {
        self.0[2]
    }
    pub fn d(&self) -> F {
        self.0[3]
    }
    pub fn e(&self) -> F {
        self.0[4]
    }
    pub fn is_imm(&self) -> F {
        self.0[4]
    }
    pub fn imm32(&self) -> Word<F> {
        Word([self.0[0], self.0[1], self.0[2], self.0[3]])
    }
}

impl<F: PrimeField> Operands<F> {
    pub fn from_i32_slice(slice: &[i32]) -> Self {
        let mut operands = [F::ZERO; 5];
        for (i, &operand) in slice.iter().enumerate() {
            let abs = F::from_canonical_u32(operand.abs() as u32);
            operands[i] = if operand < 0 { -abs } else { abs };
        }
        Self(operands)
    }
}

#[derive(Default)]
pub struct ProgramROM<F>(Vec<InstructionWord<F>>);

impl<F> ProgramROM<F> {
    pub fn new(instructions: Vec<InstructionWord<F>>) -> Self {
        Self(instructions)
    }

    pub fn get_instruction(&self, pc: u32) -> &InstructionWord<F> {
        &self.0[pc as usize]
    }
}

#[derive(Default)]
pub struct PublicMemory<F> {
    pub cells: BTreeMap<u32, Word<F>>,
}

pub trait Machine {
    type F: PrimeField64;
    type EF: ExtensionField<Self::F>;
    fn run(&mut self, program: ProgramROM<i32>, public_memory: PublicMemory<u8>);
    fn prove(&self);
    fn verify();
}
