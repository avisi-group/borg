//! aarch64
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
use crate::dbt::{
    x86::{
        emitter::{X86SymbolRef, X86Emitter},
        X86TranslationContext,
    },
    TranslationContext, emitter::{Type, TypeKind, Emitter},
};
#[inline(never)]
pub fn u__DecodeA64(
    ctx: &mut X86TranslationContext,
    emitter: X86Emitter,
    pc: X86SymbolRef,
    opcode: X86SymbolRef,
) -> () {
    struct FunctionState {
        v__21: X86SymbolRef,
        v__0: X86SymbolRef,
        v__3: X86SymbolRef,
        pc: X86SymbolRef,
        opcode: X86SymbolRef,
    }
    let fn_state = FunctionState {
        pc,
        opcode,
        v__21: ctx.create_symbol(),
        v__0: ctx.create_symbol(),
        v__3: ctx.create_symbol(),
    };
    let emitter = ctx.emitter();
    return block_0(emitter, fn_state);
    fn block_0(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_0_0: read-var opcode:u32
        let s_0_0 = emitter.read_variable(fn_state.opcode);
        // D [D] s_0_1: write-var v__0:u32 <= s_0_0:u32
        emitter.write_variable(fn_state.v__0, s_0_0);
        // D [D] s_0_2: const #31s : i
        let s_0_2 = 31;
        // D [D] s_0_3: cast zx s_0_0 -> bv
        let s_0_3 = Bits::new(s_0_0 as u128, 32u16);
        // D [D] s_0_4: const #1s : i64
        let s_0_4 = 1;
        // D [D] s_0_5: cast zx s_0_4 -> i
        let s_0_5 = (i128::try_from(s_0_4).unwrap());
        // C [C] s_0_6: const #0s : i
        let s_0_6 = 0;
        // D [D] s_0_7: add s_0_6 s_0_5
        let s_0_7 = (s_0_6 + s_0_5);
        // D [D] s_0_8: bit-extract s_0_3 s_0_2 s_0_7
        let s_0_8 = (Bits::new(
            ((s_0_3) >> (s_0_2)).value(),
            u16::try_from(s_0_7).unwrap(),
        ));
        // D [D] s_0_9: cast reint s_0_8 -> u8
        let s_0_9 = ((s_0_8.value()) != 0);
        // D [D] s_0_10: cast zx s_0_9 -> bv
        let s_0_10 = Bits::new(s_0_9 as u128, 1u16);
        // D [D] s_0_11: const #0u : u8
        let s_0_11 = emitter
            .constant(
                0,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 1,
                },
            );
        // D [D] s_0_12: cast zx s_0_11 -> bv
        let s_0_12 = Bits::new(s_0_11 as u128, 1u16);
        // D [D] s_0_13: cmp-eq s_0_10 s_0_12
        let s_0_13 = ((s_0_10) == (s_0_12));
        // N [-] s_0_14: branch s_0_13 b26 b1
        if s_0_13 {
            return block_26(emitter, fn_state);
        } else {
            return block_1(emitter, fn_state);
        };
    }
    fn block_1(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_1_0: const #0u : u8
        let s_1_0 = emitter
            .constant(
                0,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 1,
                },
            );
        // D [D] s_1_1: not s_1_0
        let s_1_1 = !s_1_0;
        // D [D] s_1_2: branch s_1_1 b5 b2
        if s_1_1 {
            return block_5(emitter, fn_state);
        } else {
            return block_2(emitter, fn_state);
        };
    }
    fn block_2(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_2_0: read-var pc:i
        let s_2_0 = emitter.read_variable(fn_state.pc);
        // D [D] s_2_1: read-var opcode:u32
        let s_2_1 = emitter.read_variable(fn_state.opcode);
        // D [D] s_2_2: call __DecodeA64_Reserved(s_2_0, s_2_1)
        let s_2_2 = u__DecodeA64_Reserved(emitter, s_2_0, s_2_1);
        // D [D] s_2_3: const #15664u : u32
        let s_2_3 = emitter
            .constant(
                15664,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );
        // D [D] s_2_4: read-reg s_2_3:u8
        let s_2_4 = {
            let value = state.read_register::<bool>(s_2_3 as usize);
            tracer.read_register(s_2_3 as usize, &value);
            value
        };
        // D [D] s_2_5: branch s_2_4 b4 b3
        if s_2_4 {
            return block_4(emitter, fn_state);
        } else {
            return block_3(emitter, fn_state);
        };
    }
    fn block_3(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_3_0: return
        return;
    }
    fn block_4(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_4_0: return
        return;
    }
    fn block_5(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_5_0: read-var opcode:u32
        let s_5_0 = emitter.read_variable(fn_state.opcode);
        // D [D] s_5_1: write-var v__3:u32 <= s_5_0:u32
        emitter.write_variable(fn_state.v__3, s_5_0);
        // D [D] s_5_2: const #31s : i
        let s_5_2 = 31;
        // D [D] s_5_3: cast zx s_5_0 -> bv
        let s_5_3 = Bits::new(s_5_0 as u128, 32u16);
        // D [D] s_5_4: const #1s : i64
        let s_5_4 = 1;
        // D [D] s_5_5: cast zx s_5_4 -> i
        let s_5_5 = (i128::try_from(s_5_4).unwrap());
        // C [C] s_5_6: const #0s : i
        let s_5_6 = 0;
        // D [D] s_5_7: add s_5_6 s_5_5
        let s_5_7 = (s_5_6 + s_5_5);
        // D [D] s_5_8: bit-extract s_5_3 s_5_2 s_5_7
        let s_5_8 = (Bits::new(
            ((s_5_3) >> (s_5_2)).value(),
            u16::try_from(s_5_7).unwrap(),
        ));
        // D [D] s_5_9: cast reint s_5_8 -> u8
        let s_5_9 = ((s_5_8.value()) != 0);
        // D [D] s_5_10: cast zx s_5_9 -> bv
        let s_5_10 = Bits::new(s_5_9 as u128, 1u16);
        // D [D] s_5_11: const #1u : u8
        let s_5_11 = emitter
            .constant(
                1,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 1,
                },
            );
        // D [D] s_5_12: cast zx s_5_11 -> bv
        let s_5_12 = Bits::new(s_5_11 as u128, 1u16);
        // D [D] s_5_13: cmp-eq s_5_10 s_5_12
        let s_5_13 = ((s_5_10) == (s_5_12));
        // N [-] s_5_14: branch s_5_13 b25 b6
        if s_5_13 {
            return block_25(emitter, fn_state);
        } else {
            return block_6(emitter, fn_state);
        };
    }
    fn block_6(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_6_0: const #0u : u8
        let s_6_0 = emitter
            .constant(
                0,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 1,
                },
            );
        // D [D] s_6_1: not s_6_0
        let s_6_1 = !s_6_0;
        // D [D] s_6_2: branch s_6_1 b8 b7
        if s_6_1 {
            return block_8(emitter, fn_state);
        } else {
            return block_7(emitter, fn_state);
        };
    }
    fn block_7(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_7_0: read-var pc:i
        let s_7_0 = emitter.read_variable(fn_state.pc);
        // D [D] s_7_1: read-var opcode:u32
        let s_7_1 = emitter.read_variable(fn_state.opcode);
        // D [D] s_7_2: call __DecodeA64_SME(s_7_0, s_7_1)
        let s_7_2 = u__DecodeA64_SME(emitter, s_7_0, s_7_1);
        // D [D] s_7_3: const #15664u : u32
        let s_7_3 = emitter
            .constant(
                15664,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );
        // D [D] s_7_4: read-reg s_7_3:u8
        let s_7_4 = {
            let value = state.read_register::<bool>(s_7_3 as usize);
            tracer.read_register(s_7_3 as usize, &value);
            value
        };
        // D [D] s_7_5: branch s_7_4 b4 b3
        if s_7_4 {
            return block_4(emitter, fn_state);
        } else {
            return block_3(emitter, fn_state);
        };
    }
    fn block_8(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_8_0: read-var opcode:u32
        let s_8_0 = emitter.read_variable(fn_state.opcode);
        // D [D] s_8_1: const #25s : i
        let s_8_1 = 25;
        // D [D] s_8_2: cast zx s_8_0 -> bv
        let s_8_2 = Bits::new(s_8_0 as u128, 32u16);
        // D [D] s_8_3: const #1s : i64
        let s_8_3 = 1;
        // D [D] s_8_4: cast zx s_8_3 -> i
        let s_8_4 = (i128::try_from(s_8_3).unwrap());
        // C [C] s_8_5: const #3s : i
        let s_8_5 = 3;
        // D [D] s_8_6: add s_8_5 s_8_4
        let s_8_6 = (s_8_5 + s_8_4);
        // D [D] s_8_7: bit-extract s_8_2 s_8_1 s_8_6
        let s_8_7 = (Bits::new(
            ((s_8_2) >> (s_8_1)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D [D] s_8_8: cast reint s_8_7 -> u8
        let s_8_8 = (s_8_7.value() as u8);
        // D [D] s_8_9: cast zx s_8_8 -> bv
        let s_8_9 = Bits::new(s_8_8 as u128, 4u16);
        // D [D] s_8_10: const #1u : u8
        let s_8_10 = emitter
            .constant(
                1,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 4,
                },
            );
        // D [D] s_8_11: cast zx s_8_10 -> bv
        let s_8_11 = Bits::new(s_8_10 as u128, 4u16);
        // D [D] s_8_12: cmp-eq s_8_9 s_8_11
        let s_8_12 = ((s_8_9) == (s_8_11));
        // D [D] s_8_13: not s_8_12
        let s_8_13 = !s_8_12;
        // D [D] s_8_14: branch s_8_13 b10 b9
        if s_8_13 {
            return block_10(emitter, fn_state);
        } else {
            return block_9(emitter, fn_state);
        };
    }
    fn block_9(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_9_0: read-var pc:i
        let s_9_0 = emitter.read_variable(fn_state.pc);
        // D [D] s_9_1: read-var opcode:u32
        let s_9_1 = emitter.read_variable(fn_state.opcode);
        // D [D] s_9_2: call __DecodeA64_Unallocated1(s_9_0, s_9_1)
        let s_9_2 = u__DecodeA64_Unallocated1(emitter, s_9_0, s_9_1);
        // D [D] s_9_3: const #15664u : u32
        let s_9_3 = emitter
            .constant(
                15664,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );
        // D [D] s_9_4: read-reg s_9_3:u8
        let s_9_4 = {
            let value = state.read_register::<bool>(s_9_3 as usize);
            tracer.read_register(s_9_3 as usize, &value);
            value
        };
        // D [D] s_9_5: branch s_9_4 b4 b3
        if s_9_4 {
            return block_4(emitter, fn_state);
        } else {
            return block_3(emitter, fn_state);
        };
    }
    fn block_10(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_10_0: read-var opcode:u32
        let s_10_0 = emitter.read_variable(fn_state.opcode);
        // D [D] s_10_1: const #25s : i
        let s_10_1 = 25;
        // D [D] s_10_2: cast zx s_10_0 -> bv
        let s_10_2 = Bits::new(s_10_0 as u128, 32u16);
        // D [D] s_10_3: const #1s : i64
        let s_10_3 = 1;
        // D [D] s_10_4: cast zx s_10_3 -> i
        let s_10_4 = (i128::try_from(s_10_3).unwrap());
        // C [C] s_10_5: const #3s : i
        let s_10_5 = 3;
        // D [D] s_10_6: add s_10_5 s_10_4
        let s_10_6 = (s_10_5 + s_10_4);
        // D [D] s_10_7: bit-extract s_10_2 s_10_1 s_10_6
        let s_10_7 = (Bits::new(
            ((s_10_2) >> (s_10_1)).value(),
            u16::try_from(s_10_6).unwrap(),
        ));
        // D [D] s_10_8: cast reint s_10_7 -> u8
        let s_10_8 = (s_10_7.value() as u8);
        // D [D] s_10_9: cast zx s_10_8 -> bv
        let s_10_9 = Bits::new(s_10_8 as u128, 4u16);
        // D [D] s_10_10: const #2u : u8
        let s_10_10 = emitter
            .constant(
                2,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 4,
                },
            );
        // D [D] s_10_11: cast zx s_10_10 -> bv
        let s_10_11 = Bits::new(s_10_10 as u128, 4u16);
        // D [D] s_10_12: cmp-eq s_10_9 s_10_11
        let s_10_12 = ((s_10_9) == (s_10_11));
        // D [D] s_10_13: not s_10_12
        let s_10_13 = !s_10_12;
        // D [D] s_10_14: branch s_10_13 b12 b11
        if s_10_13 {
            return block_12(emitter, fn_state);
        } else {
            return block_11(emitter, fn_state);
        };
    }
    fn block_11(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_11_0: read-var pc:i
        let s_11_0 = emitter.read_variable(fn_state.pc);
        // D [D] s_11_1: read-var opcode:u32
        let s_11_1 = emitter.read_variable(fn_state.opcode);
        // D [D] s_11_2: call __DecodeA64_SVE(s_11_0, s_11_1)
        let s_11_2 = u__DecodeA64_SVE(emitter, s_11_0, s_11_1);
        // D [D] s_11_3: const #15664u : u32
        let s_11_3 = emitter
            .constant(
                15664,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );
        // D [D] s_11_4: read-reg s_11_3:u8
        let s_11_4 = {
            let value = state.read_register::<bool>(s_11_3 as usize);
            tracer.read_register(s_11_3 as usize, &value);
            value
        };
        // D [D] s_11_5: branch s_11_4 b4 b3
        if s_11_4 {
            return block_4(emitter, fn_state);
        } else {
            return block_3(emitter, fn_state);
        };
    }
    fn block_12(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_12_0: read-var opcode:u32
        let s_12_0 = emitter.read_variable(fn_state.opcode);
        // D [D] s_12_1: const #25s : i
        let s_12_1 = 25;
        // D [D] s_12_2: cast zx s_12_0 -> bv
        let s_12_2 = Bits::new(s_12_0 as u128, 32u16);
        // D [D] s_12_3: const #1s : i64
        let s_12_3 = 1;
        // D [D] s_12_4: cast zx s_12_3 -> i
        let s_12_4 = (i128::try_from(s_12_3).unwrap());
        // C [C] s_12_5: const #3s : i
        let s_12_5 = 3;
        // D [D] s_12_6: add s_12_5 s_12_4
        let s_12_6 = (s_12_5 + s_12_4);
        // D [D] s_12_7: bit-extract s_12_2 s_12_1 s_12_6
        let s_12_7 = (Bits::new(
            ((s_12_2) >> (s_12_1)).value(),
            u16::try_from(s_12_6).unwrap(),
        ));
        // D [D] s_12_8: cast reint s_12_7 -> u8
        let s_12_8 = (s_12_7.value() as u8);
        // D [D] s_12_9: cast zx s_12_8 -> bv
        let s_12_9 = Bits::new(s_12_8 as u128, 4u16);
        // D [D] s_12_10: const #3u : u8
        let s_12_10 = emitter
            .constant(
                3,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 4,
                },
            );
        // D [D] s_12_11: cast zx s_12_10 -> bv
        let s_12_11 = Bits::new(s_12_10 as u128, 4u16);
        // D [D] s_12_12: cmp-eq s_12_9 s_12_11
        let s_12_12 = ((s_12_9) == (s_12_11));
        // D [D] s_12_13: not s_12_12
        let s_12_13 = !s_12_12;
        // D [D] s_12_14: branch s_12_13 b14 b13
        if s_12_13 {
            return block_14(emitter, fn_state);
        } else {
            return block_13(emitter, fn_state);
        };
    }
    fn block_13(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_13_0: read-var pc:i
        let s_13_0 = emitter.read_variable(fn_state.pc);
        // D [D] s_13_1: read-var opcode:u32
        let s_13_1 = emitter.read_variable(fn_state.opcode);
        // D [D] s_13_2: call __DecodeA64_Unallocated2(s_13_0, s_13_1)
        let s_13_2 = u__DecodeA64_Unallocated2(emitter, s_13_0, s_13_1);
        // D [D] s_13_3: const #15664u : u32
        let s_13_3 = emitter
            .constant(
                15664,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );
        // D [D] s_13_4: read-reg s_13_3:u8
        let s_13_4 = {
            let value = state.read_register::<bool>(s_13_3 as usize);
            tracer.read_register(s_13_3 as usize, &value);
            value
        };
        // D [D] s_13_5: branch s_13_4 b4 b3
        if s_13_4 {
            return block_4(emitter, fn_state);
        } else {
            return block_3(emitter, fn_state);
        };
    }
    fn block_14(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_14_0: read-var opcode:u32
        let s_14_0 = emitter.read_variable(fn_state.opcode);
        // D [D] s_14_1: const #26s : i
        let s_14_1 = 26;
        // D [D] s_14_2: cast zx s_14_0 -> bv
        let s_14_2 = Bits::new(s_14_0 as u128, 32u16);
        // D [D] s_14_3: const #1s : i64
        let s_14_3 = 1;
        // D [D] s_14_4: cast zx s_14_3 -> i
        let s_14_4 = (i128::try_from(s_14_3).unwrap());
        // C [C] s_14_5: const #2s : i
        let s_14_5 = 2;
        // D [D] s_14_6: add s_14_5 s_14_4
        let s_14_6 = (s_14_5 + s_14_4);
        // D [D] s_14_7: bit-extract s_14_2 s_14_1 s_14_6
        let s_14_7 = (Bits::new(
            ((s_14_2) >> (s_14_1)).value(),
            u16::try_from(s_14_6).unwrap(),
        ));
        // D [D] s_14_8: cast reint s_14_7 -> u8
        let s_14_8 = (s_14_7.value() as u8);
        // D [D] s_14_9: cast zx s_14_8 -> bv
        let s_14_9 = Bits::new(s_14_8 as u128, 3u16);
        // D [D] s_14_10: const #4u : u8
        let s_14_10 = emitter
            .constant(
                4,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 3,
                },
            );
        // D [D] s_14_11: cast zx s_14_10 -> bv
        let s_14_11 = Bits::new(s_14_10 as u128, 3u16);
        // D [D] s_14_12: cmp-eq s_14_9 s_14_11
        let s_14_12 = ((s_14_9) == (s_14_11));
        // D [D] s_14_13: not s_14_12
        let s_14_13 = !s_14_12;
        // D [D] s_14_14: branch s_14_13 b16 b15
        if s_14_13 {
            return block_16(emitter, fn_state);
        } else {
            return block_15(emitter, fn_state);
        };
    }
    fn block_15(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_15_0: read-var pc:i
        let s_15_0 = emitter.read_variable(fn_state.pc);
        // D [D] s_15_1: read-var opcode:u32
        let s_15_1 = emitter.read_variable(fn_state.opcode);
        // D [D] s_15_2: call __DecodeA64_DataProcImm(s_15_0, s_15_1)
        let s_15_2 = u__DecodeA64_DataProcImm(emitter, s_15_0, s_15_1);
        // D [D] s_15_3: const #15664u : u32
        let s_15_3 = emitter
            .constant(
                15664,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );
        // D [D] s_15_4: read-reg s_15_3:u8
        let s_15_4 = {
            let value = state.read_register::<bool>(s_15_3 as usize);
            tracer.read_register(s_15_3 as usize, &value);
            value
        };
        // D [D] s_15_5: branch s_15_4 b4 b3
        if s_15_4 {
            return block_4(emitter, fn_state);
        } else {
            return block_3(emitter, fn_state);
        };
    }
    fn block_16(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_16_0: read-var opcode:u32
        let s_16_0 = emitter.read_variable(fn_state.opcode);
        // D [D] s_16_1: const #26s : i
        let s_16_1 = 26;
        // D [D] s_16_2: cast zx s_16_0 -> bv
        let s_16_2 = Bits::new(s_16_0 as u128, 32u16);
        // D [D] s_16_3: const #1s : i64
        let s_16_3 = 1;
        // D [D] s_16_4: cast zx s_16_3 -> i
        let s_16_4 = (i128::try_from(s_16_3).unwrap());
        // C [C] s_16_5: const #2s : i
        let s_16_5 = 2;
        // D [D] s_16_6: add s_16_5 s_16_4
        let s_16_6 = (s_16_5 + s_16_4);
        // D [D] s_16_7: bit-extract s_16_2 s_16_1 s_16_6
        let s_16_7 = (Bits::new(
            ((s_16_2) >> (s_16_1)).value(),
            u16::try_from(s_16_6).unwrap(),
        ));
        // D [D] s_16_8: cast reint s_16_7 -> u8
        let s_16_8 = (s_16_7.value() as u8);
        // D [D] s_16_9: cast zx s_16_8 -> bv
        let s_16_9 = Bits::new(s_16_8 as u128, 3u16);
        // D [D] s_16_10: const #5u : u8
        let s_16_10 = emitter
            .constant(
                5,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 3,
                },
            );
        // D [D] s_16_11: cast zx s_16_10 -> bv
        let s_16_11 = Bits::new(s_16_10 as u128, 3u16);
        // D [D] s_16_12: cmp-eq s_16_9 s_16_11
        let s_16_12 = ((s_16_9) == (s_16_11));
        // D [D] s_16_13: not s_16_12
        let s_16_13 = !s_16_12;
        // D [D] s_16_14: branch s_16_13 b18 b17
        if s_16_13 {
            return block_18(emitter, fn_state);
        } else {
            return block_17(emitter, fn_state);
        };
    }
    fn block_17(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_17_0: read-var pc:i
        let s_17_0 = emitter.read_variable(fn_state.pc);
        // D [D] s_17_1: read-var opcode:u32
        let s_17_1 = emitter.read_variable(fn_state.opcode);
        // D [D] s_17_2: call __DecodeA64_BranchExcSys(s_17_0, s_17_1)
        let s_17_2 = u__DecodeA64_BranchExcSys(emitter, s_17_0, s_17_1);
        // D [D] s_17_3: const #15664u : u32
        let s_17_3 = emitter
            .constant(
                15664,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );
        // D [D] s_17_4: read-reg s_17_3:u8
        let s_17_4 = {
            let value = state.read_register::<bool>(s_17_3 as usize);
            tracer.read_register(s_17_3 as usize, &value);
            value
        };
        // D [D] s_17_5: branch s_17_4 b4 b3
        if s_17_4 {
            return block_4(emitter, fn_state);
        } else {
            return block_3(emitter, fn_state);
        };
    }
    fn block_18(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_18_0: read-var opcode:u32
        let s_18_0 = emitter.read_variable(fn_state.opcode);
        // D [D] s_18_1: write-var v__21:u32 <= s_18_0:u32
        emitter.write_variable(fn_state.v__21, s_18_0);
        // D [D] s_18_2: const #27s : i
        let s_18_2 = 27;
        // D [D] s_18_3: cast zx s_18_0 -> bv
        let s_18_3 = Bits::new(s_18_0 as u128, 32u16);
        // D [D] s_18_4: const #1s : i64
        let s_18_4 = 1;
        // D [D] s_18_5: cast zx s_18_4 -> i
        let s_18_5 = (i128::try_from(s_18_4).unwrap());
        // C [C] s_18_6: const #0s : i
        let s_18_6 = 0;
        // D [D] s_18_7: add s_18_6 s_18_5
        let s_18_7 = (s_18_6 + s_18_5);
        // D [D] s_18_8: bit-extract s_18_3 s_18_2 s_18_7
        let s_18_8 = (Bits::new(
            ((s_18_3) >> (s_18_2)).value(),
            u16::try_from(s_18_7).unwrap(),
        ));
        // D [D] s_18_9: cast reint s_18_8 -> u8
        let s_18_9 = ((s_18_8.value()) != 0);
        // D [D] s_18_10: cast zx s_18_9 -> bv
        let s_18_10 = Bits::new(s_18_9 as u128, 1u16);
        // D [D] s_18_11: const #1u : u8
        let s_18_11 = emitter
            .constant(
                1,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 1,
                },
            );
        // D [D] s_18_12: cast zx s_18_11 -> bv
        let s_18_12 = Bits::new(s_18_11 as u128, 1u16);
        // D [D] s_18_13: cmp-eq s_18_10 s_18_12
        let s_18_13 = ((s_18_10) == (s_18_12));
        // N [-] s_18_14: branch s_18_13 b24 b19
        if s_18_13 {
            return block_24(emitter, fn_state);
        } else {
            return block_19(emitter, fn_state);
        };
    }
    fn block_19(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_19_0: const #0u : u8
        let s_19_0 = emitter
            .constant(
                0,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 1,
                },
            );
        // D [D] s_19_1: not s_19_0
        let s_19_1 = !s_19_0;
        // D [D] s_19_2: branch s_19_1 b21 b20
        if s_19_1 {
            return block_21(emitter, fn_state);
        } else {
            return block_20(emitter, fn_state);
        };
    }
    fn block_20(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_20_0: read-var pc:i
        let s_20_0 = emitter.read_variable(fn_state.pc);
        // D [D] s_20_1: read-var opcode:u32
        let s_20_1 = emitter.read_variable(fn_state.opcode);
        // D [D] s_20_2: call __DecodeA64_LoadStore(s_20_0, s_20_1)
        let s_20_2 = u__DecodeA64_LoadStore(emitter, s_20_0, s_20_1);
        // D [D] s_20_3: const #15664u : u32
        let s_20_3 = emitter
            .constant(
                15664,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );
        // D [D] s_20_4: read-reg s_20_3:u8
        let s_20_4 = {
            let value = state.read_register::<bool>(s_20_3 as usize);
            tracer.read_register(s_20_3 as usize, &value);
            value
        };
        // D [D] s_20_5: branch s_20_4 b4 b3
        if s_20_4 {
            return block_4(emitter, fn_state);
        } else {
            return block_3(emitter, fn_state);
        };
    }
    fn block_21(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_21_0: read-var opcode:u32
        let s_21_0 = emitter.read_variable(fn_state.opcode);
        // D [D] s_21_1: const #25s : i
        let s_21_1 = 25;
        // D [D] s_21_2: cast zx s_21_0 -> bv
        let s_21_2 = Bits::new(s_21_0 as u128, 32u16);
        // D [D] s_21_3: const #1s : i64
        let s_21_3 = 1;
        // D [D] s_21_4: cast zx s_21_3 -> i
        let s_21_4 = (i128::try_from(s_21_3).unwrap());
        // C [C] s_21_5: const #2s : i
        let s_21_5 = 2;
        // D [D] s_21_6: add s_21_5 s_21_4
        let s_21_6 = (s_21_5 + s_21_4);
        // D [D] s_21_7: bit-extract s_21_2 s_21_1 s_21_6
        let s_21_7 = (Bits::new(
            ((s_21_2) >> (s_21_1)).value(),
            u16::try_from(s_21_6).unwrap(),
        ));
        // D [D] s_21_8: cast reint s_21_7 -> u8
        let s_21_8 = (s_21_7.value() as u8);
        // D [D] s_21_9: cast zx s_21_8 -> bv
        let s_21_9 = Bits::new(s_21_8 as u128, 3u16);
        // D [D] s_21_10: const #5u : u8
        let s_21_10 = emitter
            .constant(
                5,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 3,
                },
            );
        // D [D] s_21_11: cast zx s_21_10 -> bv
        let s_21_11 = Bits::new(s_21_10 as u128, 3u16);
        // D [D] s_21_12: cmp-eq s_21_9 s_21_11
        let s_21_12 = ((s_21_9) == (s_21_11));
        // D [D] s_21_13: not s_21_12
        let s_21_13 = !s_21_12;
        // D [D] s_21_14: branch s_21_13 b23 b22
        if s_21_13 {
            return block_23(emitter, fn_state);
        } else {
            return block_22(emitter, fn_state);
        };
    }
    fn block_22(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_22_0: read-var pc:i
        let s_22_0 = emitter.read_variable(fn_state.pc);
        // D [D] s_22_1: read-var opcode:u32
        let s_22_1 = emitter.read_variable(fn_state.opcode);
        // D [D] s_22_2: call __DecodeA64_DataProcReg(s_22_0, s_22_1)
        let s_22_2 = u__DecodeA64_DataProcReg(emitter, s_22_0, s_22_1);
        // D [D] s_22_3: const #15664u : u32
        let s_22_3 = emitter
            .constant(
                15664,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );
        // D [D] s_22_4: read-reg s_22_3:u8
        let s_22_4 = {
            let value = state.read_register::<bool>(s_22_3 as usize);
            tracer.read_register(s_22_3 as usize, &value);
            value
        };
        // D [D] s_22_5: branch s_22_4 b4 b3
        if s_22_4 {
            return block_4(emitter, fn_state);
        } else {
            return block_3(emitter, fn_state);
        };
    }
    fn block_23(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_23_0: read-var pc:i
        let s_23_0 = emitter.read_variable(fn_state.pc);
        // D [D] s_23_1: read-var opcode:u32
        let s_23_1 = emitter.read_variable(fn_state.opcode);
        // D [D] s_23_2: call __DecodeA64_DataProcFPSIMD(s_23_0, s_23_1)
        let s_23_2 = u__DecodeA64_DataProcFPSIMD(emitter, s_23_0, s_23_1);
        // D [D] s_23_3: const #15664u : u32
        let s_23_3 = emitter
            .constant(
                15664,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );
        // D [D] s_23_4: read-reg s_23_3:u8
        let s_23_4 = {
            let value = state.read_register::<bool>(s_23_3 as usize);
            tracer.read_register(s_23_3 as usize, &value);
            value
        };
        // D [D] s_23_5: branch s_23_4 b4 b3
        if s_23_4 {
            return block_4(emitter, fn_state);
        } else {
            return block_3(emitter, fn_state);
        };
    }
    fn block_24(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_24_0: const #25s : i
        let s_24_0 = 25;
        // D [D] s_24_1: read-var v__21:u32
        let s_24_1 = emitter.read_variable(fn_state.v__21);
        // D [D] s_24_2: cast zx s_24_1 -> bv
        let s_24_2 = Bits::new(s_24_1 as u128, 32u16);
        // D [D] s_24_3: const #1s : i64
        let s_24_3 = 1;
        // D [D] s_24_4: cast zx s_24_3 -> i
        let s_24_4 = (i128::try_from(s_24_3).unwrap());
        // C [C] s_24_5: const #0s : i
        let s_24_5 = 0;
        // D [D] s_24_6: add s_24_5 s_24_4
        let s_24_6 = (s_24_5 + s_24_4);
        // D [D] s_24_7: bit-extract s_24_2 s_24_0 s_24_6
        let s_24_7 = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_6).unwrap(),
        ));
        // D [D] s_24_8: cast reint s_24_7 -> u8
        let s_24_8 = ((s_24_7.value()) != 0);
        // D [D] s_24_9: cast zx s_24_8 -> bv
        let s_24_9 = Bits::new(s_24_8 as u128, 1u16);
        // D [D] s_24_10: const #0u : u8
        let s_24_10 = emitter
            .constant(
                0,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 1,
                },
            );
        // D [D] s_24_11: cast zx s_24_10 -> bv
        let s_24_11 = Bits::new(s_24_10 as u128, 1u16);
        // D [D] s_24_12: cmp-eq s_24_9 s_24_11
        let s_24_12 = ((s_24_9) == (s_24_11));
        // D [D] s_24_13: not s_24_12
        let s_24_13 = !s_24_12;
        // D [D] s_24_14: branch s_24_13 b21 b20
        if s_24_13 {
            return block_21(emitter, fn_state);
        } else {
            return block_20(emitter, fn_state);
        };
    }
    fn block_25(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_25_0: const #25s : i
        let s_25_0 = 25;
        // D [D] s_25_1: read-var v__3:u32
        let s_25_1 = emitter.read_variable(fn_state.v__3);
        // D [D] s_25_2: cast zx s_25_1 -> bv
        let s_25_2 = Bits::new(s_25_1 as u128, 32u16);
        // D [D] s_25_3: const #1s : i64
        let s_25_3 = 1;
        // D [D] s_25_4: cast zx s_25_3 -> i
        let s_25_4 = (i128::try_from(s_25_3).unwrap());
        // C [C] s_25_5: const #3s : i
        let s_25_5 = 3;
        // D [D] s_25_6: add s_25_5 s_25_4
        let s_25_6 = (s_25_5 + s_25_4);
        // D [D] s_25_7: bit-extract s_25_2 s_25_0 s_25_6
        let s_25_7 = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_6).unwrap(),
        ));
        // D [D] s_25_8: cast reint s_25_7 -> u8
        let s_25_8 = (s_25_7.value() as u8);
        // D [D] s_25_9: cast zx s_25_8 -> bv
        let s_25_9 = Bits::new(s_25_8 as u128, 4u16);
        // D [D] s_25_10: const #0u : u8
        let s_25_10 = emitter
            .constant(
                0,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 4,
                },
            );
        // D [D] s_25_11: cast zx s_25_10 -> bv
        let s_25_11 = Bits::new(s_25_10 as u128, 4u16);
        // D [D] s_25_12: cmp-eq s_25_9 s_25_11
        let s_25_12 = ((s_25_9) == (s_25_11));
        // D [D] s_25_13: not s_25_12
        let s_25_13 = !s_25_12;
        // D [D] s_25_14: branch s_25_13 b8 b7
        if s_25_13 {
            return block_8(emitter, fn_state);
        } else {
            return block_7(emitter, fn_state);
        };
    }
    fn block_26(emitter: &mut X86Emitter, mut fn_state: FunctionState) -> () {
        // D [D] s_26_0: const #25s : i
        let s_26_0 = 25;
        // D [D] s_26_1: read-var v__0:u32
        let s_26_1 = emitter.read_variable(fn_state.v__0);
        // D [D] s_26_2: cast zx s_26_1 -> bv
        let s_26_2 = Bits::new(s_26_1 as u128, 32u16);
        // D [D] s_26_3: const #1s : i64
        let s_26_3 = 1;
        // D [D] s_26_4: cast zx s_26_3 -> i
        let s_26_4 = (i128::try_from(s_26_3).unwrap());
        // C [C] s_26_5: const #3s : i
        let s_26_5 = 3;
        // D [D] s_26_6: add s_26_5 s_26_4
        let s_26_6 = (s_26_5 + s_26_4);
        // D [D] s_26_7: bit-extract s_26_2 s_26_0 s_26_6
        let s_26_7 = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_6).unwrap(),
        ));
        // D [D] s_26_8: cast reint s_26_7 -> u8
        let s_26_8 = (s_26_7.value() as u8);
        // D [D] s_26_9: cast zx s_26_8 -> bv
        let s_26_9 = Bits::new(s_26_8 as u128, 4u16);
        // D [D] s_26_10: const #0u : u8
        let s_26_10 = emitter
            .constant(
                0,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 4,
                },
            );
        // D [D] s_26_11: cast zx s_26_10 -> bv
        let s_26_11 = Bits::new(s_26_10 as u128, 4u16);
        // D [D] s_26_12: cmp-eq s_26_9 s_26_11
        let s_26_12 = ((s_26_9) == (s_26_11));
        // D [D] s_26_13: not s_26_12
        let s_26_13 = !s_26_12;
        // D [D] s_26_14: branch s_26_13 b5 b2
        if s_26_13 {
            return block_5(emitter, fn_state);
        } else {
            return block_2(emitter, fn_state);
        };
    }
}
const REGISTER_NAME_MAP: &[(usize, &str)] = &[
    (0usize, "CYCLE_COUNTER_ID"),
    (8usize, "INSTRUCTION_COUNTER_ID"),
    (16usize, "PMU_EVENT_SW_INCR"),
    (24usize, "PMU_EVENT_L1D_CACHE_REFILL"),
    (32usize, "PMU_EVENT_L1D_CACHE"),
    (40usize, "PMU_EVENT_INST_RETIRED"),
    (48usize, "PMU_EVENT_EXC_TAKEN"),
    (56usize, "PMU_EVENT_BR_MIS_PRED"),
    (64usize, "PMU_EVENT_CPU_CYCLES"),
    (72usize, "PMU_EVENT_INST_SPEC"),
    (80usize, "PMU_EVENT_CHAIN"),
    (88usize, "PMU_EVENT_BR_MIS_PRED_RETIRED"),
    (96usize, "PMU_EVENT_L1D_TLB"),
    (104usize, "PMU_EVENT_REMOTE_ACCESS"),
    (112usize, "PMU_EVENT_LL_CACHE"),
    (120usize, "PMU_EVENT_LL_CACHE_MISS"),
    (128usize, "PMU_EVENT_DTLB_WALK"),
    (136usize, "PMU_EVENT_L1D_CACHE_LMISS_RD"),
    (144usize, "PMU_EVENT_L2D_CACHE_RD"),
    (152usize, "PMU_EVENT_SAMPLE_POP"),
    (160usize, "PMU_EVENT_SAMPLE_FEED"),
    (168usize, "PMU_EVENT_SAMPLE_FILTRATE"),
    (176usize, "PMU_EVENT_SAMPLE_COLLISION"),
    (184usize, "PMU_EVENT_L2D_CACHE_LMISS_RD"),
    (192usize, "PMU_EVENT_LDST_ALIGN_LAT"),
    (200usize, "PMU_EVENT_SVE_PRED_EMPTY_SPEC"),
    (208usize, "PMU_EVENT_SVE_PRED_PARTIAL_SPEC"),
    (216usize, "PMU_EVENT_BRB_FILTRATE"),
    (224usize, "PMU_EVENT_SAMPLE_WRAP"),
    (232usize, "PMU_EVENT_SAMPLE_FEED_BR"),
    (240usize, "PMU_EVENT_SAMPLE_FEED_LD"),
    (248usize, "PMU_EVENT_SAMPLE_FEED_ST"),
    (256usize, "PMU_EVENT_SAMPLE_FEED_OP"),
    (264usize, "PMU_EVENT_SAMPLE_FEED_EVENT"),
    (272usize, "PMU_EVENT_SAMPLE_FEED_LAT"),
    (280usize, "PMU_EVENT_DSNP_HIT_RD"),
    (288usize, "PMU_EVENT_L1D_CACHE_HITM_RD"),
    (296usize, "PMU_EVENT_L2D_CACHE_HITM_RD"),
    (304usize, "PMU_EVENT_L3D_CACHE_HITM_RD"),
    (312usize, "PMU_EVENT_LL_CACHE_HITM_RD"),
    (320usize, "PMU_EVENT_L1D_LFB_HIT_RD"),
    (328usize, "PMU_EVENT_L2D_LFB_HIT_RD"),
    (336usize, "PMU_EVENT_L3D_LFB_HIT_RD"),
    (344usize, "PMU_EVENT_LL_LFB_HIT_RD"),
    (352usize, "M32_User"),
    (360usize, "M32_FIQ"),
    (368usize, "M32_IRQ"),
    (376usize, "M32_Svc"),
    (384usize, "M32_Monitor"),
    (392usize, "M32_Abort"),
    (400usize, "M32_Hyp"),
    (408usize, "M32_Undef"),
    (416usize, "M32_System"),
    (424usize, "EL3"),
    (432usize, "EL2"),
    (440usize, "EL1"),
    (448usize, "EL0"),
    (456usize, "LOG2_TAG_GRANULE"),
    (464usize, "MemAttr_NC"),
    (472usize, "MemAttr_WT"),
    (480usize, "MemAttr_WB"),
    (488usize, "MemHint_No"),
    (496usize, "MemHint_WA"),
    (504usize, "MemHint_RA"),
    (512usize, "MemHint_RWA"),
    (520usize, "GPRs"),
    (768usize, "DefaultPARTID"),
    (776usize, "DefaultPMG"),
    (784usize, "Domain_NoAccess"),
    (792usize, "Domain_Client"),
    (800usize, "FINAL_LEVEL"),
    (808usize, "MAX_VL"),
    (816usize, "MAX_PL"),
    (824usize, "ZT0_LEN"),
    (832usize, "DEFAULT_MECID"),
    (840usize, "GPT_NoAccess"),
    (848usize, "GPT_Table"),
    (856usize, "GPT_Block"),
    (864usize, "GPT_Contig"),
    (872usize, "GPT_Secure"),
    (880usize, "GPT_NonSecure"),
    (888usize, "GPT_Root"),
    (896usize, "GPT_Realm"),
    (904usize, "GPT_Any"),
    (912usize, "GPTRange_4KB"),
    (920usize, "GPTRange_16KB"),
    (928usize, "GPTRange_64KB"),
    (936usize, "GPTRange_2MB"),
    (944usize, "GPTRange_32MB"),
    (952usize, "GPTRange_512MB"),
    (960usize, "GPTRange_1GB"),
    (968usize, "GPTRange_16GB"),
    (976usize, "GPTRange_64GB"),
    (984usize, "GPTRange_512GB"),
    (992usize, "SPEMaxAddrs"),
    (1000usize, "SPEMaxCounters"),
    (1008usize, "SPEMaxRecordSize"),
    (1016usize, "SPEAddrPosPCVirtual"),
    (1024usize, "SPEAddrPosBranchTarget"),
    (1032usize, "SPEAddrPosDataVirtual"),
    (1040usize, "SPEAddrPosDataPhysical"),
    (1048usize, "SPEAddrPosPrevBranchTarget"),
    (1056usize, "SPECounterPosTotalLatency"),
    (1064usize, "SPECounterPosIssueLatency"),
    (1072usize, "SPECounterPosTranslationLatency"),
    (1080usize, "VMID_NONE"),
    (1088usize, "MAX_ZERO_BLOCK_SIZE"),
    (1096usize, "DebugHalt_Breakpoint"),
    (1104usize, "DebugHalt_EDBGRQ"),
    (1112usize, "DebugHalt_Step_Normal"),
    (1120usize, "DebugHalt_Step_Exclusive"),
    (1128usize, "DebugHalt_OSUnlockCatch"),
    (1136usize, "DebugHalt_ResetCatch"),
    (1144usize, "DebugHalt_Watchpoint"),
    (1152usize, "DebugHalt_HaltInstruction"),
    (1160usize, "DebugHalt_SoftwareAccess"),
    (1168usize, "DebugHalt_ExceptionCatch"),
    (1176usize, "DebugHalt_Step_NoSyndrome"),
    (1184usize, "RCW64_PROTECTED_BIT"),
    (1192usize, "RCW128_PROTECTED_BIT"),
    (1200usize, "lst_64bv"),
    (1208usize, "lst_64b"),
    (1216usize, "lst_64bv0"),
    (1224usize, "CFG_ID_AA64PFR0_EL1_EL0"),
    (1232usize, "CFG_ID_AA64PFR0_EL1_EL1"),
    (1240usize, "CFG_ID_AA64PFR0_EL1_EL2"),
    (1248usize, "CFG_ID_AA64PFR0_EL1_EL3"),
    (1256usize, "CFG_PMCR_IDCODE"),
    (1264usize, "CFG_MPAM_none"),
    (1272usize, "CFG_MPAM_v0p1"),
    (1280usize, "CFG_MPAM_v1p1"),
    (1288usize, "CFG_MPAM_frac_none"),
    (1296usize, "CFG_MPAM_frac_v0p1"),
    (1304usize, "CFG_MPAM_frac_v1p1"),
    (1312usize, "DebugException_Breakpoint"),
    (1320usize, "DebugException_BKPT"),
    (1328usize, "DebugException_VectorCatch"),
    (1336usize, "DebugException_Watchpoint"),
    (1344usize, "TAG_GRANULE"),
    (1352usize, "UART_BASE"),
    (1360usize, "GIC_BASE"),
    (1368usize, "__GICD_TYPER"),
    (1376usize, "__GICC_IIDR"),
    (1384usize, "GIC_PENDING_NONE"),
    (1392usize, "COLD_RESET"),
    (1400usize, "ERXMISC1_EL1"),
    (1408usize, "FEAT_VMID16_IMPLEMENTED"),
    (1416usize, "v9Ap0_IMPLEMENTED"),
    (1424usize, "FEAT_SVE_PMULL128_IMPLEMENTED"),
    (1432usize, "__DBG_ROM_ADDR"),
    (1440usize, "_ERXMISC7"),
    (1448usize, "GICH_EISR"),
    (1456usize, "_VTCR"),
    (1464usize, "SCTLR2_EL3"),
    (1472usize, "ICC_CTLR_EL1_NS"),
    (1480usize, "ID_ISAR5_EL1"),
    (1488usize, "FEAT_EVT_IMPLEMENTED"),
    (1496usize, "_PMINTENSET"),
    (1504usize, "AMPIDR2"),
    (1512usize, "FEAT_EL3_IMPLEMENTED"),
    (1520usize, "PMSDSFR_EL1"),
    (1528usize, "_ICV_CTLR"),
    (1536usize, "PMVCIDSR"),
    (1544usize, "FEAT_FGT2_IMPLEMENTED"),
    (1552usize, "RLPIDEN"),
    (1560usize, "FEAT_ETEv1p2_IMPLEMENTED"),
    (1568usize, "FEAT_AES_IMPLEMENTED"),
    (1576usize, "__max_implemented_smeveclen"),
    (1592usize, "MECID_A0_EL2"),
    (1600usize, "ICC_AP1R_EL1_S"),
    (1632usize, "FEAT_SHA256_IMPLEMENTED"),
    (1640usize, "EDCIDR2"),
    (1648usize, "LORC_EL1"),
    (1656usize, "_PMEVCNTR"),
    (1784usize, "__exclusive_granule_size"),
    (1792usize, "FEAT_FGT_IMPLEMENTED"),
    (1800usize, "_Z"),
    (9992usize, "FEAT_GICv4_IMPLEMENTED"),
    (10000usize, "FEAT_SEL2_IMPLEMENTED"),
    (10008usize, "_ICH_AP1R"),
    (10024usize, "FEAT_SME2p1_IMPLEMENTED"),
    (10032usize, "__ETEBase"),
    (10040usize, "GICC_BPR"),
    (10048usize, "CONTEXTIDR_EL1"),
    (10056usize, "GICR_STATUSR"),
    (10064usize, "CNTHVS_CVAL_EL2"),
    (10072usize, "STACK_LIMIT"),
    (10080usize, "GICC_ABPR"),
    (10088usize, "_CTR"),
    (10096usize, "GICC_AIAR"),
    (10104usize, "FEAT_RASSAv1p1_IMPLEMENTED"),
    (10112usize, "ERXADDR_EL1"),
    (10120usize, "FEAT_PACQARMA5_IMPLEMENTED"),
    (10128usize, "OSLSR_EL1"),
    (10136usize, "GICR_SETLPIR"),
    (10144usize, "PMSLATFR_EL1"),
    (10152usize, "_DBGDTRRXext"),
    (10160usize, "_HDCR"),
    (10168usize, "BRBINFINJ_EL1"),
    (10176usize, "PMCEID1_EL0"),
    (10184usize, "SP_EL1"),
    (10192usize, "CP15SDISABLE"),
    (10200usize, "ICC_SRE_EL3"),
    (10208usize, "FEAT_HPMN0_IMPLEMENTED"),
    (10216usize, "GCSPR_EL2"),
    (10224usize, "_ERXMISC1"),
    (10232usize, "GICV_RPR"),
    (10240usize, "ICH_LR_EL2"),
    (10368usize, "__highest_el_aarch32"),
    (10376usize, "SMCR_EL2"),
    (10384usize, "SPERecordSize"),
    (10400usize, "FEAT_RNG_TRAP_IMPLEMENTED"),
    (10408usize, "FEAT_DoPD_IMPLEMENTED"),
    (10416usize, "ERXCTLR_EL1"),
    (10424usize, "__cycle_count"),
    (10440usize, "PMUACR_EL1"),
    (10448usize, "_CNTV_CTL"),
    (10456usize, "_ICC_CTLR_S"),
    (10464usize, "FEAT_HAFT_IMPLEMENTED"),
    (10472usize, "FEAT_PMUv3_EXT32_IMPLEMENTED"),
    (10480usize, "ACTLR2_S"),
    (10488usize, "_ID_MMFR1"),
    (10496usize, "GICD_CTLR"),
    (10504usize, "CNTHPS_CTL_EL2"),
    (10512usize, "AMCFGR_EL0"),
    (10520usize, "PMCIDR2"),
    (10528usize, "SPESampleInstIsNV2"),
    (10536usize, "VBAR_S"),
    (10544usize, "MAIR_EL2"),
    (10552usize, "FEAT_PACIMP_IMPLEMENTED"),
    (10560usize, "PMULastThresholdValue"),
    (10592usize, "R25"),
    (10600usize, "ICV_IGRPEN1_EL1"),
    (10608usize, "ID_AA64AFR0_EL1"),
    (10616usize, "ACTLR_EL2"),
    (10624usize, "FEAT_DGH_IMPLEMENTED"),
    (10632usize, "GITS_TYPER"),
    (10640usize, "__monomorphize_reads"),
    (10648usize, "MPAMVPM1_EL2"),
    (10656usize, "RNDRRS"),
    (10664usize, "SPERecordData"),
    (10728usize, "GICR_VSGIR"),
    (10736usize, "TCR_EL3"),
    (10744usize, "PMEVCNTR_EL0"),
    (11000usize, "_MAIR0_NS"),
    (11008usize, "EDRCR"),
    (11016usize, "IFSR_S"),
    (11024usize, "FEAT_FlagM2_IMPLEMENTED"),
    (11032usize, "MPAMIDR_EL1"),
    (11040usize, "ICH_MISR_EL2"),
    (11048usize, "_AIFSR_NS"),
    (11056usize, "GICC_AHPPIR"),
    (11064usize, "ZCR_EL3"),
    (11072usize, "_ERXFR"),
    (11080usize, "_ID_DFR0"),
    (11088usize, "CPTR_EL2"),
    (11096usize, "APIBKeyLo_EL1"),
    (11104usize, "NUM_PMU_COUNTERS"),
    (11120usize, "PMPCSCTL"),
    (11128usize, "__RD_base"),
    (11136usize, "SPESampleAddressValid"),
    (11168usize, "VSTCR_EL2"),
    (11176usize, "__max_implemented_sveveclen"),
    (11192usize, "_CNTHCTL"),
    (11200usize, "FEAT_ETMv4p1_IMPLEMENTED"),
    (11208usize, "PMEVTYPER_EL0"),
    (11464usize, "TRFCR_EL1"),
    (11472usize, "GICC_HPPIR"),
    (11480usize, "GCR_EL1"),
    (11488usize, "R23"),
    (11496usize, "FEAT_TIDCP1_IMPLEMENTED"),
    (11504usize, "DACR_S"),
    (11512usize, "EDPIDR1"),
    (11520usize, "_SDER32_EL3"),
    (11528usize, "SPESampleSubclassValid"),
    (11536usize, "ICC_AP1R_EL1_NS"),
    (11568usize, "_DBGDTR_EL0"),
    (11576usize, "FEAT_LSE128_IMPLEMENTED"),
    (11584usize, "__rme_l0gptsz"),
    (11592usize, "AMAIR_EL3"),
    (11600usize, "FEAT_AMUv1_IMPLEMENTED"),
    (11608usize, "FEAT_PMUv3_EDGE_IMPLEMENTED"),
    (11616usize, "TTBR0_NS"),
    (11624usize, "FEAT_AIE_IMPLEMENTED"),
    (11632usize, "ICC_CTLR_EL3"),
    (11640usize, "PMMIR"),
    (11648usize, "TRFCR_EL2"),
    (11656usize, "R28"),
    (11664usize, "FEAT_PCSRv8p2_IMPLEMENTED"),
    (11672usize, "TPIDRPRW_S"),
    (11680usize, "v8Ap0_IMPLEMENTED"),
    (11688usize, "FEAT_AA64EL2_IMPLEMENTED"),
    (11696usize, "LR_mon"),
    (11704usize, "GCSCR_EL3"),
    (11712usize, "_IFSR_NS"),
    (11720usize, "SCTLR2_EL1"),
    (11728usize, "ICC_NMIAR1_EL1"),
    (11736usize, "PMCNTENSET_EL0"),
    (11744usize, "ID_PFR2_EL1"),
    (11752usize, "_AMEVTYPER0"),
    (11768usize, "_ICH_LRC"),
    (11832usize, "EDDEVTYPE"),
    (11840usize, "FEAT_IDST_IMPLEMENTED"),
    (11848usize, "IsWFEsleep"),
    (11856usize, "_ICC_IAR1"),
    (11864usize, "FEAT_AA64EL3_IMPLEMENTED"),
    (11872usize, "_ICH_MISR"),
    (11880usize, "FEAT_PMUv3_ICNTR_IMPLEMENTED"),
    (11888usize, "HPFAR_EL2"),
    (11896usize, "APGAKeyLo_EL1"),
    (11904usize, "ICC_SRE_EL1_S"),
    (11912usize, "_ERXSTATUS"),
    (11920usize, "GICR_WAKER"),
    (11928usize, "FEAT_SVE_IMPLEMENTED"),
    (11936usize, "S2PIR_EL2"),
    (11944usize, "SPMACCESSR_EL1"),
    (11952usize, "_ICC_AP0R"),
    (11968usize, "CNTFID0"),
    (11976usize, "TPIDR_EL2"),
    (11984usize, "ICC_IGRPEN1_EL3"),
    (11992usize, "ESR_EL3"),
    (12000usize, "GICR_VSGIPENDR"),
    (12008usize, "FEAT_CSSC_IMPLEMENTED"),
    (12016usize, "R6"),
    (12024usize, "FEAT_SPEv1p1_IMPLEMENTED"),
    (12032usize, "FEAT_SCTLR2_IMPLEMENTED"),
    (12040usize, "FEAT_MTE_TAGGED_FAR_IMPLEMENTED"),
    (12048usize, "ICV_IGRPEN0_EL1"),
    (12056usize, "GICD_TYPER2"),
    (12064usize, "_CCSIDR"),
    (12072usize, "DBGCLAIMSET_EL1"),
    (12080usize, "SP_EL3"),
    (12088usize, "CPACR_EL1"),
    (12096usize, "_HVBAR"),
    (12104usize, "PMVIDSR"),
    (12112usize, "FEAT_TRBE_MPAM_IMPLEMENTED"),
    (12120usize, "ICV_IAR0_EL1"),
    (12128usize, "FEAT_BRBEv1p1_IMPLEMENTED"),
    (12136usize, "SPIDEN"),
    (12144usize, "FEAT_PMUv3p1_IMPLEMENTED"),
    (12152usize, "FEAT_SME_FA64_IMPLEMENTED"),
    (12160usize, "_HAMAIR0"),
    (12168usize, "FEAT_TWED_IMPLEMENTED"),
    (12176usize, "PIR_EL3"),
    (12184usize, "DBGBCR_EL1"),
    (12696usize, "STACK_BASE"),
    (12704usize, "_ICC_RPR"),
    (12712usize, "AMAIR0_S"),
    (12720usize, "GICV_STATUSR"),
    (12728usize, "PMITCTRL"),
    (12736usize, "PMSIRR_EL1"),
    (12744usize, "_PC"),
    (12752usize, "_ICC_ASGI1R"),
    (12760usize, "NUM_AMU_COUNTER_GROUPS"),
    (12776usize, "ICC_PMR_EL1"),
    (12784usize, "FEAT_RASSAv2_IMPLEMENTED"),
    (12792usize, "_MPAM3_EL3"),
    (12800usize, "FEAT_PAN3_IMPLEMENTED"),
    (12808usize, "CNTHCTL_EL2"),
    (12816usize, "TCR_EL2"),
    (12824usize, "ICV_CTLR_EL1"),
    (12832usize, "AMAIR_EL2"),
    (12840usize, "_MVFR1"),
    (12848usize, "_ICC_AP1R_NS"),
    (12864usize, "_CCSIDR2"),
    (12872usize, "_AMCGCR"),
    (12880usize, "TFSR_EL1"),
    (12888usize, "_HSR"),
    (12896usize, "FEAT_RASv2_IMPLEMENTED"),
    (12904usize, "PMSNEVFR_EL1"),
    (12912usize, "FEAT_CSV2_1p2_IMPLEMENTED"),
    (12920usize, "FPCR"),
    (12928usize, "_PMCCNTR"),
    (12936usize, "ERXMISC3_EL1"),
    (12944usize, "PMICNTR_EL0"),
    (12952usize, "__dczid_log2_block_size"),
    (12968usize, "EDPIDR2"),
    (12976usize, "_Dclone"),
    (13232usize, "CTIAUTHSTATUS"),
    (13240usize, "__syncAbortOnTTWNonCache"),
    (13248usize, "__syncAbortOnReadNormNonCache"),
    (13256usize, "_ICV_DIR"),
    (13264usize, "_AIDR"),
    (13272usize, "PMSSCR_EL1"),
    (13280usize, "_CNTP_CTL_NS"),
    (13288usize, "FEAT_AA32EL3_IMPLEMENTED"),
    (13296usize, "_AMEVTYPER1"),
    (13360usize, "DLR_EL0"),
    (13368usize, "AFSR0_EL2"),
    (13376usize, "_TTBCR2_NS"),
    (13384usize, "_ICV_BPR1"),
    (13392usize, "__mpam_pmg_max"),
    (13400usize, "FEAT_HPDS2_IMPLEMENTED"),
    (13408usize, "FEAT_PMUv3p9_IMPLEMENTED"),
    (13416usize, "_HADFSR"),
    (13424usize, "_ICH_ELRSR"),
    (13432usize, "APGAKeyHi_EL1"),
    (13440usize, "AMCNTENSET1_EL0"),
    (13448usize, "APDAKeyHi_EL1"),
    (13456usize, "PhysicalCount"),
    (13472usize, "__GICITSControlBase"),
    (13480usize, "ID_AA64PFR2_EL1"),
    (13488usize, "_AMCFGR"),
    (13496usize, "BRBIDR0_EL1"),
    (13504usize, "SPESampleTimestamp"),
    (13512usize, "GICR_SYNCR"),
    (13520usize, "_NMRR_NS"),
    (13528usize, "SPESampleSubclass"),
    (13536usize, "_MPAM1_EL1"),
    (13544usize, "_ID_MMFR5"),
    (13552usize, "ICV_EOIR0_EL1"),
    (13560usize, "FEAT_ExS_IMPLEMENTED"),
    (13568usize, "ICV_HPPIR0_EL1"),
    (13576usize, "FEAT_BBM_IMPLEMENTED"),
    (13584usize, "__sme_only"),
    (13592usize, "POR_EL1"),
    (13600usize, "__ThisInstrEnc"),
    (13608usize, "HFGITR_EL2"),
    (13616usize, "PMECR_EL1"),
    (13624usize, "EDAA32PFR"),
    (13632usize, "DISR_EL1"),
    (13640usize, "_ID_ISAR6"),
    (13648usize, "VNCR_EL2"),
    (13656usize, "FEAT_PFAR_IMPLEMENTED"),
    (13664usize, "ICC_EOIR0_EL1"),
    (13672usize, "GICR_IIDR"),
    (13680usize, "CTICIDR0"),
    (13688usize, "SPMACCESSR_EL3"),
    (13696usize, "CNTEL0ACR"),
    (13704usize, "PMBSR_EL1"),
    (13712usize, "_AMCR"),
    (13720usize, "_ICV_RPR"),
    (13728usize, "__impdef_TG1"),
    (13736usize, "CTIDEVTYPE"),
    (13744usize, "EDCIDR1"),
    (13752usize, "CTIDEVCTL"),
    (13760usize, "_HTRFCR"),
    (13768usize, "FEAT_RASv1p1_IMPLEMENTED"),
    (13776usize, "SPESampleAddress"),
    (14032usize, "__last_branch_valid"),
    (14040usize, "EDPRSR"),
    (14048usize, "CFG_MPIDR"),
    (14056usize, "FEAT_Debugv8p2_IMPLEMENTED"),
    (14064usize, "FEAT_LRCPC_IMPLEMENTED"),
    (14072usize, "PMPIDR2"),
    (14080usize, "_IFAR_NS"),
    (14088usize, "_HAIFSR"),
    (14096usize, "_DBGWCR"),
    (14160usize, "CNTPS_CVAL_EL1"),
    (14168usize, "_TTBR1_EL1"),
    (14184usize, "SPESampleDataSourceValid"),
    (14192usize, "AMDEVTYPE"),
    (14200usize, "POR_EL3"),
    (14208usize, "_EDSCR2"),
    (14216usize, "__supported_va_size"),
    (14232usize, "FEAT_HCX_IMPLEMENTED"),
    (14240usize, "__CNTbase_frequency"),
    (14248usize, "GITS_CBASER"),
    (14256usize, "__mpam_frac"),
    (14264usize, "FEAT_ADERR_IMPLEMENTED"),
    (14272usize, "_PMCNTEN"),
    (14280usize, "TPIDR_EL1"),
    (14288usize, "_TPIDRURW_NS"),
    (14296usize, "FEAT_AMUv1p1_IMPLEMENTED"),
    (14304usize, "FEAT_CSV2_1p1_IMPLEMENTED"),
    (14312usize, "FEAT_ANERR_IMPLEMENTED"),
    (14320usize, "APDBKeyHi_EL1"),
    (14328usize, "NUM_GIC_PREEMPTION_BITS"),
    (14344usize, "__set_mops_option_a_supported"),
    (14352usize, "FEAT_LS64_V_IMPLEMENTED"),
    (14360usize, "HEAP_LIMIT"),
    (14368usize, "_PMCEID0"),
    (14376usize, "sp_rel_access_pc"),
    (14384usize, "ID_ISAR1_EL1"),
    (14392usize, "_ERRIDR"),
    (14400usize, "__has_sme_priority_control"),
    (14408usize, "GICR_CLRLPIR"),
    (14416usize, "ERXGSR_EL1"),
    (14424usize, "FEAT_TRC_SR_IMPLEMENTED"),
    (14432usize, "FEAT_RNG_IMPLEMENTED"),
    (14440usize, "GITS_MPIDR"),
    (14448usize, "FEAT_PMUv3p5_IMPLEMENTED"),
    (14456usize, "FEAT_LVA3_IMPLEMENTED"),
    (14464usize, "FEAT_MTE_STORE_ONLY_IMPLEMENTED"),
    (14472usize, "FEAT_PCSRv8p9_IMPLEMENTED"),
    (14480usize, "FEAT_SPE_FDS_IMPLEMENTED"),
    (14488usize, "_AMAIR1_NS"),
    (14496usize, "ICC_IGRPEN0_EL1"),
    (14504usize, "_PMINTEN"),
    (14512usize, "GICR_CTLR"),
    (14520usize, "DBGDEVID"),
    (14528usize, "throw"),
    (14536usize, "_TTBR0_EL1"),
    (14552usize, "__CNTBaseN"),
    (14560usize, "_FFR"),
    (14592usize, "CNTPOFF_EL2"),
    (14600usize, "APDAKeyLo_EL1"),
    (14608usize, "ID_AA64ISAR1_EL1"),
    (14616usize, "AFSR1_EL3"),
    (14624usize, "FEAT_SHA512_IMPLEMENTED"),
    (14632usize, "AMEVCNTR0"),
    (14664usize, "AMCGCR_EL0"),
    (14672usize, "FEAT_EL1_IMPLEMENTED"),
    (14680usize, "_ID_ISAR3"),
    (14688usize, "_PMSWINC"),
    (14696usize, "FEAT_IVIPT_IMPLEMENTED"),
    (14704usize, "SEE"),
    (14720usize, "EDESR"),
    (14728usize, "_IFAR_S"),
    (14736usize, "_ID_PFR0"),
    (14744usize, "PMSIDR_EL1"),
    (14752usize, "FEAT_SB_IMPLEMENTED"),
    (14760usize, "_CNTHP_CVAL"),
    (14768usize, "FEAT_PCSRv8_IMPLEMENTED"),
    (14776usize, "R29"),
    (14784usize, "TCR2_EL1"),
    (14792usize, "FEAT_LSE_IMPLEMENTED"),
    (14800usize, "APIAKeyHi_EL1"),
    (14808usize, "ZCR_EL3_LEN_VALUE"),
    (14824usize, "FEAT_SVE_BitPerm_IMPLEMENTED"),
    (14832usize, "HTTBR"),
    (14840usize, "ICH_AP0R_EL2"),
    (14872usize, "ID_AA64ISAR2_EL1"),
    (14880usize, "CNTHVS_CTL_EL2"),
    (14888usize, "SPESampleContextEL2Valid"),
    (14896usize, "ICC_ASGI1R_EL1"),
    (14904usize, "ID_AA64MMFR0_EL1"),
    (14912usize, "HACR_EL2"),
    (14920usize, "FEAT_CONSTPACFIELD_IMPLEMENTED"),
    (14928usize, "FEAT_GICv3_IMPLEMENTED"),
    (14936usize, "FEAT_CHK_IMPLEMENTED"),
    (14944usize, "FEAT_ETEv1p1_IMPLEMENTED"),
    (14952usize, "__BranchTaken"),
    (14960usize, "TFSRE0_EL1"),
    (14968usize, "MDRAR_EL1"),
    (14976usize, "PMCEID0_EL0"),
    (14984usize, "GITS_CREADR"),
    (14992usize, "PMIIDR"),
    (15000usize, "_ID_ISAR4"),
    (15008usize, "__CNTCTLBase"),
    (15016usize, "_ERXMISC4"),
    (15024usize, "GITS_CTLR"),
    (15032usize, "GICM_CLRSPI_NSR"),
    (15040usize, "RVBAR"),
    (15048usize, "_EDSCR"),
    (15056usize, "SDCR"),
    (15064usize, "IFSR32_EL2"),
    (15072usize, "ICV_PMR_EL1"),
    (15080usize, "ZCR_EL2"),
    (15088usize, "_AMEVCNTR1"),
    (15216usize, "FEAT_FRINTTS_IMPLEMENTED"),
    (15224usize, "_SPSR_svc"),
    (15232usize, "__empam_tidr_implemented"),
    (15240usize, "DBGDEVID1"),
    (15248usize, "FEAT_TRC_EXT_IMPLEMENTED"),
    (15256usize, "_ERXMISC0"),
    (15264usize, "FEAT_F32MM_IMPLEMENTED"),
    (15272usize, "v8Ap3_IMPLEMENTED"),
    (15280usize, "ERRIDR_EL1"),
    (15288usize, "GICC_AEOIR"),
    (15296usize, "GICC_DIR"),
    (15304usize, "FEAT_ECV_IMPLEMENTED"),
    (15312usize, "_CPACR"),
    (15320usize, "FEAT_SPEv1p2_IMPLEMENTED"),
    (15328usize, "__syncAbortOnPrefetch"),
    (15336usize, "VTCR_EL2"),
    (15344usize, "POR_EL2"),
    (15352usize, "PMCCNTSVR_EL1"),
    (15360usize, "PMXEVCNTR_EL0"),
    (15368usize, "SP_mon"),
    (15376usize, "TTBCR_S"),
    (15384usize, "ICH_VMCR_EL2"),
    (15392usize, "_FPSCR"),
    (15400usize, "ICV_RPR_EL1"),
    (15408usize, "AFSR1_EL2"),
    (15416usize, "ACTLR_S"),
    (15424usize, "FEAT_LPA_IMPLEMENTED"),
    (15432usize, "EDPFR"),
    (15440usize, "FEAT_ETMv4p4_IMPLEMENTED"),
    (15448usize, "SPESamplePreviousBranchAddress"),
    (15456usize, "PMINTENCLR_EL1"),
    (15464usize, "EDLSR"),
    (15472usize, "MPAMVPM2_EL2"),
    (15480usize, "AMPIDR1"),
    (15488usize, "RTPIDEN"),
    (15496usize, "FEAT_DotProd_IMPLEMENTED"),
    (15504usize, "GICR_PENDBASER"),
    (15512usize, "_ID_ISAR2"),
    (15520usize, "GICC_IAR"),
    (15528usize, "_MAIR1_S"),
    (15536usize, "_ICC_BPR0"),
    (15544usize, "SPSR_fiq"),
    (15552usize, "AMCR_EL0"),
    (15560usize, "FEAT_DPB_IMPLEMENTED"),
    (15568usize, "_SCTLR_NS"),
    (15576usize, "ICC_IAR0_EL1"),
    (15584usize, "FPSID"),
    (15592usize, "FEAT_CSV3_IMPLEMENTED"),
    (15600usize, "FEAT_S1POE_IMPLEMENTED"),
    (15608usize, "FEAT_LSMAOC_IMPLEMENTED"),
    (15616usize, "GCSCRE0_EL1"),
    (15624usize, "AMIIDR"),
    (15632usize, "__block_bbm_implemented"),
    (15648usize, "_ERXCTLR"),
    (15656usize, "GICC_CTLR"),
    (15664usize, "have_exception"),
    (15672usize, "CPTR_EL3_EZ_VALUE"),
    (15688usize, "R2"),
    (15696usize, "ACTLR_EL3"),
    (15704usize, "FEAT_VPIPT_IMPLEMENTED"),
    (15712usize, "_ICC_HPPIR0"),
    (15720usize, "PMBIDR_EL1"),
    (15728usize, "CTIITCTRL"),
    (15736usize, "VMECID_A_EL2"),
    (15744usize, "_HAMAIR1"),
    (15752usize, "SPSR_EL2"),
    (15760usize, "current_exception"),
    (15768usize, "LORSA_EL1"),
    (15776usize, "TCR2_EL2"),
    (15784usize, "APDBKeyLo_EL1"),
    (15792usize, "RVBAR_EL3"),
    (15800usize, "PMPIDR0"),
    (15808usize, "_ICH_LR"),
    (15872usize, "__clock_divider"),
    (15888usize, "PMCCFILTR_EL0"),
    (15896usize, "OSDTRRX_EL1"),
    (15904usize, "DBGDSAR"),
    (15912usize, "_VPIDR"),
    (15920usize, "CNTID"),
    (15928usize, "FEAT_SVE2_IMPLEMENTED"),
    (15936usize, "FEAT_SME2_IMPLEMENTED"),
    (15944usize, "HEAP_BASE"),
    (15952usize, "FEAT_ETMv4p2_IMPLEMENTED"),
    (15960usize, "__mecid_width"),
    (15968usize, "BRBTGT_EL1"),
    (16224usize, "GICV_HPPIR"),
    (16232usize, "FEAT_PMUv3_IMPLEMENTED"),
    (16240usize, "FEAT_SSBS_IMPLEMENTED"),
    (16248usize, "_HDFAR"),
    (16256usize, "_ICV_IAR1"),
    (16264usize, "ISR_EL1"),
    (16272usize, "FEAT_nTLBPA_IMPLEMENTED"),
    (16280usize, "FAR_EL1"),
    (16288usize, "RVBAR_EL1"),
    (16296usize, "_CNTKCTL"),
    (16304usize, "TPIDR_EL3"),
    (16312usize, "ID_PFR0_EL1"),
    (16320usize, "FEAT_RPRES_IMPLEMENTED"),
    (16328usize, "_PRRR_NS"),
    (16336usize, "FEAT_TCR2_IMPLEMENTED"),
    (16344usize, "_ICC_IAR0"),
    (16352usize, "FEAT_SHA1_IMPLEMENTED"),
    (16360usize, "FEAT_AA32HPD_IMPLEMENTED"),
    (16368usize, "FEAT_LSE2_IMPLEMENTED"),
    (16376usize, "CFG_RMR_AA64"),
    (16384usize, "_PMCNTENSET"),
    (16392usize, "ICC_SRE_EL2"),
    (16400usize, "HFGWTR2_EL2"),
    (16408usize, "PMPIDR3"),
    (16416usize, "_DBGBVR"),
    (16480usize, "SCTLR_S"),
    (16488usize, "FEAT_FHM_IMPLEMENTED"),
    (16496usize, "EDWAR"),
    (16504usize, "R1"),
    (16512usize, "_CONTEXTIDR_NS"),
    (16520usize, "AFSR0_EL1"),
    (16528usize, "RCWSMASK_EL1"),
    (16544usize, "SCXTNUM_EL2"),
    (16552usize, "ERXPFGCDN_EL1"),
    (16560usize, "BRBFCR_EL1"),
    (16568usize, "__impdef_TG0"),
    (16576usize, "SPMSELR_EL0"),
    (16584usize, "_PMUSERENR"),
    (16592usize, "FCSEIDR"),
    (16600usize, "GICD_SETSPI_SR"),
    (16608usize, "DACR32_EL2"),
    (16616usize, "HFGRTR_EL2"),
    (16624usize, "TPIDRURO_S"),
    (16632usize, "FEAT_Debugv8p9_IMPLEMENTED"),
    (16640usize, "FEAT_MEC_IMPLEMENTED"),
    (16648usize, "MPAM0_EL1"),
    (16656usize, "FEAT_TLBIOS_IMPLEMENTED"),
    (16664usize, "CNTHP_CVAL_EL2"),
    (16672usize, "GPCCR_EL3"),
    (16680usize, "AFSR0_EL3"),
    (16688usize, "AMEVCNTVOFF1_EL2"),
    (16816usize, "_AMUSERENR"),
    (16824usize, "_ICC_EOIR1"),
    (16832usize, "EDCIDR3"),
    (16840usize, "DBGDIDR"),
    (16848usize, "FEAT_LVA_IMPLEMENTED"),
    (16856usize, "MDCCSR_EL0"),
    (16864usize, "CPTR_EL3"),
    (16872usize, "CNTP_CVAL_S"),
    (16880usize, "AIDR_EL1"),
    (16888usize, "_AMCNTENSET0"),
    (16896usize, "_DACR_NS"),
    (16904usize, "EDLAR"),
    (16912usize, "FEAT_AA64EL1_IMPLEMENTED"),
    (16920usize, "_ICH_AP0R"),
    (16936usize, "ERRnFR"),
    (16968usize, "R15"),
    (16976usize, "_PMCCFILTR"),
    (16984usize, "PMCFGR"),
    (16992usize, "PSTATE"),
    (17024usize, "EDDEVARCH"),
    (17032usize, "_ID_ISAR1"),
    (17040usize, "TCMTR"),
    (17048usize, "EDHSR"),
    (17056usize, "__CNTReadBase"),
    (17064usize, "ICC_IGRPEN1_EL1_NS"),
    (17072usize, "GICH_VTR"),
    (17080usize, "GICD_SGIR"),
    (17088usize, "FEAT_AdvSIMD_IMPLEMENTED"),
    (17096usize, "SCTLR_EL3"),
    (17104usize, "_ERXMISC3"),
    (17112usize, "_ELR_hyp"),
    (17120usize, "_PMSELR"),
    (17128usize, "R19"),
    (17136usize, "CNTHVS_TVAL_EL2"),
    (17144usize, "AIFSR_S"),
    (17152usize, "_PMCEID2"),
    (17160usize, "SPESampleClass"),
    (17168usize, "NIDEN"),
    (17176usize, "VBAR_EL1"),
    (17184usize, "FEAT_ECBHB_IMPLEMENTED"),
    (17192usize, "ICC_HPPIR1_EL1"),
    (17200usize, "ICH_ELRSR_EL2"),
    (17208usize, "FEAT_MOPS_IMPLEMENTED"),
    (17216usize, "CLIDR_EL1"),
    (17224usize, "CNTV_CTL_EL0"),
    (17232usize, "_MAIR1_NS"),
    (17240usize, "FEAT_SPE_IMPLEMENTED"),
    (17248usize, "ELR_EL2"),
    (17256usize, "DBGDTRTX_EL0"),
    (17264usize, "TPIDRRO_EL0"),
    (17272usize, "ICC_EOIR1_EL1"),
    (17280usize, "PMCIDR0"),
    (17288usize, "FEAT_SME_I16I64_IMPLEMENTED"),
    (17296usize, "FEAT_FP_IMPLEMENTED"),
    (17304usize, "FEAT_MTE_ASYM_FAULT_IMPLEMENTED"),
    (17312usize, "FEAT_SPE_CRR_IMPLEMENTED"),
    (17320usize, "FEAT_TRBE_IMPLEMENTED"),
    (17328usize, "SMCR_EL1"),
    (17336usize, "MPAMVPMV_EL2"),
    (17344usize, "_VDISR"),
    (17352usize, "ICC_BPR0_EL1"),
    (17360usize, "ID_ISAR0_EL1"),
    (17368usize, "ICC_BPR1_EL1_NS"),
    (17376usize, "ICH_VTR_EL2"),
    (17384usize, "HDFGWTR_EL2"),
    (17392usize, "FEAT_MTE_PERM_IMPLEMENTED"),
    (17400usize, "MPIDR_EL1"),
    (17408usize, "PMPCSR"),
    (17416usize, "_ICC_SGI0R"),
    (17424usize, "AMEVCNTVOFF0_EL2"),
    (17552usize, "ERXFR_EL1"),
    (17560usize, "GICR_VPENDBASER"),
    (17568usize, "_ICC_BPR1_NS"),
    (17576usize, "SPESampleDataSource"),
    (17584usize, "GICM_SETSPI_NSR"),
    (17592usize, "NUM_GIC_LIST_REGS"),
    (17608usize, "_PMINTENCLR"),
    (17616usize, "GICM_TYPER"),
    (17624usize, "FEAT_Debugv8p8_IMPLEMENTED"),
    (17632usize, "MPAMHCR_EL2"),
    (17640usize, "SPESampleTimestampValid"),
    (17648usize, "FEAT_CMOW_IMPLEMENTED"),
    (17656usize, "FEAT_ETEv1p3_IMPLEMENTED"),
    (17664usize, "v8Ap1_IMPLEMENTED"),
    (17672usize, "_DBGDSCRext"),
    (17680usize, "MAIR_EL3"),
    (17688usize, "HDFGWTR2_EL2"),
    (17696usize, "FEAT_ABLE_IMPLEMENTED"),
    (17704usize, "GICV_IAR"),
    (17712usize, "_PMOVS"),
    (17720usize, "CTIPIDR2"),
    (17728usize, "v8Ap8_IMPLEMENTED"),
    (17736usize, "FEAT_RME_IMPLEMENTED"),
    (17744usize, "_DBGDRAR"),
    (17752usize, "GITS_PARTIDR"),
    (17760usize, "_P"),
    (18272usize, "GCSPR_EL3"),
    (18280usize, "FEAT_ASMv8p2_IMPLEMENTED"),
    (18288usize, "__VLPI_base"),
    (18296usize, "BRBCR_EL2"),
    (18304usize, "__unpred_tsize_aborts"),
    (18312usize, "CNTCR"),
    (18320usize, "CNTHP_TVAL_EL2"),
    (18328usize, "_ICV_HPPIR1"),
    (18336usize, "ELR_EL1"),
    (18344usize, "R4"),
    (18352usize, "__ICACHE_CCSIDR_RESET"),
    (18408usize, "_HSCTLR"),
    (18416usize, "ICC_CTLR_EL1_S"),
    (18424usize, "_TPIDRURO_NS"),
    (18432usize, "_ERXADDR2"),
    (18440usize, "MDSELR_EL1"),
    (18448usize, "SPSR_und"),
    (18456usize, "TTBR1_EL2"),
    (18472usize, "_VTTBR_EL2"),
    (18488usize, "SPESampleCounter"),
    (19000usize, "GICH_VMCR"),
    (19008usize, "CTILAR"),
    (19016usize, "PMDEVTYPE"),
    (19024usize, "GICC_EOIR"),
    (19032usize, "ID_ISAR4_EL1"),
    (19040usize, "FEAT_CSV2_2_IMPLEMENTED"),
    (19048usize, "FEAT_SYSREG128_IMPLEMENTED"),
    (19056usize, "R9"),
    (19064usize, "SPESampleOpType"),
    (19072usize, "CTIPIDR0"),
    (19080usize, "CTR_EL0"),
    (19088usize, "SPMACCESSR_EL2"),
    (19096usize, "FEAT_CSV2_3_IMPLEMENTED"),
    (19104usize, "FEAT_SPMU_IMPLEMENTED"),
    (19112usize, "__tlb_enabled"),
    (19120usize, "_VBAR_NS"),
    (19128usize, "MAIR2_EL3"),
    (19136usize, "R14"),
    (19144usize, "TTBR1_S"),
    (19152usize, "v8Ap5_IMPLEMENTED"),
    (19160usize, "PMSELR_EL0"),
    (19168usize, "HDFGRTR_EL2"),
    (19176usize, "AMEVTYPER1_EL0"),
    (19304usize, "CNTHV_CTL_EL2"),
    (19312usize, "ICC_RPR_EL1"),
    (19320usize, "AMDEVARCH"),
    (19328usize, "GCSCR_EL2"),
    (19336usize, "EDPCSR"),
    (19344usize, "_ERXFR2"),
    (19352usize, "VDISR_EL2"),
    (19360usize, "FEAT_MTE_ASYNC_IMPLEMENTED"),
    (19368usize, "_CNTP_CVAL_NS"),
    (19376usize, "DBGDEVID2"),
    (19384usize, "NUM_WATCHPOINTS"),
    (19400usize, "CNTSR"),
    (19408usize, "AMCIDR1"),
    (19416usize, "DBGWVR_EL1"),
    (19928usize, "ICH_AP1R_EL2"),
    (19960usize, "FEAT_FCMA_IMPLEMENTED"),
    (19968usize, "FEAT_GICv3p1_IMPLEMENTED"),
    (19976usize, "__syncAbortOnTTWCache"),
    (19984usize, "FEAT_S1PIE_IMPLEMENTED"),
    (19992usize, "OSECCR_EL1"),
    (20000usize, "FEAT_ETMv4p5_IMPLEMENTED"),
    (20008usize, "PRRR_S"),
    (20016usize, "ICC_MSRE"),
    (20024usize, "_ERXMISC5"),
    (20032usize, "PFAR_EL2"),
    (20040usize, "CTICIDR1"),
    (20048usize, "TTBR1_NS"),
    (20056usize, "SPSR_abt"),
    (20064usize, "_ICV_IAR0"),
    (20072usize, "MAIR2_EL1"),
    (20080usize, "FEAT_MTE_NO_ADDRESS_TAGS_IMPLEMENTED"),
    (20088usize, "R21"),
    (20096usize, "MDCCINT_EL1"),
    (20104usize, "AMCIDR2"),
    (20112usize, "_ICH_HCR"),
    (20120usize, "RGSR_EL1"),
    (20128usize, "_MIDR"),
    (20136usize, "ID_AA64DFR0_EL1"),
    (20144usize, "_ID_PFR1"),
    (20152usize, "ELR_EL3"),
    (20160usize, "__syncAbortOnSoRead"),
    (20168usize, "ID_AA64AFR1_EL1"),
    (20176usize, "FEAT_AA64EL0_IMPLEMENTED"),
    (20184usize, "SPESampleContextEL1Valid"),
    (20192usize, "FEAT_EBEP_IMPLEMENTED"),
    (20200usize, "EDECR"),
    (20208usize, "GICR_VPROPBASER"),
    (20216usize, "_CSSELR_NS"),
    (20224usize, "_MVFR0"),
    (20232usize, "AMAIR1_S"),
    (20240usize, "ID_MMFR5_EL1"),
    (20248usize, "PMCIDR3"),
    (20256usize, "_DBGCLAIMCLR"),
    (20264usize, "_ADFSR_NS"),
    (20272usize, "v8Ap6_IMPLEMENTED"),
    (20280usize, "_HPFAR"),
    (20288usize, "EDPIDR0"),
    (20296usize, "_DBGOSLSR"),
    (20304usize, "PIRE0_EL1"),
    (20312usize, "FEAT_LRCPC3_IMPLEMENTED"),
    (20320usize, "FEAT_SVE_AES_IMPLEMENTED"),
    (20328usize, "SPSR_EL3"),
    (20336usize, "GICM_CLRSPI_SR"),
    (20344usize, "__syncAbortOnWriteNormCache"),
    (20352usize, "CP15SDISABLE2"),
    (20360usize, "FEAT_CRC32_IMPLEMENTED"),
    (20368usize, "FEAT_TTST_IMPLEMENTED"),
    (20376usize, "TTBCR2_S"),
    (20384usize, "_ICC_IGRPEN0"),
    (20392usize, "R20"),
    (20400usize, "CNTPS_CTL_EL1"),
    (20408usize, "_HTPIDR"),
    (20416usize, "GICR_PARTIDR"),
    (20424usize, "FEAT_PMUv3_EXT_IMPLEMENTED"),
    (20432usize, "R13"),
    (20440usize, "ID_DFR0_EL1"),
    (20448usize, "GICD_CLRSPI_SR"),
    (20456usize, "PMMIR_EL1"),
    (20464usize, "DBGEN"),
    (20472usize, "FEAT_IESB_IMPLEMENTED"),
    (20480usize, "FEAT_BTI_IMPLEMENTED"),
    (20488usize, "ICC_SGI1R_EL1"),
    (20496usize, "R30"),
    (20504usize, "PMBLIMITR_EL1"),
    (20512usize, "_TPIDRPRW_NS"),
    (20520usize, "FEAT_GTG_IMPLEMENTED"),
    (20528usize, "_CNTHV_CTL"),
    (20536usize, "GITS_MPAMIDR"),
    (20544usize, "_DBGDTRRXint"),
    (20552usize, "FEAT_AA32EL0_IMPLEMENTED"),
    (20560usize, "FEAT_DoubleFault_IMPLEMENTED"),
    (20568usize, "__isla_vector_gpr"),
    (20576usize, "__GICCPUInterfaceBase"),
    (20584usize, "RC"),
    (20624usize, "VMECID_P_EL2"),
    (20632usize, "__GIC_Pending"),
    (20640usize, "ICC_DIR_EL1"),
    (20648usize, "GPTBR_EL3"),
    (20656usize, "_ICC_EOIR0"),
    (20664usize, "_MAIR0_S"),
    (20672usize, "_ICC_SRE_S"),
    (20680usize, "FEAT_SPECRES2_IMPLEMENTED"),
    (20688usize, "__mops_forward_copy"),
    (20696usize, "VMPIDR_EL2"),
    (20704usize, "_ICV_BPR0"),
    (20712usize, "FEAT_PMUv3_SS_IMPLEMENTED"),
    (20720usize, "FPSR"),
    (20728usize, "_HIFAR"),
    (20736usize, "_ICV_EOIR1"),
    (20744usize, "_HMAIR1"),
    (20752usize, "SPESamplePreviousBranchAddressValid"),
    (20760usize, "Branchtypetaken"),
    (20768usize, "ICV_AP1R_EL1"),
    (20800usize, "AMAIR2_EL3"),
    (20808usize, "SCTLR_EL2"),
    (20816usize, "VPIDR_EL2"),
    (20824usize, "CNTP_CVAL_EL0"),
    (20832usize, "_ICV_AP0R"),
    (20848usize, "NUM_BRBE_RECORDS"),
    (20864usize, "GCSPR_EL0"),
    (20872usize, "__has_sve_extended_bf16"),
    (20888usize, "v8Ap2_IMPLEMENTED"),
    (20896usize, "_ACTLR2_NS"),
    (20904usize, "SPESampleContextEL2"),
    (20912usize, "AMEVTYPER0_EL0"),
    (20944usize, "SCR"),
    (20952usize, "MAIR2_EL2"),
    (20960usize, "GICC_STATUSR"),
    (20968usize, "ID_AA64MMFR4_EL1"),
    (20976usize, "BTypeCompatible"),
    (20984usize, "FEAT_S2PIE_IMPLEMENTED"),
    (20992usize, "_DBGOSDLR"),
    (21000usize, "DBGAUTHSTATUS_EL1"),
    (21008usize, "MPAMVPM7_EL2"),
    (21016usize, "ICH_HCR_EL2"),
    (21024usize, "GICV_DIR"),
    (21032usize, "FEAT_EBF16_IMPLEMENTED"),
    (21040usize, "PMCR_EL0"),
    (21048usize, "FPEXC32_EL2"),
    (21056usize, "ICV_HPPIR1_EL1"),
    (21064usize, "FEAT_FP16_IMPLEMENTED"),
    (21072usize, "_TRFCR"),
    (21080usize, "__empam_sdeflt_implemented"),
    (21088usize, "CNTHV_TVAL_EL2"),
    (21096usize, "PMSCR_EL1"),
    (21104usize, "ID_AFR0_EL1"),
    (21112usize, "DBGCLAIMCLR_EL1"),
    (21120usize, "APIAKeyLo_EL1"),
    (21128usize, "FEAT_UAO_IMPLEMENTED"),
    (21136usize, "SDER32_EL2"),
    (21144usize, "EDDFR1"),
    (21152usize, "FEAT_GICv3_NMI_IMPLEMENTED"),
    (21160usize, "SPSR_mon"),
    (21168usize, "__mpam_has_altsp"),
    (21176usize, "ICV_AP0R_EL1"),
    (21208usize, "SCXTNUM_EL3"),
    (21216usize, "__mpam_vpmr_max"),
    (21224usize, "R18"),
    (21232usize, "__SGI_base"),
    (21240usize, "R0"),
    (21248usize, "v9Ap3_IMPLEMENTED"),
    (21256usize, "__apply_effective_shareability"),
    (21264usize, "Records_SRC"),
    (21776usize, "_DFAR_S"),
    (21784usize, "HAFGRTR_EL2"),
    (21792usize, "__syncAbortOnReadNormCache"),
    (21800usize, "LOREA_EL1"),
    (21808usize, "AMAIR2_EL1"),
    (21816usize, "ERRSELR_EL1"),
    (21824usize, "ICC_MCTLR"),
    (21832usize, "__mpam_partid_max"),
    (21840usize, "FEAT_RDM_IMPLEMENTED"),
    (21848usize, "__syncAbortOnDeviceWrite"),
    (21856usize, "FEAT_ETMv4p6_IMPLEMENTED"),
    (21864usize, "R27"),
    (21872usize, "_DormantCtlReg"),
    (21880usize, "_ID_MMFR0"),
    (21888usize, "_ERXADDR"),
    (21896usize, "EDITCTRL"),
    (21904usize, "__ignore_rvbar_in_aarch32"),
    (21912usize, "CNTP_CTL_S"),
    (21920usize, "FEAT_EL2_IMPLEMENTED"),
    (21928usize, "CTICONTROL"),
    (21936usize, "GCSPR_EL1"),
    (21944usize, "__currentCond"),
    (21952usize, "BRBSRCINJ_EL1"),
    (21960usize, "CONTEXTIDR_S"),
    (21968usize, "GITS_STATUSR"),
    (21976usize, "_HCR2"),
    (21984usize, "AMCIDR0"),
    (21992usize, "EventRegister"),
    (22000usize, "FEAT_ETS2_IMPLEMENTED"),
    (22008usize, "_DBGPRCR"),
    (22016usize, "_DLR"),
    (22024usize, "FEAT_SME_IMPLEMENTED"),
    (22032usize, "__SPE_LFSR"),
    (22040usize, "CNTSCR"),
    (22048usize, "_AMEVCNTR0_EL0"),
    (22080usize, "CNTKCTL_EL1"),
    (22088usize, "__isb_is_branch"),
    (22096usize, "GICR_MPAMIDR"),
    (22104usize, "LORID_EL1"),
    (22112usize, "_ICC_SRE_NS"),
    (22120usize, "_ICV_IGRPEN0"),
    (22128usize, "FEAT_DPB2_IMPLEMENTED"),
    (22136usize, "ID_AA64MMFR3_EL1"),
    (22144usize, "BRBINF_EL1"),
    (22400usize, "GICH_ELRSR"),
    (22408usize, "GICH_MISR"),
    (22416usize, "TCR_EL1"),
    (22424usize, "CNTVOFF_EL2"),
    (22432usize, "VTTBR"),
    (22440usize, "SPESampleInFlight"),
    (22448usize, "REVIDR_EL1"),
    (22456usize, "_DBGBXVR"),
    (22520usize, "TPIDRURW_S"),
    (22528usize, "AMCIDR3"),
    (22536usize, "FEAT_XS_IMPLEMENTED"),
    (22544usize, "MPAMVPM4_EL2"),
    (22552usize, "HCRX_EL2"),
    (22560usize, "OSDTRTX_EL1"),
    (22568usize, "MPAMVPM6_EL2"),
    (22576usize, "ID_AA64PFR1_EL1"),
    (22584usize, "ERXPFGF_EL1"),
    (22592usize, "FEAT_NV2_IMPLEMENTED"),
    (22600usize, "FEAT_HAFDBS_IMPLEMENTED"),
    (22608usize, "FEAT_PAuth_IMPLEMENTED"),
    (22616usize, "ICH_EISR_EL2"),
    (22624usize, "ERXMISC0_EL1"),
    (22632usize, "JOSCR"),
    (22640usize, "AMAIR2_EL2"),
    (22648usize, "PMAUTHSTATUS"),
    (22656usize, "PMCNTENCLR_EL0"),
    (22664usize, "__last_cycle_count"),
    (22680usize, "FEAT_F64MM_IMPLEMENTED"),
    (22688usize, "FEAT_PAuth2_IMPLEMENTED"),
    (22696usize, "CNTHPS_CVAL_EL2"),
    (22704usize, "__trcclaim_tags"),
    (22712usize, "AFSR1_EL1"),
    (22720usize, "_AMCNTENCLR1"),
    (22728usize, "GICD_SETSPI_NSR"),
    (22736usize, "MDCR_EL3"),
    (22744usize, "_VMPIDR"),
    (22752usize, "GICV_AHPPIR"),
    (22760usize, "AMPIDR0"),
    (22768usize, "PMSEVFR_EL1"),
    (22776usize, "v8Ap7_IMPLEMENTED"),
    (22784usize, "__InstructionStep"),
    (22792usize, "FEAT_SVE2p1_IMPLEMENTED"),
    (22800usize, "NUM_BREAKPOINTS"),
    (22816usize, "AMCNTENCLR0_EL0"),
    (22824usize, "EDDFR"),
    (22832usize, "__SPE_LFSR_initialized"),
    (22840usize, "VBAR_EL2"),
    (22848usize, "VSTTBR_EL2"),
    (22856usize, "EDVIDSR"),
    (22864usize, "PMZR_EL0"),
    (22872usize, "ADFSR_S"),
    (22880usize, "_ID_PFR2"),
    (22888usize, "_ICC_AP1R_S"),
    (22904usize, "_ICC_SGI1R"),
    (22912usize, "_CNTFRQ"),
    (22920usize, "CSSELR_EL1"),
    (22928usize, "MECID_P0_EL2"),
    (22936usize, "CNTFRQ_EL0"),
    (22944usize, "MAIR_EL1"),
    (22952usize, "R5"),
    (22960usize, "_HRMR"),
    (22968usize, "_HACTLR2"),
    (22976usize, "ESR_EL1"),
    (22984usize, "ICC_SRE_EL1_NS"),
    (22992usize, "_PAR_EL1"),
    (23008usize, "R3"),
    (23016usize, "ShouldAdvanceSS"),
    (23024usize, "FEAT_SME_F64F64_IMPLEMENTED"),
    (23032usize, "BRBTS_EL1"),
    (23040usize, "_ICV_AP1R"),
    (23056usize, "FEAT_MTE4_IMPLEMENTED"),
    (23064usize, "_DBGDSCRint"),
    (23072usize, "_DSPSR2"),
    (23080usize, "SPESampleCounterValid"),
    (23112usize, "_DISR"),
    (23120usize, "R26"),
    (23128usize, "VBAR_EL3"),
    (23136usize, "MECID_A1_EL2"),
    (23144usize, "RMR_EL2"),
    (23152usize, "_ID_DFR1"),
    (23160usize, "_ICV_PMR"),
    (23168usize, "_CNTV_CVAL"),
    (23176usize, "R10"),
    (23184usize, "FEAT_BF16_IMPLEMENTED"),
    (23192usize, "FEAT_THE_IMPLEMENTED"),
    (23200usize, "TTBR0_EL3"),
    (23208usize, "ICC_IAR1_EL1"),
    (23216usize, "R16"),
    (23224usize, "_PMOVSSET"),
    (23232usize, "_DBGDTRTXext"),
    (23240usize, "CTICIDR3"),
    (23248usize, "FEAT_PMUv3_EXT64_IMPLEMENTED"),
    (23256usize, "FEAT_SEBEP_IMPLEMENTED"),
    (23264usize, "_REVIDR"),
    (23272usize, "FEAT_I8MM_IMPLEMENTED"),
    (23280usize, "__CNTEL0BaseN"),
    (23288usize, "FEAT_ETE_IMPLEMENTED"),
    (23296usize, "__GICDistBase"),
    (23304usize, "CCSIDR_EL1"),
    (23312usize, "FEAT_EPAC_IMPLEMENTED"),
    (23320usize, "_DBGWVR"),
    (23384usize, "__feat_rpres"),
    (23392usize, "ID_ISAR3_EL1"),
    (23400usize, "__gmid_log2_block_size"),
    (23416usize, "GICM_SETSPI_SR"),
    (23424usize, "GITS_SGIR"),
    (23432usize, "__PMUBase"),
    (23440usize, "_VDFSR"),
    (23448usize, "TPIDR_EL0"),
    (23456usize, "EDDEVID"),
    (23464usize, "GICV_EOIR"),
    (23472usize, "ICV_DIR_EL1"),
    (23480usize, "_HTCR"),
    (23488usize, "_PMEVTYPER"),
    (23616usize, "ERXPFGCTL_EL1"),
    (23624usize, "_PMCEID1"),
    (23632usize, "_AMCNTENCLR0"),
    (23640usize, "RCWMASK_EL1"),
    (23656usize, "CNTV_CVAL_EL0"),
    (23664usize, "__cpy_mops_option_a_supported"),
    (23672usize, "BRBSRC_EL1"),
    (23928usize, "GITS_IIDR"),
    (23936usize, "R24"),
    (23944usize, "FEAT_CSV2_IMPLEMENTED"),
    (23952usize, "RNDR"),
    (23960usize, "__syncAbortOnSoWrite"),
    (23968usize, "GICM_IIDR"),
    (23976usize, "_ZA"),
    (89512usize, "GICD_TYPER"),
    (89520usize, "RMR_EL1"),
    (89528usize, "GICC_PMR"),
    (89536usize, "FEAT_MTE_IMPLEMENTED"),
    (89544usize, "FEAT_MPAMv0p1_IMPLEMENTED"),
    (89552usize, "__cpyf_mops_option_a_supported"),
    (89560usize, "ICV_EOIR1_EL1"),
    (89568usize, "ICC_MGRPEN1"),
    (89576usize, "_ERXCTLR2"),
    (89584usize, "PIR_EL2"),
    (89592usize, "FEAT_SPECRES_IMPLEMENTED"),
    (89600usize, "_CNTHP_CTL"),
    (89608usize, "FEAT_TRBE_EXT_IMPLEMENTED"),
    (89616usize, "RVBAR_EL2"),
    (89624usize, "_ID_MMFR2"),
    (89632usize, "ID_MMFR0_EL1"),
    (89640usize, "FEAT_XNX_IMPLEMENTED"),
    (89648usize, "AMAIR_EL1"),
    (89656usize, "PMUEventAccumulator"),
    (90152usize, "SP_EL0"),
    (90160usize, "_ICH_VMCR"),
    (90168usize, "__mpam_major"),
    (90176usize, "FEAT_E0PD_IMPLEMENTED"),
    (90184usize, "EDPIDR4"),
    (90192usize, "MECID_P1_EL2"),
    (90200usize, "_DBGBCR"),
    (90264usize, "FEAT_GICv3_LEGACY_IMPLEMENTED"),
    (90272usize, "SMPRIMAP_EL2"),
    (90280usize, "__supported_pa_size"),
    (90296usize, "SCTLR_EL1"),
    (90304usize, "__syncAbortOnDeviceRead"),
    (90312usize, "FEAT_Debugv8p1_IMPLEMENTED"),
    (90320usize, "FEAT_TME_IMPLEMENTED"),
    (90328usize, "DBGPRCR_EL1"),
    (90336usize, "ID_MMFR4_EL1"),
    (90344usize, "PMINTENSET_EL1"),
    (90352usize, "v8Ap4_IMPLEMENTED"),
    (90360usize, "_CNTHPS_CTL"),
    (90368usize, "ICV_BPR0_EL1"),
    (90376usize, "CPTR_EL3_ESM_VALUE"),
    (90392usize, "FEAT_AFP_IMPLEMENTED"),
    (90400usize, "GITS_CWRITER"),
    (90408usize, "_ICC_IGRPEN1_NS"),
    (90416usize, "__mpam_has_hcr"),
    (90424usize, "__empam_force_ns_RAO"),
    (90432usize, "ID_ISAR6_EL1"),
    (90440usize, "SCXTNUM_EL0"),
    (90448usize, "MVFR0_EL1"),
    (90456usize, "_DFAR_NS"),
    (90464usize, "_HACR"),
    (90472usize, "FEAT_PMUv3p8_IMPLEMENTED"),
    (90480usize, "_DBGCLAIMSET"),
    (90488usize, "GICR_INMIR0"),
    (90496usize, "NUM_AMU_CG0_MONITORS"),
    (90512usize, "CTIPIDR4"),
    (90520usize, "AMUSERENR_EL0"),
    (90528usize, "MPAM2_EL2"),
    (90536usize, "PMBPTR_EL1"),
    (90544usize, "_ZT0"),
    (90608usize, "FEAT_SVE_SHA3_IMPLEMENTED"),
    (90616usize, "_HSTR"),
    (90624usize, "ID_AA64MMFR2_EL1"),
    (90632usize, "ID_AA64ISAR0_EL1"),
    (90640usize, "_DBGOSECCR"),
    (90648usize, "AMPIDR4"),
    (90656usize, "ICC_SGI0R_EL1"),
    (90664usize, "BRBCR_EL1"),
    (90672usize, "SPSR_EL1"),
    (90680usize, "_PMCR"),
    (90688usize, "_ICC_IGRPEN1_S"),
    (90696usize, "_ICH_EISR"),
    (90704usize, "__GIC_Active"),
    (90712usize, "ESR_EL2"),
    (90720usize, "FEAT_PAN2_IMPLEMENTED"),
    (90728usize, "SCR_EL3"),
    (90736usize, "PAR_S"),
    (90744usize, "FEAT_WFxT_IMPLEMENTED"),
    (90752usize, "ID_MMFR3_EL1"),
    (90760usize, "CSSELR_S"),
    (90768usize, "_ICC_HSRE"),
    (90776usize, "CNTNSAR"),
    (90784usize, "FEAT_PMUv3_TH_IMPLEMENTED"),
    (90792usize, "FEAT_HBC_IMPLEMENTED"),
    (90800usize, "FEAT_SME_F16F16_IMPLEMENTED"),
    (90808usize, "NUM_AMU_CG1_MONITORS"),
    (90824usize, "OSLAR_EL1"),
    (90832usize, "MECIDR_EL2"),
    (90840usize, "MVFR2_EL1"),
    (90848usize, "_PMCEID3"),
    (90856usize, "CNTP_CTL_EL0"),
    (90864usize, "FEAT_CLRBHB_IMPLEMENTED"),
    (90872usize, "FEAT_MTE2_IMPLEMENTED"),
    (90880usize, "_PMCNTENCLR"),
    (90888usize, "MPAMVPM3_EL2"),
    (90896usize, "ID_MMFR1_EL1"),
    (90904usize, "ICV_NMIAR1_EL1"),
    (90912usize, "FEAT_SVE_B16B16_IMPLEMENTED"),
    (90920usize, "v9Ap2_IMPLEMENTED"),
    (90928usize, "FEAT_FPACCOMBINE_IMPLEMENTED"),
    (90936usize, "BTypeNext"),
    (90944usize, "FEAT_MTE_CANONICAL_TAGS_IMPLEMENTED"),
    (90952usize, "SMCR_EL3_LEN_VALUE"),
    (90968usize, "ID_PFR1_EL1"),
    (90976usize, "_ERXMISC6"),
    (90984usize, "SMCR_EL3"),
    (90992usize, "SP_EL2"),
    (91000usize, "_ICV_EOIR0"),
    (91008usize, "FEAT_SVE_SM4_IMPLEMENTED"),
    (91016usize, "_CNTVOFF"),
    (91024usize, "__mte_implemented"),
    (91032usize, "CONTEXTIDR_EL2"),
    (91040usize, "SPSR_irq"),
    (91048usize, "_TTBR0_EL2"),
    (91064usize, "JMCR"),
    (91072usize, "ICV_IAR1_EL1"),
    (91080usize, "__empam_force_ns_implemented"),
    (91088usize, "_SPSR_hyp"),
    (91096usize, "ICC_AP0R_EL1"),
    (91128usize, "GICC_RPR"),
    (91136usize, "_HACTLR"),
    (91144usize, "GICR_ISENABLER0"),
    (91152usize, "SMPRI_EL1"),
    (91160usize, "TSTATE"),
    (100232usize, "MVBAR"),
    (100240usize, "CNTV_TVAL_EL0"),
    (100248usize, "MPAMVPM0_EL2"),
    (100256usize, "VariantImplemented"),
    (100272usize, "_DBGVCR"),
    (100280usize, "ID_AA64SMFR0_EL1"),
    (100288usize, "FEAT_PMULL_IMPLEMENTED"),
    (100296usize, "FEAT_PAN_IMPLEMENTED"),
    (100304usize, "MFAR_EL3"),
    (100312usize, "Records_INF"),
    (100824usize, "CTIPIDR3"),
    (100832usize, "FEAT_FPAC_IMPLEMENTED"),
    (100840usize, "GMID_EL1"),
    (100848usize, "VSESR_EL2"),
    (100856usize, "CNTHPS_TVAL_EL2"),
    (100864usize, "NMRR_S"),
    (100872usize, "_ID_MMFR4"),
    (100880usize, "_ICH_VTR"),
    (100888usize, "EDDEVID1"),
    (100896usize, "PMCIDR1"),
    (100904usize, "GICR_INVALLR"),
    (100912usize, "FEAT_EDHSR_IMPLEMENTED"),
    (100920usize, "FEAT_NV_IMPLEMENTED"),
    (100928usize, "FEAT_SYSINSTR128_IMPLEMENTED"),
    (100936usize, "CNTHP_CTL_EL2"),
    (100944usize, "APIBKeyHi_EL1"),
    (100952usize, "CNTP_TVAL_EL0"),
    (100960usize, "FEAT_S2FWB_IMPLEMENTED"),
    (100968usize, "FEAT_AA32EL2_IMPLEMENTED"),
    (100976usize, "R8"),
    (100984usize, "_ICC_CTLR_NS"),
    (100992usize, "_EDECCR"),
    (101000usize, "CCSIDR2_EL1"),
    (101008usize, "MPAMVPM5_EL2"),
    (101016usize, "HFGWTR_EL2"),
    (101024usize, "SMIDR_EL1"),
    (101032usize, "_ERXMISC2"),
    (101040usize, "FEAT_LS64_ACCDATA_IMPLEMENTED"),
    (101048usize, "FEAT_ITE_IMPLEMENTED"),
    (101056usize, "CTIDEVARCH"),
    (101064usize, "S2POR_EL1"),
    (101072usize, "GICD_CLRSPI_NSR"),
    (101080usize, "GCSCR_EL1"),
    (101088usize, "FEAT_GCS_IMPLEMENTED"),
    (101096usize, "FEAT_Debugv8p4_IMPLEMENTED"),
    (101104usize, "_TTBCR_NS"),
    (101112usize, "LORN_EL1"),
    (101120usize, "FEAT_PACQARMA3_IMPLEMENTED"),
    (101128usize, "_RMR"),
    (101136usize, "FEAT_PMUv3p7_IMPLEMENTED"),
    (101144usize, "R7"),
    (101152usize, "__emulator_termination_opcode"),
    (101168usize, "_PMOVSR"),
    (101176usize, "__monomorphize_writes"),
    (101184usize, "__ExclusiveMonitorSet"),
    (101192usize, "FEAT_FlagM_IMPLEMENTED"),
    (101200usize, "TLBTR"),
    (101208usize, "FEAT_SHA3_IMPLEMENTED"),
    (101216usize, "FEAT_TLBIRANGE_IMPLEMENTED"),
    (101224usize, "IsWFIsleep"),
    (101232usize, "PMSFCR_EL1"),
    (101240usize, "ICC_IGRPEN1_EL1_S"),
    (101248usize, "HDFGRTR2_EL2"),
    (101256usize, "CTIPIDR1"),
    (101264usize, "_MPIDR"),
    (101272usize, "Records_TGT"),
    (101784usize, "EDPIDR3"),
    (101792usize, "EDDEVID2"),
    (101800usize, "PMIAR_EL1"),
    (101808usize, "GICR_PROPBASER"),
    (101816usize, "v9Ap4_IMPLEMENTED"),
    (101824usize, "TTBR0_S"),
    (101832usize, "GICV_CTLR"),
    (101840usize, "PMSICR_EL1"),
    (101848usize, "ID_AA64PFR0_EL1"),
    (101856usize, "FEAT_TTL_IMPLEMENTED"),
    (101864usize, "FEAT_LS64_IMPLEMENTED"),
    (101872usize, "FEAT_HPDS_IMPLEMENTED"),
    (101880usize, "v8Ap9_IMPLEMENTED"),
    (101888usize, "_DBGDTRTXint"),
    (101896usize, "JIDR"),
    (101904usize, "DBGWFAR"),
    (101912usize, "GICV_AIAR"),
    (101920usize, "ZCR_EL1"),
    (101928usize, "FEAT_ETMv4_IMPLEMENTED"),
    (101936usize, "RMR_EL3"),
    (101944usize, "AMCNTENCLR1_EL0"),
    (101952usize, "PMEVCNTSVR_EL1"),
    (102200usize, "NUM_GIC_PRIORITY_BITS"),
    (102216usize, "_ICV_HPPIR0"),
    (102224usize, "PMLSR"),
    (102232usize, "DCZID_EL0"),
    (102240usize, "_ICV_IGRPEN1"),
    (102248usize, "__DCACHE_CCSIDR_RESET"),
    (102304usize, "FEAT_RPRFM_IMPLEMENTED"),
    (102312usize, "DBGVCR32_EL2"),
    (102320usize, "CTIDEVID"),
    (102328usize, "BRBTGTINJ_EL1"),
    (102336usize, "FEAT_DoubleLock_IMPLEMENTED"),
    (102344usize, "_ID_MMFR3"),
    (102352usize, "_SDER"),
    (102360usize, "FEAT_SM4_IMPLEMENTED"),
    (102368usize, "MPAMSM_EL1"),
    (102376usize, "FEAT_TRF_IMPLEMENTED"),
    (102384usize, "PIRE0_EL2"),
    (102392usize, "_ICC_HPPIR1"),
    (102400usize, "EDCIDR0"),
    (102408usize, "FEAT_CNTSC_IMPLEMENTED"),
    (102416usize, "__trickbox_enabled"),
    (102424usize, "AMPIDR3"),
    (102432usize, "FEAT_CCIDX_IMPLEMENTED"),
    (102440usize, "_ICC_DIR"),
    (102448usize, "PMLAR"),
    (102456usize, "FEAT_SM3_IMPLEMENTED"),
    (102464usize, "CFG_RVBAR"),
    (102472usize, "_FPEXC"),
    (102480usize, "ICV_BPR1_EL1"),
    (102488usize, "ACCDATA_EL1"),
    (102496usize, "ERXMISC2_EL1"),
    (102504usize, "FEAT_VHE_IMPLEMENTED"),
    (102512usize, "NSACR"),
    (102520usize, "__CTIBase"),
    (102528usize, "CTILSR"),
    (102536usize, "_ISR"),
    (102544usize, "InGuardedPage"),
    (102552usize, "ICC_BPR1_EL1_S"),
    (102560usize, "_ERRSELR"),
    (102568usize, "GICV_AEOIR"),
    (102576usize, "HCR_EL2"),
    (102584usize, "ID_ISAR2_EL1"),
    (102592usize, "MECID_RL_A_EL3"),
    (102600usize, "FEAT_EL0_IMPLEMENTED"),
    (102608usize, "DSPSR_EL0"),
    (102616usize, "FEAT_D128_IMPLEMENTED"),
    (102624usize, "_DFSR_NS"),
    (102632usize, "GICD_STATUSR"),
    (102640usize, "FAR_EL2"),
    (102648usize, "PMUSERENR_EL0"),
    (102656usize, "FEAT_SSBS2_IMPLEMENTED"),
    (102664usize, "_ID_ISAR5"),
    (102672usize, "SPESampleCounterPending"),
    (102704usize, "SCTLR2_EL2"),
    (102712usize, "POR_EL0"),
    (102720usize, "R12"),
    (102728usize, "FEAT_PRFMSLC_IMPLEMENTED"),
    (102736usize, "R22"),
    (102744usize, "_MVFR2"),
    (102752usize, "GICV_PMR"),
    (102760usize, "GICR_INVLPIR"),
    (102768usize, "_ACTLR_NS"),
    (102776usize, "FEAT_LOR_IMPLEMENTED"),
    (102784usize, "v9Ap1_IMPLEMENTED"),
    (102792usize, "R11"),
    (102800usize, "PMICNTSVR_EL1"),
    (102808usize, "HFGRTR2_EL2"),
    (102816usize, "__num_ctx_breakpoints"),
    (102832usize, "_ID_AFR0"),
    (102840usize, "_ConfigReg"),
    (102848usize, "PAR_NS"),
    (102856usize, "PMDEVID"),
    (102864usize, "PFAR_EL1"),
    (102872usize, "_HCR"),
    (102880usize, "OSDLR_EL1"),
    (102888usize, "FEAT_SPEv1p4_IMPLEMENTED"),
    (102896usize, "FeatureImpl"),
    (103160usize, "ICC_HPPIR0_EL1"),
    (103168usize, "ID_AA64MMFR1_EL1"),
    (103176usize, "CNTHV_CVAL_EL2"),
    (103184usize, "ID_MMFR2_EL1"),
    (103192usize, "_HCPTR"),
    (103200usize, "SCXTNUM_EL1"),
    (103208usize, "DBGDTRRX_EL0"),
    (103216usize, "__setg_mops_option_a_supported"),
    (103224usize, "_DSPSR"),
    (103232usize, "EDPRCR"),
    (103240usize, "FEAT_DIT_IMPLEMENTED"),
    (103248usize, "FEAT_MPAM_IMPLEMENTED"),
    (103256usize, "_ID_ISAR0"),
    (103264usize, "AMEVCNTR1_EL0"),
    (103392usize, "_HMAIR0"),
    (103400usize, "FEAT_AA32EL1_IMPLEMENTED"),
    (103408usize, "ERXSTATUS_EL1"),
    (103416usize, "GICH_HCR"),
    (103424usize, "DFSR_S"),
    (103432usize, "FEAT_GICv4p1_IMPLEMENTED"),
    (103440usize, "MIDR_EL1"),
    (103448usize, "DBGBVR_EL1"),
    (103960usize, "FEAT_RAS_IMPLEMENTED"),
    (103968usize, "PMSWINC_EL0"),
    (103976usize, "CNTPS_TVAL_EL1"),
    (103984usize, "PMCGCR0"),
    (103992usize, "FEAT_NMI_IMPLEMENTED"),
    (104000usize, "FEAT_LPA2_IMPLEMENTED"),
    (104008usize, "DBGWCR_EL1"),
    (104520usize, "PMICFILTR_EL0"),
    (104528usize, "FEAT_MTE3_IMPLEMENTED"),
    (104536usize, "_AMCNTENSET1"),
    (104544usize, "__g1_activity_monitor_implemented"),
    (104552usize, "FEAT_SPEv1p3_IMPLEMENTED"),
    (104560usize, "GICV_BPR"),
    (104568usize, "FEAT_TTCNP_IMPLEMENTED"),
    (104576usize, "FEAT_LRCPC2_IMPLEMENTED"),
    (104584usize, "_DBGDCCINT"),
    (104592usize, "SPESampleContextEL1"),
    (104600usize, "__CNTControlBase"),
    (104608usize, "GICD_IIDR"),
    (104616usize, "PMPIDR4"),
    (104624usize, "CTIDEVID2"),
    (104632usize, "FEAT_AA32BF16_IMPLEMENTED"),
    (104640usize, "FEAT_BRBE_IMPLEMENTED"),
    (104648usize, "FEAT_AA32I8MM_IMPLEMENTED"),
    (104656usize, "PIR_EL1"),
    (104664usize, "PMOVSSET_EL0"),
    (104672usize, "MDSCR_EL1"),
    (104680usize, "FEAT_ETMv4p3_IMPLEMENTED"),
    (104688usize, "ID_AA64ZFR0_EL1"),
    (104696usize, "__g1_activity_monitor_offset_implemented"),
    (104704usize, "ACTLR_EL1"),
    (104712usize, "_CLIDR"),
    (104720usize, "__ThisInstr"),
    (104728usize, "_CNTHVS_CTL"),
    (104736usize, "FEAT_S2POE_IMPLEMENTED"),
    (104744usize, "ID_DFR1_EL1"),
    (104752usize, "__has_spe_pseudo_cycles"),
    (104760usize, "FEAT_MTPMU_IMPLEMENTED"),
    (104768usize, "DBGOSLAR"),
    (104776usize, "__ExtDebugBase"),
    (104784usize, "TFSR_EL2"),
    (104792usize, "TFSR_EL3"),
    (104800usize, "PMCCNTR_EL0"),
    (104808usize, "_DBGAUTHSTATUS"),
    (104816usize, "ShouldAdvanceIT"),
    (104824usize, "ID_AA64DFR1_EL1"),
    (104832usize, "AMCNTENSET0_EL0"),
    (104840usize, "_ICC_BPR1_S"),
    (104848usize, "_ICC_PMR"),
    (104856usize, "CTIDEVID1"),
    (104864usize, "HFGITR2_EL2"),
    (104872usize, "AMCG1IDR_EL0"),
    (104880usize, "SPESampleEvents"),
    (104888usize, "FEAT_DoubleFault2_IMPLEMENTED"),
    (104896usize, "FAR_EL3"),
    (104904usize, "MDCR_EL2"),
    (104912usize, "PMOVSCLR_EL0"),
    (104920usize, "__syncAbortOnWriteNormNonCache"),
    (104928usize, "MVFR1_EL1"),
    (104936usize, "TPIDR2_EL0"),
    (104944usize, "SPNIDEN"),
    (104952usize, "PMSCR_EL2"),
    (104960usize, "HSTR_EL2"),
    (104968usize, "CTICIDR2"),
    (104976usize, "GICV_ABPR"),
    (104984usize, "FEAT_JSCVT_IMPLEMENTED"),
    (104992usize, "FEAT_MPAMv1p1_IMPLEMENTED"),
    (105000usize, "FEAT_PMUv3p4_IMPLEMENTED"),
    (105008usize, "PMPIDR1"),
    (105016usize, "FEAT_GICv3_TDIR_IMPLEMENTED"),
    (105024usize, "R17"),
    (105032usize, "_AMAIR0_NS"),
];
#[repr(align(8))]
pub struct State {
    data: [u8; 105036usize],
    guest_environment: alloc::boxed::Box<dyn plugins_api::guest::Environment>,
}
impl State {
    // Returns the ISA state with initial values and configuration set
    pub fn new(
        guest_environment: alloc::boxed::Box<dyn plugins_api::guest::Environment>,
    ) -> Self {
        Self {
            data: [0; 105036usize],
            guest_environment,
        }
    }
    pub fn write_register<T>(&mut self, offset: usize, value: T) {
        let start = offset;
        let end = start + core::mem::size_of::<T>();
        unsafe {
            core::ptr::write_unaligned(self.data[start..end].as_mut_ptr().cast(), value)
        };
    }
    pub fn read_register<T>(&self, offset: usize) -> T {
        let start = offset;
        let end = start + core::mem::size_of::<T>();
        unsafe { core::ptr::read_unaligned(self.data[start..end].as_ptr().cast()) }
    }
    pub fn write_memory(&self, address: u64, data: &[u8]) {
        self.guest_environment.write_memory(address, data);
    }
    pub fn read_memory(&self, address: u64, data: &mut [u8]) {
        self.guest_environment.read_memory(address, data);
    }
}
impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "State {{")?;
        for window in REGISTER_NAME_MAP.windows(2) {
            let (offset, name) = window[0];
            let (next_offset, _) = window[1];
            write!(f, "{name}: 0x")?;
            for byte_idx in 0..(next_offset - offset) {
                write!(f, "{:x}", self.read_register:: < u8 > (offset + byte_idx))?;
            }
            writeln!(f)?;
        }
        writeln!(f, "}}")
    }
}
pub const REG_ERXMISC1_EL1: usize = 1400usize;
pub const REG_FEAT_VMID16_IMPLEMENTED: usize = 1408usize;
pub const REG_V9AP0_IMPLEMENTED: usize = 1416usize;
pub const REG_FEAT_SVE_PMULL128_IMPLEMENTED: usize = 1424usize;
pub const REG_U__DBG_ROM_ADDR: usize = 1432usize;
pub const REG_U_ERXMISC7: usize = 1440usize;
pub const REG_GICH_EISR: usize = 1448usize;
pub const REG_U_VTCR: usize = 1456usize;
pub const REG_SCTLR2_EL3: usize = 1464usize;
pub const REG_PMU_EVENT_EXC_TAKEN: usize = 48usize;
pub const REG_ICC_CTLR_EL1_NS: usize = 1472usize;
pub const REG_ID_ISAR5_EL1: usize = 1480usize;
pub const REG_FEAT_EVT_IMPLEMENTED: usize = 1488usize;
pub const REG_U_PMINTENSET: usize = 1496usize;
pub const REG_FEAT_EL3_IMPLEMENTED: usize = 1512usize;
pub const REG_U_ICV_CTLR: usize = 1528usize;
pub const REG_AMPIDR2: usize = 1504usize;
pub const REG_PMVCIDSR: usize = 1536usize;
pub const REG_SPEADDRPOSPREVBRANCHTARGET: usize = 1048usize;
pub const REG_PMSDSFR_EL1: usize = 1520usize;
pub const REG_FEAT_FGT2_IMPLEMENTED: usize = 1544usize;
pub const REG_RLPIDEN: usize = 1552usize;
pub const REG_FEAT_ETEV1P2_IMPLEMENTED: usize = 1560usize;
pub const REG_FEAT_AES_IMPLEMENTED: usize = 1568usize;
pub const REG_U__MAX_IMPLEMENTED_SMEVECLEN: usize = 1576usize;
pub const REG_ICC_AP1R_EL1_S: usize = 1600usize;
pub const REG_MECID_A0_EL2: usize = 1592usize;
pub const REG_EDCIDR2: usize = 1640usize;
pub const REG_FEAT_SHA256_IMPLEMENTED: usize = 1632usize;
pub const REG_LORC_EL1: usize = 1648usize;
pub const REG_U_PMEVCNTR: usize = 1656usize;
pub const REG_U__EXCLUSIVE_GRANULE_SIZE: usize = 1784usize;
pub const REG_PMU_EVENT_LL_CACHE_HITM_RD: usize = 312usize;
pub const REG_FEAT_FGT_IMPLEMENTED: usize = 1792usize;
pub const REG_U_Z: usize = 1800usize;
pub const REG_FEAT_GICV4_IMPLEMENTED: usize = 9992usize;
pub const REG_FEAT_SEL2_IMPLEMENTED: usize = 10000usize;
pub const REG_U_ICH_AP1R: usize = 10008usize;
pub const REG_CFG_MPAM_NONE: usize = 1264usize;
pub const REG_FEAT_SME2P1_IMPLEMENTED: usize = 10024usize;
pub const REG_U__ETEBASE: usize = 10032usize;
pub const REG_GICC_BPR: usize = 10040usize;
pub const REG_CONTEXTIDR_EL1: usize = 10048usize;
pub const REG_GICR_STATUSR: usize = 10056usize;
pub const REG_CNTHVS_CVAL_EL2: usize = 10064usize;
pub const REG_SPEMAXCOUNTERS: usize = 1000usize;
pub const REG_STACK_LIMIT: usize = 10072usize;
pub const REG_GICC_ABPR: usize = 10080usize;
pub const REG_GICC_AIAR: usize = 10096usize;
pub const REG_U_CTR: usize = 10088usize;
pub const REG_FEAT_RASSAV1P1_IMPLEMENTED: usize = 10104usize;
pub const REG_ERXADDR_EL1: usize = 10112usize;
pub const REG_FEAT_PACQARMA5_IMPLEMENTED: usize = 10120usize;
pub const REG_OSLSR_EL1: usize = 10128usize;
pub const REG_GICR_SETLPIR: usize = 10136usize;
pub const REG_PMSLATFR_EL1: usize = 10144usize;
pub const REG_U_DBGDTRRXEXT: usize = 10152usize;
pub const REG_U_HDCR: usize = 10160usize;
pub const REG_BRBINFINJ_EL1: usize = 10168usize;
pub const REG_PMCEID1_EL0: usize = 10176usize;
pub const REG_SP_EL1: usize = 10184usize;
pub const REG_CP15SDISABLE: usize = 10192usize;
pub const REG_ICC_SRE_EL3: usize = 10200usize;
pub const REG_FEAT_HPMN0_IMPLEMENTED: usize = 10208usize;
pub const REG_GCSPR_EL2: usize = 10216usize;
pub const REG_U_ERXMISC1: usize = 10224usize;
pub const REG_GICV_RPR: usize = 10232usize;
pub const REG_ICH_LR_EL2: usize = 10240usize;
pub const REG_U__HIGHEST_EL_AARCH32: usize = 10368usize;
pub const REG_SMCR_EL2: usize = 10376usize;
pub const REG_SPERECORDSIZE: usize = 10384usize;
pub const REG_FEAT_RNG_TRAP_IMPLEMENTED: usize = 10400usize;
pub const REG_FEAT_DOPD_IMPLEMENTED: usize = 10408usize;
pub const REG_ERXCTLR_EL1: usize = 10416usize;
pub const REG_CFG_ID_AA64PFR0_EL1_EL1: usize = 1232usize;
pub const REG_U__CYCLE_COUNT: usize = 10424usize;
pub const REG_PMUACR_EL1: usize = 10440usize;
pub const REG_U_ICC_CTLR_S: usize = 10456usize;
pub const REG_FEAT_HAFT_IMPLEMENTED: usize = 10464usize;
pub const REG_U_CNTV_CTL: usize = 10448usize;
pub const REG_FEAT_PMUV3_EXT32_IMPLEMENTED: usize = 10472usize;
pub const REG_ACTLR2_S: usize = 10480usize;
pub const REG_U_ID_MMFR1: usize = 10488usize;
pub const REG_GICD_CTLR: usize = 10496usize;
pub const REG_CNTHPS_CTL_EL2: usize = 10504usize;
pub const REG_AMCFGR_EL0: usize = 10512usize;
pub const REG_PMCIDR2: usize = 10520usize;
pub const REG_SPESAMPLEINSTISNV2: usize = 10528usize;
pub const REG_VBAR_S: usize = 10536usize;
pub const REG_MAIR_EL2: usize = 10544usize;
pub const REG_FEAT_PACIMP_IMPLEMENTED: usize = 10552usize;
pub const REG_PMULASTTHRESHOLDVALUE: usize = 10560usize;
pub const REG_R25: usize = 10592usize;
pub const REG_ICV_IGRPEN1_EL1: usize = 10600usize;
pub const REG_ID_AA64AFR0_EL1: usize = 10608usize;
pub const REG_ACTLR_EL2: usize = 10616usize;
pub const REG_FEAT_DGH_IMPLEMENTED: usize = 10624usize;
pub const REG_GITS_TYPER: usize = 10632usize;
pub const REG_U__MONOMORPHIZE_READS: usize = 10640usize;
pub const REG_MPAMVPM1_EL2: usize = 10648usize;
pub const REG_RNDRRS: usize = 10656usize;
pub const REG_SPERECORDDATA: usize = 10664usize;
pub const REG_GICR_VSGIR: usize = 10728usize;
pub const REG_LOG2_TAG_GRANULE: usize = 456usize;
pub const REG_TCR_EL3: usize = 10736usize;
pub const REG_PMEVCNTR_EL0: usize = 10744usize;
pub const REG_U_MAIR0_NS: usize = 11000usize;
pub const REG_EDRCR: usize = 11008usize;
pub const REG_IFSR_S: usize = 11016usize;
pub const REG_GPRS: usize = 520usize;
pub const REG_FEAT_FLAGM2_IMPLEMENTED: usize = 11024usize;
pub const REG_MPAMIDR_EL1: usize = 11032usize;
pub const REG_ICH_MISR_EL2: usize = 11040usize;
pub const REG_U_AIFSR_NS: usize = 11048usize;
pub const REG_GICC_AHPPIR: usize = 11056usize;
pub const REG_ZCR_EL3: usize = 11064usize;
pub const REG_U_ERXFR: usize = 11072usize;
pub const REG_U_ID_DFR0: usize = 11080usize;
pub const REG_CPTR_EL2: usize = 11088usize;
pub const REG_APIBKEYLO_EL1: usize = 11096usize;
pub const REG_NUM_PMU_COUNTERS: usize = 11104usize;
pub const REG_PMPCSCTL: usize = 11120usize;
pub const REG_U__RD_BASE: usize = 11128usize;
pub const REG_SPESAMPLEADDRESSVALID: usize = 11136usize;
pub const REG_VSTCR_EL2: usize = 11168usize;
pub const REG_U__MAX_IMPLEMENTED_SVEVECLEN: usize = 11176usize;
pub const REG_PMU_EVENT_L2D_CACHE_RD: usize = 144usize;
pub const REG_U_CNTHCTL: usize = 11192usize;
pub const REG_SPEADDRPOSPCVIRTUAL: usize = 1016usize;
pub const REG_FEAT_ETMV4P1_IMPLEMENTED: usize = 11200usize;
pub const REG_PMEVTYPER_EL0: usize = 11208usize;
pub const REG_TRFCR_EL1: usize = 11464usize;
pub const REG_GICC_HPPIR: usize = 11472usize;
pub const REG_GCR_EL1: usize = 11480usize;
pub const REG_CFG_MPAM_V1P1: usize = 1280usize;
pub const REG_R23: usize = 11488usize;
pub const REG_FEAT_TIDCP1_IMPLEMENTED: usize = 11496usize;
pub const REG_DACR_S: usize = 11504usize;
pub const REG_EDPIDR1: usize = 11512usize;
pub const REG_U_SDER32_EL3: usize = 11520usize;
pub const REG_SPESAMPLESUBCLASSVALID: usize = 11528usize;
pub const REG_ICC_AP1R_EL1_NS: usize = 11536usize;
pub const REG_U_DBGDTR_EL0: usize = 11568usize;
pub const REG_FEAT_LSE128_IMPLEMENTED: usize = 11576usize;
pub const REG_U__RME_L0GPTSZ: usize = 11584usize;
pub const REG_AMAIR_EL3: usize = 11592usize;
pub const REG_FEAT_AMUV1_IMPLEMENTED: usize = 11600usize;
pub const REG_FEAT_PMUV3_EDGE_IMPLEMENTED: usize = 11608usize;
pub const REG_TTBR0_NS: usize = 11616usize;
pub const REG_FEAT_AIE_IMPLEMENTED: usize = 11624usize;
pub const REG_ICC_CTLR_EL3: usize = 11632usize;
pub const REG_PMMIR: usize = 11640usize;
pub const REG_TRFCR_EL2: usize = 11648usize;
pub const REG_R28: usize = 11656usize;
pub const REG_PMU_EVENT_SAMPLE_COLLISION: usize = 176usize;
pub const REG_FEAT_PCSRV8P2_IMPLEMENTED: usize = 11664usize;
pub const REG_TPIDRPRW_S: usize = 11672usize;
pub const REG_V8AP0_IMPLEMENTED: usize = 11680usize;
pub const REG_FEAT_AA64EL2_IMPLEMENTED: usize = 11688usize;
pub const REG_LR_MON: usize = 11696usize;
pub const REG_GCSCR_EL3: usize = 11704usize;
pub const REG_U_IFSR_NS: usize = 11712usize;
pub const REG_RCW128_PROTECTED_BIT: usize = 1192usize;
pub const REG_SCTLR2_EL1: usize = 11720usize;
pub const REG_ICC_NMIAR1_EL1: usize = 11728usize;
pub const REG_PMCNTENSET_EL0: usize = 11736usize;
pub const REG_ID_PFR2_EL1: usize = 11744usize;
pub const REG_U_AMEVTYPER0: usize = 11752usize;
pub const REG_U_ICH_LRC: usize = 11768usize;
pub const REG_EDDEVTYPE: usize = 11832usize;
pub const REG_FEAT_IDST_IMPLEMENTED: usize = 11840usize;
pub const REG_ISWFESLEEP: usize = 11848usize;
pub const REG_U_ICC_IAR1: usize = 11856usize;
pub const REG_FEAT_AA64EL3_IMPLEMENTED: usize = 11864usize;
pub const REG_U_ICH_MISR: usize = 11872usize;
pub const REG_FEAT_PMUV3_ICNTR_IMPLEMENTED: usize = 11880usize;
pub const REG_HPFAR_EL2: usize = 11888usize;
pub const REG_APGAKEYLO_EL1: usize = 11896usize;
pub const REG_ICC_SRE_EL1_S: usize = 11904usize;
pub const REG_U_ERXSTATUS: usize = 11912usize;
pub const REG_GICR_WAKER: usize = 11920usize;
pub const REG_FEAT_SVE_IMPLEMENTED: usize = 11928usize;
pub const REG_S2PIR_EL2: usize = 11936usize;
pub const REG_SPMACCESSR_EL1: usize = 11944usize;
pub const REG_U_ICC_AP0R: usize = 11952usize;
pub const REG_CNTFID0: usize = 11968usize;
pub const REG_TPIDR_EL2: usize = 11976usize;
pub const REG_ICC_IGRPEN1_EL3: usize = 11984usize;
pub const REG_ESR_EL3: usize = 11992usize;
pub const REG_FEAT_CSSC_IMPLEMENTED: usize = 12008usize;
pub const REG_GICR_VSGIPENDR: usize = 12000usize;
pub const REG_R6: usize = 12016usize;
pub const REG_FEAT_SPEV1P1_IMPLEMENTED: usize = 12024usize;
pub const REG_FEAT_SCTLR2_IMPLEMENTED: usize = 12032usize;
pub const REG_FEAT_MTE_TAGGED_FAR_IMPLEMENTED: usize = 12040usize;
pub const REG_ICV_IGRPEN0_EL1: usize = 12048usize;
pub const REG_GICD_TYPER2: usize = 12056usize;
pub const REG_U_CCSIDR: usize = 12064usize;
pub const REG_DBGCLAIMSET_EL1: usize = 12072usize;
pub const REG_SP_EL3: usize = 12080usize;
pub const REG_CPACR_EL1: usize = 12088usize;
pub const REG_U_HVBAR: usize = 12096usize;
pub const REG_PMVIDSR: usize = 12104usize;
pub const REG_FEAT_TRBE_MPAM_IMPLEMENTED: usize = 12112usize;
pub const REG_ICV_IAR0_EL1: usize = 12120usize;
pub const REG_FEAT_BRBEV1P1_IMPLEMENTED: usize = 12128usize;
pub const REG_SPIDEN: usize = 12136usize;
pub const REG_FEAT_PMUV3P1_IMPLEMENTED: usize = 12144usize;
pub const REG_FEAT_SME_FA64_IMPLEMENTED: usize = 12152usize;
pub const REG_U_HAMAIR0: usize = 12160usize;
pub const REG_FEAT_TWED_IMPLEMENTED: usize = 12168usize;
pub const REG_PIR_EL3: usize = 12176usize;
pub const REG_DBGBCR_EL1: usize = 12184usize;
pub const REG_STACK_BASE: usize = 12696usize;
pub const REG_U_ICC_RPR: usize = 12704usize;
pub const REG_AMAIR0_S: usize = 12712usize;
pub const REG_GICV_STATUSR: usize = 12720usize;
pub const REG_PMITCTRL: usize = 12728usize;
pub const REG_PMSIRR_EL1: usize = 12736usize;
pub const REG_U_PC: usize = 12744usize;
pub const REG_U_ICC_ASGI1R: usize = 12752usize;
pub const REG_NUM_AMU_COUNTER_GROUPS: usize = 12760usize;
pub const REG_ICC_PMR_EL1: usize = 12776usize;
pub const REG_FEAT_RASSAV2_IMPLEMENTED: usize = 12784usize;
pub const REG_U_MPAM3_EL3: usize = 12792usize;
pub const REG_FEAT_PAN3_IMPLEMENTED: usize = 12800usize;
pub const REG_CNTHCTL_EL2: usize = 12808usize;
pub const REG_TAG_GRANULE: usize = 1344usize;
pub const REG_TCR_EL2: usize = 12816usize;
pub const REG_ICV_CTLR_EL1: usize = 12824usize;
pub const REG_AMAIR_EL2: usize = 12832usize;
pub const REG_U_MVFR1: usize = 12840usize;
pub const REG_U_ICC_AP1R_NS: usize = 12848usize;
pub const REG_U_CCSIDR2: usize = 12864usize;
pub const REG_U_AMCGCR: usize = 12872usize;
pub const REG_TFSR_EL1: usize = 12880usize;
pub const REG_U_HSR: usize = 12888usize;
pub const REG_FEAT_RASV2_IMPLEMENTED: usize = 12896usize;
pub const REG_PMSNEVFR_EL1: usize = 12904usize;
pub const REG_FEAT_CSV2_1P2_IMPLEMENTED: usize = 12912usize;
pub const REG_PMU_EVENT_SAMPLE_WRAP: usize = 224usize;
pub const REG_DEBUGEXCEPTION_BREAKPOINT: usize = 1312usize;
pub const REG_FPCR: usize = 12920usize;
pub const REG_U_PMCCNTR: usize = 12928usize;
pub const REG_ERXMISC3_EL1: usize = 12936usize;
pub const REG_PMICNTR_EL0: usize = 12944usize;
pub const REG_U__DCZID_LOG2_BLOCK_SIZE: usize = 12952usize;
pub const REG_EDPIDR2: usize = 12968usize;
pub const REG_U_DCLONE: usize = 12976usize;
pub const REG_CTIAUTHSTATUS: usize = 13232usize;
pub const REG_U__SYNCABORTONTTWNONCACHE: usize = 13240usize;
pub const REG_U__SYNCABORTONREADNORMNONCACHE: usize = 13248usize;
pub const REG_U_ICV_DIR: usize = 13256usize;
pub const REG_U_AIDR: usize = 13264usize;
pub const REG_PMSSCR_EL1: usize = 13272usize;
pub const REG_U_CNTP_CTL_NS: usize = 13280usize;
pub const REG_FEAT_AA32EL3_IMPLEMENTED: usize = 13288usize;
pub const REG_U_AMEVTYPER1: usize = 13296usize;
pub const REG_DLR_EL0: usize = 13360usize;
pub const REG_AFSR0_EL2: usize = 13368usize;
pub const REG_U_TTBCR2_NS: usize = 13376usize;
pub const REG_GIC_BASE: usize = 1360usize;
pub const REG_U_ICV_BPR1: usize = 13384usize;
pub const REG_U__MPAM_PMG_MAX: usize = 13392usize;
pub const REG_FEAT_HPDS2_IMPLEMENTED: usize = 13400usize;
pub const REG_FEAT_PMUV3P9_IMPLEMENTED: usize = 13408usize;
pub const REG_U_HADFSR: usize = 13416usize;
pub const REG_U_ICH_ELRSR: usize = 13424usize;
pub const REG_APGAKEYHI_EL1: usize = 13432usize;
pub const REG_AMCNTENSET1_EL0: usize = 13440usize;
pub const REG_APDAKEYHI_EL1: usize = 13448usize;
pub const REG_PHYSICALCOUNT: usize = 13456usize;
pub const REG_U__GICITSCONTROLBASE: usize = 13472usize;
pub const REG_DEBUGHALT_STEP_EXCLUSIVE: usize = 1120usize;
pub const REG_ID_AA64PFR2_EL1: usize = 13480usize;
pub const REG_U_AMCFGR: usize = 13488usize;
pub const REG_BRBIDR0_EL1: usize = 13496usize;
pub const REG_SPESAMPLETIMESTAMP: usize = 13504usize;
pub const REG_GICR_SYNCR: usize = 13512usize;
pub const REG_U_NMRR_NS: usize = 13520usize;
pub const REG_SPESAMPLESUBCLASS: usize = 13528usize;
pub const REG_U_MPAM1_EL1: usize = 13536usize;
pub const REG_U_ID_MMFR5: usize = 13544usize;
pub const REG_ICV_EOIR0_EL1: usize = 13552usize;
pub const REG_FEAT_EXS_IMPLEMENTED: usize = 13560usize;
pub const REG_ICV_HPPIR0_EL1: usize = 13568usize;
pub const REG_FEAT_BBM_IMPLEMENTED: usize = 13576usize;
pub const REG_U__SME_ONLY: usize = 13584usize;
pub const REG_POR_EL1: usize = 13592usize;
pub const REG_U__THISINSTRENC: usize = 13600usize;
pub const REG_HFGITR_EL2: usize = 13608usize;
pub const REG_PMECR_EL1: usize = 13616usize;
pub const REG_EDAA32PFR: usize = 13624usize;
pub const REG_PMU_EVENT_SAMPLE_FILTRATE: usize = 168usize;
pub const REG_DISR_EL1: usize = 13632usize;
pub const REG_U_ID_ISAR6: usize = 13640usize;
pub const REG_M32_MONITOR: usize = 384usize;
pub const REG_VNCR_EL2: usize = 13648usize;
pub const REG_FEAT_PFAR_IMPLEMENTED: usize = 13656usize;
pub const REG_ICC_EOIR0_EL1: usize = 13664usize;
pub const REG_GICR_IIDR: usize = 13672usize;
pub const REG_CTICIDR0: usize = 13680usize;
pub const REG_SPMACCESSR_EL3: usize = 13688usize;
pub const REG_CNTEL0ACR: usize = 13696usize;
pub const REG_PMBSR_EL1: usize = 13704usize;
pub const REG_U_AMCR: usize = 13712usize;
pub const REG_U_ICV_RPR: usize = 13720usize;
pub const REG_U__IMPDEF_TG1: usize = 13728usize;
pub const REG_CTIDEVTYPE: usize = 13736usize;
pub const REG_EDCIDR1: usize = 13744usize;
pub const REG_CTIDEVCTL: usize = 13752usize;
pub const REG_PMU_EVENT_LDST_ALIGN_LAT: usize = 192usize;
pub const REG_GPT_CONTIG: usize = 864usize;
pub const REG_U_HTRFCR: usize = 13760usize;
pub const REG_FEAT_RASV1P1_IMPLEMENTED: usize = 13768usize;
pub const REG_SPESAMPLEADDRESS: usize = 13776usize;
pub const REG_U__LAST_BRANCH_VALID: usize = 14032usize;
pub const REG_EDPRSR: usize = 14040usize;
pub const REG_CFG_MPIDR: usize = 14048usize;
pub const REG_FEAT_DEBUGV8P2_IMPLEMENTED: usize = 14056usize;
pub const REG_FEAT_LRCPC_IMPLEMENTED: usize = 14064usize;
pub const REG_PMPIDR2: usize = 14072usize;
pub const REG_U_IFAR_NS: usize = 14080usize;
pub const REG_U_HAIFSR: usize = 14088usize;
pub const REG_U_DBGWCR: usize = 14096usize;
pub const REG_CNTPS_CVAL_EL1: usize = 14160usize;
pub const REG_DEBUGEXCEPTION_BKPT: usize = 1320usize;
pub const REG_U_TTBR1_EL1: usize = 14168usize;
pub const REG_SPESAMPLEDATASOURCEVALID: usize = 14184usize;
pub const REG_AMDEVTYPE: usize = 14192usize;
pub const REG_POR_EL3: usize = 14200usize;
pub const REG_U_EDSCR2: usize = 14208usize;
pub const REG_U__SUPPORTED_VA_SIZE: usize = 14216usize;
pub const REG_FEAT_HCX_IMPLEMENTED: usize = 14232usize;
pub const REG_U__CNTBASE_FREQUENCY: usize = 14240usize;
pub const REG_GITS_CBASER: usize = 14248usize;
pub const REG_U__MPAM_FRAC: usize = 14256usize;
pub const REG_FEAT_ADERR_IMPLEMENTED: usize = 14264usize;
pub const REG_U_PMCNTEN: usize = 14272usize;
pub const REG_TPIDR_EL1: usize = 14280usize;
pub const REG_U_TPIDRURW_NS: usize = 14288usize;
pub const REG_GPT_REALM: usize = 896usize;
pub const REG_FEAT_AMUV1P1_IMPLEMENTED: usize = 14296usize;
pub const REG_FEAT_CSV2_1P1_IMPLEMENTED: usize = 14304usize;
pub const REG_FEAT_ANERR_IMPLEMENTED: usize = 14312usize;
pub const REG_APDBKEYHI_EL1: usize = 14320usize;
pub const REG_NUM_GIC_PREEMPTION_BITS: usize = 14328usize;
pub const REG_U__SET_MOPS_OPTION_A_SUPPORTED: usize = 14344usize;
pub const REG_FEAT_LS64_V_IMPLEMENTED: usize = 14352usize;
pub const REG_HEAP_LIMIT: usize = 14360usize;
pub const REG_U_PMCEID0: usize = 14368usize;
pub const REG_SP_REL_ACCESS_PC: usize = 14376usize;
pub const REG_ID_ISAR1_EL1: usize = 14384usize;
pub const REG_U_ERRIDR: usize = 14392usize;
pub const REG_U__HAS_SME_PRIORITY_CONTROL: usize = 14400usize;
pub const REG_GICR_CLRLPIR: usize = 14408usize;
pub const REG_ERXGSR_EL1: usize = 14416usize;
pub const REG_FEAT_TRC_SR_IMPLEMENTED: usize = 14424usize;
pub const REG_FEAT_RNG_IMPLEMENTED: usize = 14432usize;
pub const REG_GITS_MPIDR: usize = 14440usize;
pub const REG_FEAT_PMUV3P5_IMPLEMENTED: usize = 14448usize;
pub const REG_FEAT_LVA3_IMPLEMENTED: usize = 14456usize;
pub const REG_FEAT_MTE_STORE_ONLY_IMPLEMENTED: usize = 14464usize;
pub const REG_FEAT_PCSRV8P9_IMPLEMENTED: usize = 14472usize;
pub const REG_FEAT_SPE_FDS_IMPLEMENTED: usize = 14480usize;
pub const REG_PMU_EVENT_SAMPLE_FEED_OP: usize = 256usize;
pub const REG_U_AMAIR1_NS: usize = 14488usize;
pub const REG_ICC_IGRPEN0_EL1: usize = 14496usize;
pub const REG_U_PMINTEN: usize = 14504usize;
pub const REG_GICR_CTLR: usize = 14512usize;
pub const REG_DBGDEVID: usize = 14520usize;
pub const REG_THROW: usize = 14528usize;
pub const REG_U_TTBR0_EL1: usize = 14536usize;
pub const REG_U__CNTBASEN: usize = 14552usize;
pub const REG_U_FFR: usize = 14560usize;
pub const REG_CNTPOFF_EL2: usize = 14592usize;
pub const REG_APDAKEYLO_EL1: usize = 14600usize;
pub const REG_ID_AA64ISAR1_EL1: usize = 14608usize;
pub const REG_AFSR1_EL3: usize = 14616usize;
pub const REG_FEAT_SHA512_IMPLEMENTED: usize = 14624usize;
pub const REG_AMEVCNTR0: usize = 14632usize;
pub const REG_AMCGCR_EL0: usize = 14664usize;
pub const REG_MAX_ZERO_BLOCK_SIZE: usize = 1088usize;
pub const REG_FEAT_EL1_IMPLEMENTED: usize = 14672usize;
pub const REG_U_ID_ISAR3: usize = 14680usize;
pub const REG_U_PMSWINC: usize = 14688usize;
pub const REG_FEAT_IVIPT_IMPLEMENTED: usize = 14696usize;
pub const REG_SEE: usize = 14704usize;
pub const REG_EDESR: usize = 14720usize;
pub const REG_U_IFAR_S: usize = 14728usize;
pub const REG_U_ID_PFR0: usize = 14736usize;
pub const REG_PMSIDR_EL1: usize = 14744usize;
pub const REG_FEAT_SB_IMPLEMENTED: usize = 14752usize;
pub const REG_U_CNTHP_CVAL: usize = 14760usize;
pub const REG_FEAT_PCSRV8_IMPLEMENTED: usize = 14768usize;
pub const REG_R29: usize = 14776usize;
pub const REG_TCR2_EL1: usize = 14784usize;
pub const REG_FEAT_LSE_IMPLEMENTED: usize = 14792usize;
pub const REG_APIAKEYHI_EL1: usize = 14800usize;
pub const REG_ZCR_EL3_LEN_VALUE: usize = 14808usize;
pub const REG_FEAT_SVE_BITPERM_IMPLEMENTED: usize = 14824usize;
pub const REG_HTTBR: usize = 14832usize;
pub const REG_ICH_AP0R_EL2: usize = 14840usize;
pub const REG_ID_AA64ISAR2_EL1: usize = 14872usize;
pub const REG_CNTHVS_CTL_EL2: usize = 14880usize;
pub const REG_SPESAMPLECONTEXTEL2VALID: usize = 14888usize;
pub const REG_DOMAIN_NOACCESS: usize = 784usize;
pub const REG_ICC_ASGI1R_EL1: usize = 14896usize;
pub const REG_ID_AA64MMFR0_EL1: usize = 14904usize;
pub const REG_HACR_EL2: usize = 14912usize;
pub const REG_FEAT_CONSTPACFIELD_IMPLEMENTED: usize = 14920usize;
pub const REG_PMU_EVENT_SAMPLE_POP: usize = 152usize;
pub const REG_FEAT_GICV3_IMPLEMENTED: usize = 14928usize;
pub const REG_FEAT_CHK_IMPLEMENTED: usize = 14936usize;
pub const REG_FEAT_ETEV1P1_IMPLEMENTED: usize = 14944usize;
pub const REG_U__BRANCHTAKEN: usize = 14952usize;
pub const REG_TFSRE0_EL1: usize = 14960usize;
pub const REG_MDRAR_EL1: usize = 14968usize;
pub const REG_PMCEID0_EL0: usize = 14976usize;
pub const REG_GITS_CREADR: usize = 14984usize;
pub const REG_PMIIDR: usize = 14992usize;
pub const REG_U_ID_ISAR4: usize = 15000usize;
pub const REG_U__CNTCTLBASE: usize = 15008usize;
pub const REG_GICM_CLRSPI_NSR: usize = 15032usize;
pub const REG_U_ERXMISC4: usize = 15016usize;
pub const REG_RVBAR: usize = 15040usize;
pub const REG_CFG_ID_AA64PFR0_EL1_EL0: usize = 1224usize;
pub const REG_GITS_CTLR: usize = 15024usize;
pub const REG_U_EDSCR: usize = 15048usize;
pub const REG_SDCR: usize = 15056usize;
pub const REG_IFSR32_EL2: usize = 15064usize;
pub const REG_ICV_PMR_EL1: usize = 15072usize;
pub const REG_CFG_MPAM_FRAC_NONE: usize = 1288usize;
pub const REG_ZCR_EL2: usize = 15080usize;
pub const REG_U_AMEVCNTR1: usize = 15088usize;
pub const REG_M32_ABORT: usize = 392usize;
pub const REG_FEAT_FRINTTS_IMPLEMENTED: usize = 15216usize;
pub const REG_U_SPSR_SVC: usize = 15224usize;
pub const REG_U__EMPAM_TIDR_IMPLEMENTED: usize = 15232usize;
pub const REG_DBGDEVID1: usize = 15240usize;
pub const REG_FEAT_TRC_EXT_IMPLEMENTED: usize = 15248usize;
pub const REG_U_ERXMISC0: usize = 15256usize;
pub const REG_FEAT_F32MM_IMPLEMENTED: usize = 15264usize;
pub const REG_V8AP3_IMPLEMENTED: usize = 15272usize;
pub const REG_ERRIDR_EL1: usize = 15280usize;
pub const REG_GICC_AEOIR: usize = 15288usize;
pub const REG_GICC_DIR: usize = 15296usize;
pub const REG_FEAT_ECV_IMPLEMENTED: usize = 15304usize;
pub const REG_U_CPACR: usize = 15312usize;
pub const REG_FEAT_SPEV1P2_IMPLEMENTED: usize = 15320usize;
pub const REG_U__SYNCABORTONPREFETCH: usize = 15328usize;
pub const REG_VTCR_EL2: usize = 15336usize;
pub const REG_POR_EL2: usize = 15344usize;
pub const REG_PMCCNTSVR_EL1: usize = 15352usize;
pub const REG_PMXEVCNTR_EL0: usize = 15360usize;
pub const REG_SP_MON: usize = 15368usize;
pub const REG_TTBCR_S: usize = 15376usize;
pub const REG_ICH_VMCR_EL2: usize = 15384usize;
pub const REG_U_FPSCR: usize = 15392usize;
pub const REG_ICV_RPR_EL1: usize = 15400usize;
pub const REG_AFSR1_EL2: usize = 15408usize;
pub const REG_ACTLR_S: usize = 15416usize;
pub const REG_FEAT_LPA_IMPLEMENTED: usize = 15424usize;
pub const REG_DEFAULTPARTID: usize = 768usize;
pub const REG_EDPFR: usize = 15432usize;
pub const REG_FEAT_ETMV4P4_IMPLEMENTED: usize = 15440usize;
pub const REG_SPESAMPLEPREVIOUSBRANCHADDRESS: usize = 15448usize;
pub const REG_PMINTENCLR_EL1: usize = 15456usize;
pub const REG_EDLSR: usize = 15464usize;
pub const REG_MPAMVPM2_EL2: usize = 15472usize;
pub const REG_AMPIDR1: usize = 15480usize;
pub const REG_RTPIDEN: usize = 15488usize;
pub const REG_FEAT_DOTPROD_IMPLEMENTED: usize = 15496usize;
pub const REG_GICR_PENDBASER: usize = 15504usize;
pub const REG_U_ID_ISAR2: usize = 15512usize;
pub const REG_GICC_IAR: usize = 15520usize;
pub const REG_U_MAIR1_S: usize = 15528usize;
pub const REG_U_ICC_BPR0: usize = 15536usize;
pub const REG_DEBUGHALT_HALTINSTRUCTION: usize = 1152usize;
pub const REG_SPSR_FIQ: usize = 15544usize;
pub const REG_AMCR_EL0: usize = 15552usize;
pub const REG_FEAT_DPB_IMPLEMENTED: usize = 15560usize;
pub const REG_U_SCTLR_NS: usize = 15568usize;
pub const REG_ICC_IAR0_EL1: usize = 15576usize;
pub const REG_FPSID: usize = 15584usize;
pub const REG_FEAT_CSV3_IMPLEMENTED: usize = 15592usize;
pub const REG_FEAT_S1POE_IMPLEMENTED: usize = 15600usize;
pub const REG_FEAT_LSMAOC_IMPLEMENTED: usize = 15608usize;
pub const REG_GCSCRE0_EL1: usize = 15616usize;
pub const REG_LST_64BV: usize = 1200usize;
pub const REG_AMIIDR: usize = 15624usize;
pub const REG_U__BLOCK_BBM_IMPLEMENTED: usize = 15632usize;
pub const REG_PMU_EVENT_SAMPLE_FEED_ST: usize = 248usize;
pub const REG_U_ERXCTLR: usize = 15648usize;
pub const REG_GICC_CTLR: usize = 15656usize;
pub const REG_HAVE_EXCEPTION: usize = 15664usize;
pub const REG_CPTR_EL3_EZ_VALUE: usize = 15672usize;
pub const REG_R2: usize = 15688usize;
pub const REG_ACTLR_EL3: usize = 15696usize;
pub const REG_FEAT_VPIPT_IMPLEMENTED: usize = 15704usize;
pub const REG_U_ICC_HPPIR0: usize = 15712usize;
pub const REG_MEMATTR_WT: usize = 472usize;
pub const REG_PMBIDR_EL1: usize = 15720usize;
pub const REG_CTIITCTRL: usize = 15728usize;
pub const REG_VMECID_A_EL2: usize = 15736usize;
pub const REG_U_HAMAIR1: usize = 15744usize;
pub const REG_SPSR_EL2: usize = 15752usize;
pub const REG_COLD_RESET: usize = 1392usize;
pub const REG_CURRENT_EXCEPTION: usize = 15760usize;
pub const REG_LORSA_EL1: usize = 15768usize;
pub const REG_TCR2_EL2: usize = 15776usize;
pub const REG_APDBKEYLO_EL1: usize = 15784usize;
pub const REG_RVBAR_EL3: usize = 15792usize;
pub const REG_PMPIDR0: usize = 15800usize;
pub const REG_U_ICH_LR: usize = 15808usize;
pub const REG_U__CLOCK_DIVIDER: usize = 15872usize;
pub const REG_PMCCFILTR_EL0: usize = 15888usize;
pub const REG_OSDTRRX_EL1: usize = 15896usize;
pub const REG_DBGDSAR: usize = 15904usize;
pub const REG_U_VPIDR: usize = 15912usize;
pub const REG_CNTID: usize = 15920usize;
pub const REG_FEAT_SVE2_IMPLEMENTED: usize = 15928usize;
pub const REG_FEAT_SME2_IMPLEMENTED: usize = 15936usize;
pub const REG_HEAP_BASE: usize = 15944usize;
pub const REG_FEAT_ETMV4P2_IMPLEMENTED: usize = 15952usize;
pub const REG_U__MECID_WIDTH: usize = 15960usize;
pub const REG_BRBTGT_EL1: usize = 15968usize;
pub const REG_GICV_HPPIR: usize = 16224usize;
pub const REG_FEAT_PMUV3_IMPLEMENTED: usize = 16232usize;
pub const REG_FEAT_SSBS_IMPLEMENTED: usize = 16240usize;
pub const REG_U_HDFAR: usize = 16248usize;
pub const REG_U_ICV_IAR1: usize = 16256usize;
pub const REG_ISR_EL1: usize = 16264usize;
pub const REG_FEAT_NTLBPA_IMPLEMENTED: usize = 16272usize;
pub const REG_FAR_EL1: usize = 16280usize;
pub const REG_RVBAR_EL1: usize = 16288usize;
pub const REG_U_CNTKCTL: usize = 16296usize;
pub const REG_TPIDR_EL3: usize = 16304usize;
pub const REG_ID_PFR0_EL1: usize = 16312usize;
pub const REG_FEAT_RPRES_IMPLEMENTED: usize = 16320usize;
pub const REG_U_PRRR_NS: usize = 16328usize;
pub const REG_DEBUGHALT_EXCEPTIONCATCH: usize = 1168usize;
pub const REG_FEAT_TCR2_IMPLEMENTED: usize = 16336usize;
pub const REG_U_ICC_IAR0: usize = 16344usize;
pub const REG_SPECOUNTERPOSTRANSLATIONLATENCY: usize = 1072usize;
pub const REG_FEAT_SHA1_IMPLEMENTED: usize = 16352usize;
pub const REG_FEAT_AA32HPD_IMPLEMENTED: usize = 16360usize;
pub const REG_PMU_EVENT_L3D_CACHE_HITM_RD: usize = 304usize;
pub const REG_FEAT_LSE2_IMPLEMENTED: usize = 16368usize;
pub const REG_CFG_RMR_AA64: usize = 16376usize;
pub const REG_MEMHINT_NO: usize = 488usize;
pub const REG_U_PMCNTENSET: usize = 16384usize;
pub const REG_ICC_SRE_EL2: usize = 16392usize;
pub const REG_HFGWTR2_EL2: usize = 16400usize;
pub const REG_PMPIDR3: usize = 16408usize;
pub const REG_U_DBGBVR: usize = 16416usize;
pub const REG_SCTLR_S: usize = 16480usize;
pub const REG_FEAT_FHM_IMPLEMENTED: usize = 16488usize;
pub const REG_EDWAR: usize = 16496usize;
pub const REG_R1: usize = 16504usize;
pub const REG_U_CONTEXTIDR_NS: usize = 16512usize;
pub const REG_DEBUGHALT_OSUNLOCKCATCH: usize = 1128usize;
pub const REG_AFSR0_EL1: usize = 16520usize;
pub const REG_RCWSMASK_EL1: usize = 16528usize;
pub const REG_SCXTNUM_EL2: usize = 16544usize;
pub const REG_ERXPFGCDN_EL1: usize = 16552usize;
pub const REG_BRBFCR_EL1: usize = 16560usize;
pub const REG_U__IMPDEF_TG0: usize = 16568usize;
pub const REG_SPMSELR_EL0: usize = 16576usize;
pub const REG_U_PMUSERENR: usize = 16584usize;
pub const REG_FCSEIDR: usize = 16592usize;
pub const REG_GICD_SETSPI_SR: usize = 16600usize;
pub const REG_PMU_EVENT_L1D_CACHE_LMISS_RD: usize = 136usize;
pub const REG_CFG_MPAM_FRAC_V0P1: usize = 1296usize;
pub const REG_DACR32_EL2: usize = 16608usize;
pub const REG_HFGRTR_EL2: usize = 16616usize;
pub const REG_TPIDRURO_S: usize = 16624usize;
pub const REG_FEAT_DEBUGV8P9_IMPLEMENTED: usize = 16632usize;
pub const REG_FEAT_MEC_IMPLEMENTED: usize = 16640usize;
pub const REG_MPAM0_EL1: usize = 16648usize;
pub const REG_FEAT_TLBIOS_IMPLEMENTED: usize = 16656usize;
pub const REG_SPEADDRPOSDATAVIRTUAL: usize = 1032usize;
pub const REG_CNTHP_CVAL_EL2: usize = 16664usize;
pub const REG_GPCCR_EL3: usize = 16672usize;
pub const REG_AFSR0_EL3: usize = 16680usize;
pub const REG_AMEVCNTVOFF1_EL2: usize = 16688usize;
pub const REG_U_AMUSERENR: usize = 16816usize;
pub const REG_U_ICC_EOIR1: usize = 16824usize;
pub const REG_EDCIDR3: usize = 16832usize;
pub const REG_DBGDIDR: usize = 16840usize;
pub const REG_FEAT_LVA_IMPLEMENTED: usize = 16848usize;
pub const REG_MDCCSR_EL0: usize = 16856usize;
pub const REG_CPTR_EL3: usize = 16864usize;
pub const REG_CNTP_CVAL_S: usize = 16872usize;
pub const REG_GPTRANGE_32MB: usize = 944usize;
pub const REG_AIDR_EL1: usize = 16880usize;
pub const REG_U_AMCNTENSET0: usize = 16888usize;
pub const REG_U_DACR_NS: usize = 16896usize;
pub const REG_EDLAR: usize = 16904usize;
pub const REG_FEAT_AA64EL1_IMPLEMENTED: usize = 16912usize;
pub const REG_U_ICH_AP0R: usize = 16920usize;
pub const REG_ERRNFR: usize = 16936usize;
pub const REG_R15: usize = 16968usize;
pub const REG_U_PMCCFILTR: usize = 16976usize;
pub const REG_PMCFGR: usize = 16984usize;
pub const REG_PSTATE: usize = 16992usize;
pub const REG_PMU_EVENT_L2D_LFB_HIT_RD: usize = 328usize;
pub const REG_EDDEVARCH: usize = 17024usize;
pub const REG_U_ID_ISAR1: usize = 17032usize;
pub const REG_TCMTR: usize = 17040usize;
pub const REG_EDHSR: usize = 17048usize;
pub const REG_U__CNTREADBASE: usize = 17056usize;
pub const REG_ICC_IGRPEN1_EL1_NS: usize = 17064usize;
pub const REG_GICH_VTR: usize = 17072usize;
pub const REG_GICD_SGIR: usize = 17080usize;
pub const REG_FEAT_ADVSIMD_IMPLEMENTED: usize = 17088usize;
pub const REG_SCTLR_EL3: usize = 17096usize;
pub const REG_U_ERXMISC3: usize = 17104usize;
pub const REG_U_ELR_HYP: usize = 17112usize;
pub const REG_U_PMSELR: usize = 17120usize;
pub const REG_R19: usize = 17128usize;
pub const REG_CNTHVS_TVAL_EL2: usize = 17136usize;
pub const REG_AIFSR_S: usize = 17144usize;
pub const REG_U_PMCEID2: usize = 17152usize;
pub const REG_SPESAMPLECLASS: usize = 17160usize;
pub const REG_NIDEN: usize = 17168usize;
pub const REG_VBAR_EL1: usize = 17176usize;
pub const REG_FEAT_ECBHB_IMPLEMENTED: usize = 17184usize;
pub const REG_ICC_HPPIR1_EL1: usize = 17192usize;
pub const REG_ICH_ELRSR_EL2: usize = 17200usize;
pub const REG_FEAT_MOPS_IMPLEMENTED: usize = 17208usize;
pub const REG_CLIDR_EL1: usize = 17216usize;
pub const REG_CNTV_CTL_EL0: usize = 17224usize;
pub const REG_U_MAIR1_NS: usize = 17232usize;
pub const REG_FEAT_SPE_IMPLEMENTED: usize = 17240usize;
pub const REG_ELR_EL2: usize = 17248usize;
pub const REG_DBGDTRTX_EL0: usize = 17256usize;
pub const REG_TPIDRRO_EL0: usize = 17264usize;
pub const REG_ICC_EOIR1_EL1: usize = 17272usize;
pub const REG_PMCIDR0: usize = 17280usize;
pub const REG_FEAT_SME_I16I64_IMPLEMENTED: usize = 17288usize;
pub const REG_MEMATTR_WB: usize = 480usize;
pub const REG_FEAT_FP_IMPLEMENTED: usize = 17296usize;
pub const REG_FEAT_MTE_ASYM_FAULT_IMPLEMENTED: usize = 17304usize;
pub const REG_FEAT_SPE_CRR_IMPLEMENTED: usize = 17312usize;
pub const REG_FEAT_TRBE_IMPLEMENTED: usize = 17320usize;
pub const REG_SMCR_EL1: usize = 17328usize;
pub const REG_MPAMVPMV_EL2: usize = 17336usize;
pub const REG_U_VDISR: usize = 17344usize;
pub const REG_ICC_BPR0_EL1: usize = 17352usize;
pub const REG_ID_ISAR0_EL1: usize = 17360usize;
pub const REG_ICC_BPR1_EL1_NS: usize = 17368usize;
pub const REG_ICH_VTR_EL2: usize = 17376usize;
pub const REG_HDFGWTR_EL2: usize = 17384usize;
pub const REG_FEAT_MTE_PERM_IMPLEMENTED: usize = 17392usize;
pub const REG_MPIDR_EL1: usize = 17400usize;
pub const REG_DEBUGHALT_RESETCATCH: usize = 1136usize;
pub const REG_PMPCSR: usize = 17408usize;
pub const REG_U_ICC_SGI0R: usize = 17416usize;
pub const REG_AMEVCNTVOFF0_EL2: usize = 17424usize;
pub const REG_ERXFR_EL1: usize = 17552usize;
pub const REG_GICR_VPENDBASER: usize = 17560usize;
pub const REG_U_ICC_BPR1_NS: usize = 17568usize;
pub const REG_SPESAMPLEDATASOURCE: usize = 17576usize;
pub const REG_GICM_SETSPI_NSR: usize = 17584usize;
pub const REG_NUM_GIC_LIST_REGS: usize = 17592usize;
pub const REG_U_PMINTENCLR: usize = 17608usize;
pub const REG_GICM_TYPER: usize = 17616usize;
pub const REG_FEAT_DEBUGV8P8_IMPLEMENTED: usize = 17624usize;
pub const REG_MPAMHCR_EL2: usize = 17632usize;
pub const REG_PMU_EVENT_SAMPLE_FEED: usize = 160usize;
pub const REG_SPESAMPLETIMESTAMPVALID: usize = 17640usize;
pub const REG_FEAT_CMOW_IMPLEMENTED: usize = 17648usize;
pub const REG_FEAT_ETEV1P3_IMPLEMENTED: usize = 17656usize;
pub const REG_V8AP1_IMPLEMENTED: usize = 17664usize;
pub const REG_U_DBGDSCREXT: usize = 17672usize;
pub const REG_MAIR_EL3: usize = 17680usize;
pub const REG_FINAL_LEVEL: usize = 800usize;
pub const REG_HDFGWTR2_EL2: usize = 17688usize;
pub const REG_FEAT_ABLE_IMPLEMENTED: usize = 17696usize;
pub const REG_GICV_IAR: usize = 17704usize;
pub const REG_U_PMOVS: usize = 17712usize;
pub const REG_CTIPIDR2: usize = 17720usize;
pub const REG_V8AP8_IMPLEMENTED: usize = 17728usize;
pub const REG_EL1: usize = 440usize;
pub const REG_FEAT_RME_IMPLEMENTED: usize = 17736usize;
pub const REG_U_DBGDRAR: usize = 17744usize;
pub const REG_GITS_PARTIDR: usize = 17752usize;
pub const REG_U_P: usize = 17760usize;
pub const REG_PMU_EVENT_CHAIN: usize = 80usize;
pub const REG_GCSPR_EL3: usize = 18272usize;
pub const REG_FEAT_ASMV8P2_IMPLEMENTED: usize = 18280usize;
pub const REG_U__VLPI_BASE: usize = 18288usize;
pub const REG_BRBCR_EL2: usize = 18296usize;
pub const REG_U__UNPRED_TSIZE_ABORTS: usize = 18304usize;
pub const REG_CNTCR: usize = 18312usize;
pub const REG_CNTHP_TVAL_EL2: usize = 18320usize;
pub const REG_U_ICV_HPPIR1: usize = 18328usize;
pub const REG_ELR_EL1: usize = 18336usize;
pub const REG_R4: usize = 18344usize;
pub const REG_U__ICACHE_CCSIDR_RESET: usize = 18352usize;
pub const REG_U_HSCTLR: usize = 18408usize;
pub const REG_ICC_CTLR_EL1_S: usize = 18416usize;
pub const REG_U_TPIDRURO_NS: usize = 18424usize;
pub const REG_U_ERXADDR2: usize = 18432usize;
pub const REG_MDSELR_EL1: usize = 18440usize;
pub const REG_DEFAULT_MECID: usize = 832usize;
pub const REG_SPSR_UND: usize = 18448usize;
pub const REG_TTBR1_EL2: usize = 18456usize;
pub const REG_U_VTTBR_EL2: usize = 18472usize;
pub const REG_SPESAMPLECOUNTER: usize = 18488usize;
pub const REG_GICH_VMCR: usize = 19000usize;
pub const REG_CTILAR: usize = 19008usize;
pub const REG_PMDEVTYPE: usize = 19016usize;
pub const REG_GICC_EOIR: usize = 19024usize;
pub const REG_GPTRANGE_16KB: usize = 920usize;
pub const REG_ID_ISAR4_EL1: usize = 19032usize;
pub const REG_FEAT_CSV2_2_IMPLEMENTED: usize = 19040usize;
pub const REG_FEAT_SYSREG128_IMPLEMENTED: usize = 19048usize;
pub const REG_R9: usize = 19056usize;
pub const REG_SPESAMPLEOPTYPE: usize = 19064usize;
pub const REG_CTIPIDR0: usize = 19072usize;
pub const REG_CTR_EL0: usize = 19080usize;
pub const REG_SPMACCESSR_EL2: usize = 19088usize;
pub const REG_FEAT_CSV2_3_IMPLEMENTED: usize = 19096usize;
pub const REG_FEAT_SPMU_IMPLEMENTED: usize = 19104usize;
pub const REG_U__TLB_ENABLED: usize = 19112usize;
pub const REG_U_VBAR_NS: usize = 19120usize;
pub const REG_MAIR2_EL3: usize = 19128usize;
pub const REG_R14: usize = 19136usize;
pub const REG_TTBR1_S: usize = 19144usize;
pub const REG_V8AP5_IMPLEMENTED: usize = 19152usize;
pub const REG_PMSELR_EL0: usize = 19160usize;
pub const REG_DEBUGHALT_SOFTWAREACCESS: usize = 1160usize;
pub const REG_HDFGRTR_EL2: usize = 19168usize;
pub const REG_AMEVTYPER1_EL0: usize = 19176usize;
pub const REG_MEMATTR_NC: usize = 464usize;
pub const REG_CNTHV_CTL_EL2: usize = 19304usize;
pub const REG_ICC_RPR_EL1: usize = 19312usize;
pub const REG_AMDEVARCH: usize = 19320usize;
pub const REG_GCSCR_EL2: usize = 19328usize;
pub const REG_EDPCSR: usize = 19336usize;
pub const REG_U_ERXFR2: usize = 19344usize;
pub const REG_VDISR_EL2: usize = 19352usize;
pub const REG_FEAT_MTE_ASYNC_IMPLEMENTED: usize = 19360usize;
pub const REG_U_CNTP_CVAL_NS: usize = 19368usize;
pub const REG_DBGDEVID2: usize = 19376usize;
pub const REG_NUM_WATCHPOINTS: usize = 19384usize;
pub const REG_CNTSR: usize = 19400usize;
pub const REG_AMCIDR1: usize = 19408usize;
pub const REG_DBGWVR_EL1: usize = 19416usize;
pub const REG_ICH_AP1R_EL2: usize = 19928usize;
pub const REG_FEAT_FCMA_IMPLEMENTED: usize = 19960usize;
pub const REG_FEAT_GICV3P1_IMPLEMENTED: usize = 19968usize;
pub const REG_DEBUGHALT_STEP_NORMAL: usize = 1112usize;
pub const REG_U__SYNCABORTONTTWCACHE: usize = 19976usize;
pub const REG_FEAT_S1PIE_IMPLEMENTED: usize = 19984usize;
pub const REG_OSECCR_EL1: usize = 19992usize;
pub const REG_GPT_NOACCESS: usize = 840usize;
pub const REG_PMU_EVENT_SAMPLE_FEED_BR: usize = 232usize;
pub const REG_FEAT_ETMV4P5_IMPLEMENTED: usize = 20000usize;
pub const REG_PRRR_S: usize = 20008usize;
pub const REG_ICC_MSRE: usize = 20016usize;
pub const REG_U_ERXMISC5: usize = 20024usize;
pub const REG_PFAR_EL2: usize = 20032usize;
pub const REG_CTICIDR1: usize = 20040usize;
pub const REG_TTBR1_NS: usize = 20048usize;
pub const REG_SPSR_ABT: usize = 20056usize;
pub const REG_U_ICV_IAR0: usize = 20064usize;
pub const REG_MAIR2_EL1: usize = 20072usize;
pub const REG_FEAT_MTE_NO_ADDRESS_TAGS_IMPLEMENTED: usize = 20080usize;
pub const REG_R21: usize = 20088usize;
pub const REG_MDCCINT_EL1: usize = 20096usize;
pub const REG_AMCIDR2: usize = 20104usize;
pub const REG_U_ICH_HCR: usize = 20112usize;
pub const REG_PMU_EVENT_BRB_FILTRATE: usize = 216usize;
pub const REG_RGSR_EL1: usize = 20120usize;
pub const REG_U_MIDR: usize = 20128usize;
pub const REG_ID_AA64DFR0_EL1: usize = 20136usize;
pub const REG_U_ID_PFR1: usize = 20144usize;
pub const REG_ELR_EL3: usize = 20152usize;
pub const REG_U__SYNCABORTONSOREAD: usize = 20160usize;
pub const REG_INSTRUCTION_COUNTER_ID: usize = 8usize;
pub const REG_ID_AA64AFR1_EL1: usize = 20168usize;
pub const REG_FEAT_AA64EL0_IMPLEMENTED: usize = 20176usize;
pub const REG_FEAT_EBEP_IMPLEMENTED: usize = 20192usize;
pub const REG_GPTRANGE_64GB: usize = 976usize;
pub const REG_SPESAMPLECONTEXTEL1VALID: usize = 20184usize;
pub const REG_EDECR: usize = 20200usize;
pub const REG_GICR_VPROPBASER: usize = 20208usize;
pub const REG_AMAIR1_S: usize = 20232usize;
pub const REG_U_CSSELR_NS: usize = 20216usize;
pub const REG_U_MVFR0: usize = 20224usize;
pub const REG_PMU_EVENT_SAMPLE_FEED_LAT: usize = 272usize;
pub const REG_ID_MMFR5_EL1: usize = 20240usize;
pub const REG_PMCIDR3: usize = 20248usize;
pub const REG_U_DBGCLAIMCLR: usize = 20256usize;
pub const REG_U_ADFSR_NS: usize = 20264usize;
pub const REG_V8AP6_IMPLEMENTED: usize = 20272usize;
pub const REG_PMU_EVENT_L3D_LFB_HIT_RD: usize = 336usize;
pub const REG_U_HPFAR: usize = 20280usize;
pub const REG_EDPIDR0: usize = 20288usize;
pub const REG_U_DBGOSLSR: usize = 20296usize;
pub const REG_PIRE0_EL1: usize = 20304usize;
pub const REG_FEAT_LRCPC3_IMPLEMENTED: usize = 20312usize;
pub const REG_PMU_EVENT_SAMPLE_FEED_EVENT: usize = 264usize;
pub const REG_FEAT_SVE_AES_IMPLEMENTED: usize = 20320usize;
pub const REG_SPSR_EL3: usize = 20328usize;
pub const REG_GICM_CLRSPI_SR: usize = 20336usize;
pub const REG_U__SYNCABORTONWRITENORMCACHE: usize = 20344usize;
pub const REG_CP15SDISABLE2: usize = 20352usize;
pub const REG_FEAT_CRC32_IMPLEMENTED: usize = 20360usize;
pub const REG_FEAT_TTST_IMPLEMENTED: usize = 20368usize;
pub const REG_TTBCR2_S: usize = 20376usize;
pub const REG_U_ICC_IGRPEN0: usize = 20384usize;
pub const REG_R20: usize = 20392usize;
pub const REG_CNTPS_CTL_EL1: usize = 20400usize;
pub const REG_U_HTPIDR: usize = 20408usize;
pub const REG_GICR_PARTIDR: usize = 20416usize;
pub const REG_FEAT_PMUV3_EXT_IMPLEMENTED: usize = 20424usize;
pub const REG_R13: usize = 20432usize;
pub const REG_ID_DFR0_EL1: usize = 20440usize;
pub const REG_GICD_CLRSPI_SR: usize = 20448usize;
pub const REG_PMMIR_EL1: usize = 20456usize;
pub const REG_DBGEN: usize = 20464usize;
pub const REG_FEAT_IESB_IMPLEMENTED: usize = 20472usize;
pub const REG_FEAT_BTI_IMPLEMENTED: usize = 20480usize;
pub const REG_ICC_SGI1R_EL1: usize = 20488usize;
pub const REG_R30: usize = 20496usize;
pub const REG_PMBLIMITR_EL1: usize = 20504usize;
pub const REG_U_TPIDRPRW_NS: usize = 20512usize;
pub const REG_FEAT_GTG_IMPLEMENTED: usize = 20520usize;
pub const REG_U_CNTHV_CTL: usize = 20528usize;
pub const REG_GITS_MPAMIDR: usize = 20536usize;
pub const REG_U_DBGDTRRXINT: usize = 20544usize;
pub const REG_FEAT_AA32EL0_IMPLEMENTED: usize = 20552usize;
pub const REG_FEAT_DOUBLEFAULT_IMPLEMENTED: usize = 20560usize;
pub const REG_SPEADDRPOSBRANCHTARGET: usize = 1024usize;
pub const REG_U__ISLA_VECTOR_GPR: usize = 20568usize;
pub const REG_U__GICCPUINTERFACEBASE: usize = 20576usize;
pub const REG_RC: usize = 20584usize;
pub const REG_SPECOUNTERPOSTOTALLATENCY: usize = 1056usize;
pub const REG_VMECID_P_EL2: usize = 20624usize;
pub const REG_U__GIC_PENDING: usize = 20632usize;
pub const REG_ICC_DIR_EL1: usize = 20640usize;
pub const REG_GPTBR_EL3: usize = 20648usize;
pub const REG_U_ICC_EOIR0: usize = 20656usize;
pub const REG_U_MAIR0_S: usize = 20664usize;
pub const REG_U_ICC_SRE_S: usize = 20672usize;
pub const REG_FEAT_SPECRES2_IMPLEMENTED: usize = 20680usize;
pub const REG_U__MOPS_FORWARD_COPY: usize = 20688usize;
pub const REG_VMPIDR_EL2: usize = 20696usize;
pub const REG_U_ICV_BPR0: usize = 20704usize;
pub const REG_FEAT_PMUV3_SS_IMPLEMENTED: usize = 20712usize;
pub const REG_FPSR: usize = 20720usize;
pub const REG_U_HIFAR: usize = 20728usize;
pub const REG_U_ICV_EOIR1: usize = 20736usize;
pub const REG_U_HMAIR1: usize = 20744usize;
pub const REG_SPESAMPLEPREVIOUSBRANCHADDRESSVALID: usize = 20752usize;
pub const REG_BRANCHTYPETAKEN: usize = 20760usize;
pub const REG_ICV_AP1R_EL1: usize = 20768usize;
pub const REG_AMAIR2_EL3: usize = 20800usize;
pub const REG_SCTLR_EL2: usize = 20808usize;
pub const REG_VPIDR_EL2: usize = 20816usize;
pub const REG_CNTP_CVAL_EL0: usize = 20824usize;
pub const REG_U_ICV_AP0R: usize = 20832usize;
pub const REG_NUM_BRBE_RECORDS: usize = 20848usize;
pub const REG_GCSPR_EL0: usize = 20864usize;
pub const REG_U__HAS_SVE_EXTENDED_BF16: usize = 20872usize;
pub const REG_GPT_NONSECURE: usize = 880usize;
pub const REG_V8AP2_IMPLEMENTED: usize = 20888usize;
pub const REG_U_ACTLR2_NS: usize = 20896usize;
pub const REG_SPESAMPLECONTEXTEL2: usize = 20904usize;
pub const REG_AMEVTYPER0_EL0: usize = 20912usize;
pub const REG_SCR: usize = 20944usize;
pub const REG_MAIR2_EL2: usize = 20952usize;
pub const REG_GICC_STATUSR: usize = 20960usize;
pub const REG_ID_AA64MMFR4_EL1: usize = 20968usize;
pub const REG_BTYPECOMPATIBLE: usize = 20976usize;
pub const REG_FEAT_S2PIE_IMPLEMENTED: usize = 20984usize;
pub const REG_U_DBGOSDLR: usize = 20992usize;
pub const REG_DBGAUTHSTATUS_EL1: usize = 21000usize;
pub const REG_MPAMVPM7_EL2: usize = 21008usize;
pub const REG_ICH_HCR_EL2: usize = 21016usize;
pub const REG_GICV_DIR: usize = 21024usize;
pub const REG_LST_64B: usize = 1208usize;
pub const REG_FEAT_EBF16_IMPLEMENTED: usize = 21032usize;
pub const REG_PMCR_EL0: usize = 21040usize;
pub const REG_FPEXC32_EL2: usize = 21048usize;
pub const REG_ICV_HPPIR1_EL1: usize = 21056usize;
pub const REG_FEAT_FP16_IMPLEMENTED: usize = 21064usize;
pub const REG_U_TRFCR: usize = 21072usize;
pub const REG_U__EMPAM_SDEFLT_IMPLEMENTED: usize = 21080usize;
pub const REG_CNTHV_TVAL_EL2: usize = 21088usize;
pub const REG_PMSCR_EL1: usize = 21096usize;
pub const REG_ID_AFR0_EL1: usize = 21104usize;
pub const REG_DBGCLAIMCLR_EL1: usize = 21112usize;
pub const REG_APIAKEYLO_EL1: usize = 21120usize;
pub const REG_FEAT_UAO_IMPLEMENTED: usize = 21128usize;
pub const REG_SDER32_EL2: usize = 21136usize;
pub const REG_EDDFR1: usize = 21144usize;
pub const REG_FEAT_GICV3_NMI_IMPLEMENTED: usize = 21152usize;
pub const REG_SPSR_MON: usize = 21160usize;
pub const REG_U__MPAM_HAS_ALTSP: usize = 21168usize;
pub const REG_PMU_EVENT_L2D_CACHE_LMISS_RD: usize = 184usize;
pub const REG_ICV_AP0R_EL1: usize = 21176usize;
pub const REG_SCXTNUM_EL3: usize = 21208usize;
pub const REG_U__MPAM_VPMR_MAX: usize = 21216usize;
pub const REG_R18: usize = 21224usize;
pub const REG_U__SGI_BASE: usize = 21232usize;
pub const REG_R0: usize = 21240usize;
pub const REG_V9AP3_IMPLEMENTED: usize = 21248usize;
pub const REG_U__APPLY_EFFECTIVE_SHAREABILITY: usize = 21256usize;
pub const REG_RECORDS_SRC: usize = 21264usize;
pub const REG_U_DFAR_S: usize = 21776usize;
pub const REG_HAFGRTR_EL2: usize = 21784usize;
pub const REG_U__SYNCABORTONREADNORMCACHE: usize = 21792usize;
pub const REG_LOREA_EL1: usize = 21800usize;
pub const REG_AMAIR2_EL1: usize = 21808usize;
pub const REG_ERRSELR_EL1: usize = 21816usize;
pub const REG_ICC_MCTLR: usize = 21824usize;
pub const REG_U__MPAM_PARTID_MAX: usize = 21832usize;
pub const REG_FEAT_RDM_IMPLEMENTED: usize = 21840usize;
pub const REG_U__SYNCABORTONDEVICEWRITE: usize = 21848usize;
pub const REG_FEAT_ETMV4P6_IMPLEMENTED: usize = 21856usize;
pub const REG_R27: usize = 21864usize;
pub const REG_U_DORMANTCTLREG: usize = 21872usize;
pub const REG_U_ID_MMFR0: usize = 21880usize;
pub const REG_U_ERXADDR: usize = 21888usize;
pub const REG_EDITCTRL: usize = 21896usize;
pub const REG_U__IGNORE_RVBAR_IN_AARCH32: usize = 21904usize;
pub const REG_CNTP_CTL_S: usize = 21912usize;
pub const REG_FEAT_EL2_IMPLEMENTED: usize = 21920usize;
pub const REG_CTICONTROL: usize = 21928usize;
pub const REG_GCSPR_EL1: usize = 21936usize;
pub const REG_U__CURRENTCOND: usize = 21944usize;
pub const REG_BRBSRCINJ_EL1: usize = 21952usize;
pub const REG_CONTEXTIDR_S: usize = 21960usize;
pub const REG_GITS_STATUSR: usize = 21968usize;
pub const REG_U_HCR2: usize = 21976usize;
pub const REG_AMCIDR0: usize = 21984usize;
pub const REG_EVENTREGISTER: usize = 21992usize;
pub const REG_FEAT_ETS2_IMPLEMENTED: usize = 22000usize;
pub const REG_U_DBGPRCR: usize = 22008usize;
pub const REG_U_DLR: usize = 22016usize;
pub const REG_FEAT_SME_IMPLEMENTED: usize = 22024usize;
pub const REG_U__SPE_LFSR: usize = 22032usize;
pub const REG_MEMHINT_RWA: usize = 512usize;
pub const REG_CNTSCR: usize = 22040usize;
pub const REG_U_AMEVCNTR0_EL0: usize = 22048usize;
pub const REG_CNTKCTL_EL1: usize = 22080usize;
pub const REG_CFG_ID_AA64PFR0_EL1_EL2: usize = 1240usize;
pub const REG_U__ISB_IS_BRANCH: usize = 22088usize;
pub const REG_GICR_MPAMIDR: usize = 22096usize;
pub const REG_LORID_EL1: usize = 22104usize;
pub const REG_U_ICC_SRE_NS: usize = 22112usize;
pub const REG_U_ICV_IGRPEN0: usize = 22120usize;
pub const REG_FEAT_DPB2_IMPLEMENTED: usize = 22128usize;
pub const REG_ID_AA64MMFR3_EL1: usize = 22136usize;
pub const REG_BRBINF_EL1: usize = 22144usize;
pub const REG_GICH_ELRSR: usize = 22400usize;
pub const REG_GICH_MISR: usize = 22408usize;
pub const REG_TCR_EL1: usize = 22416usize;
pub const REG_CNTVOFF_EL2: usize = 22424usize;
pub const REG_VTTBR: usize = 22432usize;
pub const REG_UART_BASE: usize = 1352usize;
pub const REG_SPESAMPLEINFLIGHT: usize = 22440usize;
pub const REG_REVIDR_EL1: usize = 22448usize;
pub const REG_U_DBGBXVR: usize = 22456usize;
pub const REG_TPIDRURW_S: usize = 22520usize;
pub const REG_AMCIDR3: usize = 22528usize;
pub const REG_PMU_EVENT_L1D_CACHE: usize = 32usize;
pub const REG_FEAT_XS_IMPLEMENTED: usize = 22536usize;
pub const REG_MPAMVPM4_EL2: usize = 22544usize;
pub const REG_HCRX_EL2: usize = 22552usize;
pub const REG_OSDTRTX_EL1: usize = 22560usize;
pub const REG_MPAMVPM6_EL2: usize = 22568usize;
pub const REG_ID_AA64PFR1_EL1: usize = 22576usize;
pub const REG_ERXPFGF_EL1: usize = 22584usize;
pub const REG_FEAT_NV2_IMPLEMENTED: usize = 22592usize;
pub const REG_MAX_VL: usize = 808usize;
pub const REG_FEAT_HAFDBS_IMPLEMENTED: usize = 22600usize;
pub const REG_FEAT_PAUTH_IMPLEMENTED: usize = 22608usize;
pub const REG_ICH_EISR_EL2: usize = 22616usize;
pub const REG_ERXMISC0_EL1: usize = 22624usize;
pub const REG_JOSCR: usize = 22632usize;
pub const REG_AMAIR2_EL2: usize = 22640usize;
pub const REG_PMAUTHSTATUS: usize = 22648usize;
pub const REG_PMCNTENCLR_EL0: usize = 22656usize;
pub const REG_U__LAST_CYCLE_COUNT: usize = 22664usize;
pub const REG_FEAT_F64MM_IMPLEMENTED: usize = 22680usize;
pub const REG_FEAT_PAUTH2_IMPLEMENTED: usize = 22688usize;
pub const REG_CNTHPS_CVAL_EL2: usize = 22696usize;
pub const REG_U__TRCCLAIM_TAGS: usize = 22704usize;
pub const REG_AFSR1_EL1: usize = 22712usize;
pub const REG_U_AMCNTENCLR1: usize = 22720usize;
pub const REG_GICD_SETSPI_NSR: usize = 22728usize;
pub const REG_MDCR_EL3: usize = 22736usize;
pub const REG_U_VMPIDR: usize = 22744usize;
pub const REG_GICV_AHPPIR: usize = 22752usize;
pub const REG_DEBUGHALT_BREAKPOINT: usize = 1096usize;
pub const REG_AMPIDR0: usize = 22760usize;
pub const REG_PMSEVFR_EL1: usize = 22768usize;
pub const REG_SPEMAXRECORDSIZE: usize = 1008usize;
pub const REG_V8AP7_IMPLEMENTED: usize = 22776usize;
pub const REG_U__INSTRUCTIONSTEP: usize = 22784usize;
pub const REG_FEAT_SVE2P1_IMPLEMENTED: usize = 22792usize;
pub const REG_NUM_BREAKPOINTS: usize = 22800usize;
pub const REG_AMCNTENCLR0_EL0: usize = 22816usize;
pub const REG_EDDFR: usize = 22824usize;
pub const REG_U__SPE_LFSR_INITIALIZED: usize = 22832usize;
pub const REG_VBAR_EL2: usize = 22840usize;
pub const REG_VSTTBR_EL2: usize = 22848usize;
pub const REG_EDVIDSR: usize = 22856usize;
pub const REG_PMZR_EL0: usize = 22864usize;
pub const REG_ADFSR_S: usize = 22872usize;
pub const REG_U_ID_PFR2: usize = 22880usize;
pub const REG_U_ICC_AP1R_S: usize = 22888usize;
pub const REG_PMU_EVENT_L1D_TLB: usize = 96usize;
pub const REG_U_ICC_SGI1R: usize = 22904usize;
pub const REG_U_CNTFRQ: usize = 22912usize;
pub const REG_CSSELR_EL1: usize = 22920usize;
pub const REG_DEBUGHALT_WATCHPOINT: usize = 1144usize;
pub const REG_MECID_P0_EL2: usize = 22928usize;
pub const REG_CNTFRQ_EL0: usize = 22936usize;
pub const REG_MAIR_EL1: usize = 22944usize;
pub const REG_R5: usize = 22952usize;
pub const REG_U_HRMR: usize = 22960usize;
pub const REG_MAX_PL: usize = 816usize;
pub const REG_U_HACTLR2: usize = 22968usize;
pub const REG_ESR_EL1: usize = 22976usize;
pub const REG_ICC_SRE_EL1_NS: usize = 22984usize;
pub const REG_U_PAR_EL1: usize = 22992usize;
pub const REG_R3: usize = 23008usize;
pub const REG_SHOULDADVANCESS: usize = 23016usize;
pub const REG_FEAT_SME_F64F64_IMPLEMENTED: usize = 23024usize;
pub const REG_BRBTS_EL1: usize = 23032usize;
pub const REG_U_ICV_AP1R: usize = 23040usize;
pub const REG_FEAT_MTE4_IMPLEMENTED: usize = 23056usize;
pub const REG_U_DBGDSCRINT: usize = 23064usize;
pub const REG_U_DSPSR2: usize = 23072usize;
pub const REG_GPTRANGE_2MB: usize = 936usize;
pub const REG_SPESAMPLECOUNTERVALID: usize = 23080usize;
pub const REG_U_DISR: usize = 23112usize;
pub const REG_R26: usize = 23120usize;
pub const REG_VBAR_EL3: usize = 23128usize;
pub const REG_MECID_A1_EL2: usize = 23136usize;
pub const REG_RMR_EL2: usize = 23144usize;
pub const REG_U_ID_DFR1: usize = 23152usize;
pub const REG_U_ICV_PMR: usize = 23160usize;
pub const REG_U_CNTV_CVAL: usize = 23168usize;
pub const REG_R10: usize = 23176usize;
pub const REG_FEAT_BF16_IMPLEMENTED: usize = 23184usize;
pub const REG_FEAT_THE_IMPLEMENTED: usize = 23192usize;
pub const REG_CFG_PMCR_IDCODE: usize = 1256usize;
pub const REG_TTBR0_EL3: usize = 23200usize;
pub const REG_ICC_IAR1_EL1: usize = 23208usize;
pub const REG_R16: usize = 23216usize;
pub const REG_DOMAIN_CLIENT: usize = 792usize;
pub const REG_U_PMOVSSET: usize = 23224usize;
pub const REG_U_DBGDTRTXEXT: usize = 23232usize;
pub const REG_CTICIDR3: usize = 23240usize;
pub const REG_FEAT_PMUV3_EXT64_IMPLEMENTED: usize = 23248usize;
pub const REG_FEAT_SEBEP_IMPLEMENTED: usize = 23256usize;
pub const REG_U_REVIDR: usize = 23264usize;
pub const REG_PMU_EVENT_L1D_CACHE_HITM_RD: usize = 288usize;
pub const REG_FEAT_I8MM_IMPLEMENTED: usize = 23272usize;
pub const REG_U__CNTEL0BASEN: usize = 23280usize;
pub const REG_FEAT_ETE_IMPLEMENTED: usize = 23288usize;
pub const REG_U__GICDISTBASE: usize = 23296usize;
pub const REG_CCSIDR_EL1: usize = 23304usize;
pub const REG_FEAT_EPAC_IMPLEMENTED: usize = 23312usize;
pub const REG_U_DBGWVR: usize = 23320usize;
pub const REG_PMU_EVENT_LL_CACHE: usize = 112usize;
pub const REG_U__FEAT_RPRES: usize = 23384usize;
pub const REG_ID_ISAR3_EL1: usize = 23392usize;
pub const REG_U__GMID_LOG2_BLOCK_SIZE: usize = 23400usize;
pub const REG_GICM_SETSPI_SR: usize = 23416usize;
pub const REG_GITS_SGIR: usize = 23424usize;
pub const REG_U__PMUBASE: usize = 23432usize;
pub const REG_U_VDFSR: usize = 23440usize;
pub const REG_PMU_EVENT_INST_RETIRED: usize = 40usize;
pub const REG_TPIDR_EL0: usize = 23448usize;
pub const REG_EDDEVID: usize = 23456usize;
pub const REG_GICV_EOIR: usize = 23464usize;
pub const REG_ICV_DIR_EL1: usize = 23472usize;
pub const REG_U_HTCR: usize = 23480usize;
pub const REG_U_PMEVTYPER: usize = 23488usize;
pub const REG_ERXPFGCTL_EL1: usize = 23616usize;
pub const REG_U_PMCEID1: usize = 23624usize;
pub const REG_U_AMCNTENCLR0: usize = 23632usize;
pub const REG_RCWMASK_EL1: usize = 23640usize;
pub const REG_CNTV_CVAL_EL0: usize = 23656usize;
pub const REG_U__CPY_MOPS_OPTION_A_SUPPORTED: usize = 23664usize;
pub const REG_BRBSRC_EL1: usize = 23672usize;
pub const REG_GITS_IIDR: usize = 23928usize;
pub const REG_R24: usize = 23936usize;
pub const REG_FEAT_CSV2_IMPLEMENTED: usize = 23944usize;
pub const REG_RNDR: usize = 23952usize;
pub const REG_U__SYNCABORTONSOWRITE: usize = 23960usize;
pub const REG_GICM_IIDR: usize = 23968usize;
pub const REG_U_ZA: usize = 23976usize;
pub const REG_GICD_TYPER: usize = 89512usize;
pub const REG_RMR_EL1: usize = 89520usize;
pub const REG_GICC_PMR: usize = 89528usize;
pub const REG_PMU_EVENT_CPU_CYCLES: usize = 64usize;
pub const REG_FEAT_MTE_IMPLEMENTED: usize = 89536usize;
pub const REG_FEAT_MPAMV0P1_IMPLEMENTED: usize = 89544usize;
pub const REG_U__CPYF_MOPS_OPTION_A_SUPPORTED: usize = 89552usize;
pub const REG_ICV_EOIR1_EL1: usize = 89560usize;
pub const REG_ICC_MGRPEN1: usize = 89568usize;
pub const REG_U_ERXCTLR2: usize = 89576usize;
pub const REG_PIR_EL2: usize = 89584usize;
pub const REG_FEAT_SPECRES_IMPLEMENTED: usize = 89592usize;
pub const REG_U_CNTHP_CTL: usize = 89600usize;
pub const REG_FEAT_TRBE_EXT_IMPLEMENTED: usize = 89608usize;
pub const REG_RVBAR_EL2: usize = 89616usize;
pub const REG_U_ID_MMFR2: usize = 89624usize;
pub const REG_ID_MMFR0_EL1: usize = 89632usize;
pub const REG_PMU_EVENT_L2D_CACHE_HITM_RD: usize = 296usize;
pub const REG_FEAT_XNX_IMPLEMENTED: usize = 89640usize;
pub const REG_PMU_EVENT_SVE_PRED_PARTIAL_SPEC: usize = 208usize;
pub const REG_AMAIR_EL1: usize = 89648usize;
pub const REG_PMUEVENTACCUMULATOR: usize = 89656usize;
pub const REG_SP_EL0: usize = 90152usize;
pub const REG_U_ICH_VMCR: usize = 90160usize;
pub const REG_U__MPAM_MAJOR: usize = 90168usize;
pub const REG_FEAT_E0PD_IMPLEMENTED: usize = 90176usize;
pub const REG_EDPIDR4: usize = 90184usize;
pub const REG_MECID_P1_EL2: usize = 90192usize;
pub const REG_U_DBGBCR: usize = 90200usize;
pub const REG_FEAT_GICV3_LEGACY_IMPLEMENTED: usize = 90264usize;
pub const REG_SMPRIMAP_EL2: usize = 90272usize;
pub const REG_U__SUPPORTED_PA_SIZE: usize = 90280usize;
pub const REG_SCTLR_EL1: usize = 90296usize;
pub const REG_U__SYNCABORTONDEVICEREAD: usize = 90304usize;
pub const REG_FEAT_DEBUGV8P1_IMPLEMENTED: usize = 90312usize;
pub const REG_GPT_TABLE: usize = 848usize;
pub const REG_FEAT_TME_IMPLEMENTED: usize = 90320usize;
pub const REG_DBGPRCR_EL1: usize = 90328usize;
pub const REG_ID_MMFR4_EL1: usize = 90336usize;
pub const REG_PMINTENSET_EL1: usize = 90344usize;
pub const REG_V8AP4_IMPLEMENTED: usize = 90352usize;
pub const REG_U_CNTHPS_CTL: usize = 90360usize;
pub const REG_CFG_MPAM_FRAC_V1P1: usize = 1304usize;
pub const REG_ICV_BPR0_EL1: usize = 90368usize;
pub const REG_CPTR_EL3_ESM_VALUE: usize = 90376usize;
pub const REG_FEAT_AFP_IMPLEMENTED: usize = 90392usize;
pub const REG_GITS_CWRITER: usize = 90400usize;
pub const REG_PMU_EVENT_SW_INCR: usize = 16usize;
pub const REG_DEBUGHALT_EDBGRQ: usize = 1104usize;
pub const REG_U_ICC_IGRPEN1_NS: usize = 90408usize;
pub const REG_U__MPAM_HAS_HCR: usize = 90416usize;
pub const REG_U__EMPAM_FORCE_NS_RAO: usize = 90424usize;
pub const REG_ID_ISAR6_EL1: usize = 90432usize;
pub const REG_SCXTNUM_EL0: usize = 90440usize;
pub const REG_MVFR0_EL1: usize = 90448usize;
pub const REG_GIC_PENDING_NONE: usize = 1384usize;
pub const REG_U_DFAR_NS: usize = 90456usize;
pub const REG_U_HACR: usize = 90464usize;
pub const REG_FEAT_PMUV3P8_IMPLEMENTED: usize = 90472usize;
pub const REG_U_DBGCLAIMSET: usize = 90480usize;
pub const REG_GICR_INMIR0: usize = 90488usize;
pub const REG_NUM_AMU_CG0_MONITORS: usize = 90496usize;
pub const REG_PMU_EVENT_L1D_LFB_HIT_RD: usize = 320usize;
pub const REG_CTIPIDR4: usize = 90512usize;
pub const REG_AMUSERENR_EL0: usize = 90520usize;
pub const REG_MPAM2_EL2: usize = 90528usize;
pub const REG_PMBPTR_EL1: usize = 90536usize;
pub const REG_U_ZT0: usize = 90544usize;
pub const REG_FEAT_SVE_SHA3_IMPLEMENTED: usize = 90608usize;
pub const REG_U_HSTR: usize = 90616usize;
pub const REG_ID_AA64MMFR2_EL1: usize = 90624usize;
pub const REG_ID_AA64ISAR0_EL1: usize = 90632usize;
pub const REG_U_DBGOSECCR: usize = 90640usize;
pub const REG_AMPIDR4: usize = 90648usize;
pub const REG_ICC_SGI0R_EL1: usize = 90656usize;
pub const REG_BRBCR_EL1: usize = 90664usize;
pub const REG_SPSR_EL1: usize = 90672usize;
pub const REG_U_PMCR: usize = 90680usize;
pub const REG_U_ICC_IGRPEN1_S: usize = 90688usize;
pub const REG_U_ICH_EISR: usize = 90696usize;
pub const REG_U__GIC_ACTIVE: usize = 90704usize;
pub const REG_ESR_EL2: usize = 90712usize;
pub const REG_FEAT_PAN2_IMPLEMENTED: usize = 90720usize;
pub const REG_SCR_EL3: usize = 90728usize;
pub const REG_PAR_S: usize = 90736usize;
pub const REG_FEAT_WFXT_IMPLEMENTED: usize = 90744usize;
pub const REG_RCW64_PROTECTED_BIT: usize = 1184usize;
pub const REG_ID_MMFR3_EL1: usize = 90752usize;
pub const REG_CSSELR_S: usize = 90760usize;
pub const REG_U_ICC_HSRE: usize = 90768usize;
pub const REG_CNTNSAR: usize = 90776usize;
pub const REG_FEAT_PMUV3_TH_IMPLEMENTED: usize = 90784usize;
pub const REG_FEAT_HBC_IMPLEMENTED: usize = 90792usize;
pub const REG_FEAT_SME_F16F16_IMPLEMENTED: usize = 90800usize;
pub const REG_M32_UNDEF: usize = 408usize;
pub const REG_PMU_EVENT_DTLB_WALK: usize = 128usize;
pub const REG_NUM_AMU_CG1_MONITORS: usize = 90808usize;
pub const REG_OSLAR_EL1: usize = 90824usize;
pub const REG_MECIDR_EL2: usize = 90832usize;
pub const REG_MVFR2_EL1: usize = 90840usize;
pub const REG_PMU_EVENT_INST_SPEC: usize = 72usize;
pub const REG_EL3: usize = 424usize;
pub const REG_U_PMCEID3: usize = 90848usize;
pub const REG_CNTP_CTL_EL0: usize = 90856usize;
pub const REG_FEAT_CLRBHB_IMPLEMENTED: usize = 90864usize;
pub const REG_FEAT_MTE2_IMPLEMENTED: usize = 90872usize;
pub const REG_U_PMCNTENCLR: usize = 90880usize;
pub const REG_MPAMVPM3_EL2: usize = 90888usize;
pub const REG_ID_MMFR1_EL1: usize = 90896usize;
pub const REG_ICV_NMIAR1_EL1: usize = 90904usize;
pub const REG_FEAT_SVE_B16B16_IMPLEMENTED: usize = 90912usize;
pub const REG_DEBUGEXCEPTION_WATCHPOINT: usize = 1336usize;
pub const REG_V9AP2_IMPLEMENTED: usize = 90920usize;
pub const REG_FEAT_FPACCOMBINE_IMPLEMENTED: usize = 90928usize;
pub const REG_BTYPENEXT: usize = 90936usize;
pub const REG_FEAT_MTE_CANONICAL_TAGS_IMPLEMENTED: usize = 90944usize;
pub const REG_SMCR_EL3_LEN_VALUE: usize = 90952usize;
pub const REG_ID_PFR1_EL1: usize = 90968usize;
pub const REG_U_ERXMISC6: usize = 90976usize;
pub const REG_SMCR_EL3: usize = 90984usize;
pub const REG_SP_EL2: usize = 90992usize;
pub const REG_U_ICV_EOIR0: usize = 91000usize;
pub const REG_PMU_EVENT_DSNP_HIT_RD: usize = 280usize;
pub const REG_FEAT_SVE_SM4_IMPLEMENTED: usize = 91008usize;
pub const REG_U_CNTVOFF: usize = 91016usize;
pub const REG_U__MTE_IMPLEMENTED: usize = 91024usize;
pub const REG_CONTEXTIDR_EL2: usize = 91032usize;
pub const REG_SPSR_IRQ: usize = 91040usize;
pub const REG_U_TTBR0_EL2: usize = 91048usize;
pub const REG_JMCR: usize = 91064usize;
pub const REG_MEMHINT_WA: usize = 496usize;
pub const REG_ICV_IAR1_EL1: usize = 91072usize;
pub const REG_EL0: usize = 448usize;
pub const REG_U__EMPAM_FORCE_NS_IMPLEMENTED: usize = 91080usize;
pub const REG_U_SPSR_HYP: usize = 91088usize;
pub const REG_SPEMAXADDRS: usize = 992usize;
pub const REG_ICC_AP0R_EL1: usize = 91096usize;
pub const REG_GICC_RPR: usize = 91128usize;
pub const REG_U_HACTLR: usize = 91136usize;
pub const REG_GICR_ISENABLER0: usize = 91144usize;
pub const REG_SMPRI_EL1: usize = 91152usize;
pub const REG_TSTATE: usize = 91160usize;
pub const REG_MVBAR: usize = 100232usize;
pub const REG_GPTRANGE_16GB: usize = 968usize;
pub const REG_U__GICC_IIDR: usize = 1376usize;
pub const REG_CNTV_TVAL_EL0: usize = 100240usize;
pub const REG_MPAMVPM0_EL2: usize = 100248usize;
pub const REG_VARIANTIMPLEMENTED: usize = 100256usize;
pub const REG_U_DBGVCR: usize = 100272usize;
pub const REG_ID_AA64SMFR0_EL1: usize = 100280usize;
pub const REG_FEAT_PMULL_IMPLEMENTED: usize = 100288usize;
pub const REG_FEAT_PAN_IMPLEMENTED: usize = 100296usize;
pub const REG_GPT_SECURE: usize = 872usize;
pub const REG_MFAR_EL3: usize = 100304usize;
pub const REG_RECORDS_INF: usize = 100312usize;
pub const REG_CTIPIDR3: usize = 100824usize;
pub const REG_FEAT_FPAC_IMPLEMENTED: usize = 100832usize;
pub const REG_GMID_EL1: usize = 100840usize;
pub const REG_VSESR_EL2: usize = 100848usize;
pub const REG_CNTHPS_TVAL_EL2: usize = 100856usize;
pub const REG_NMRR_S: usize = 100864usize;
pub const REG_U_ID_MMFR4: usize = 100872usize;
pub const REG_U_ICH_VTR: usize = 100880usize;
pub const REG_EDDEVID1: usize = 100888usize;
pub const REG_PMCIDR1: usize = 100896usize;
pub const REG_GICR_INVALLR: usize = 100904usize;
pub const REG_FEAT_EDHSR_IMPLEMENTED: usize = 100912usize;
pub const REG_FEAT_NV_IMPLEMENTED: usize = 100920usize;
pub const REG_PMU_EVENT_SVE_PRED_EMPTY_SPEC: usize = 200usize;
pub const REG_FEAT_SYSINSTR128_IMPLEMENTED: usize = 100928usize;
pub const REG_CNTHP_CTL_EL2: usize = 100936usize;
pub const REG_APIBKEYHI_EL1: usize = 100944usize;
pub const REG_CNTP_TVAL_EL0: usize = 100952usize;
pub const REG_FEAT_S2FWB_IMPLEMENTED: usize = 100960usize;
pub const REG_FEAT_AA32EL2_IMPLEMENTED: usize = 100968usize;
pub const REG_R8: usize = 100976usize;
pub const REG_U_ICC_CTLR_NS: usize = 100984usize;
pub const REG_U_EDECCR: usize = 100992usize;
pub const REG_CCSIDR2_EL1: usize = 101000usize;
pub const REG_VMID_NONE: usize = 1080usize;
pub const REG_MPAMVPM5_EL2: usize = 101008usize;
pub const REG_HFGWTR_EL2: usize = 101016usize;
pub const REG_SMIDR_EL1: usize = 101024usize;
pub const REG_U_ERXMISC2: usize = 101032usize;
pub const REG_FEAT_LS64_ACCDATA_IMPLEMENTED: usize = 101040usize;
pub const REG_FEAT_ITE_IMPLEMENTED: usize = 101048usize;
pub const REG_CTIDEVARCH: usize = 101056usize;
pub const REG_S2POR_EL1: usize = 101064usize;
pub const REG_PMU_EVENT_LL_LFB_HIT_RD: usize = 344usize;
pub const REG_GICD_CLRSPI_NSR: usize = 101072usize;
pub const REG_GCSCR_EL1: usize = 101080usize;
pub const REG_M32_SVC: usize = 376usize;
pub const REG_FEAT_GCS_IMPLEMENTED: usize = 101088usize;
pub const REG_FEAT_DEBUGV8P4_IMPLEMENTED: usize = 101096usize;
pub const REG_U_TTBCR_NS: usize = 101104usize;
pub const REG_LORN_EL1: usize = 101112usize;
pub const REG_FEAT_PACQARMA3_IMPLEMENTED: usize = 101120usize;
pub const REG_U_RMR: usize = 101128usize;
pub const REG_SPECOUNTERPOSISSUELATENCY: usize = 1064usize;
pub const REG_FEAT_PMUV3P7_IMPLEMENTED: usize = 101136usize;
pub const REG_R7: usize = 101144usize;
pub const REG_U__EMULATOR_TERMINATION_OPCODE: usize = 101152usize;
pub const REG_U_PMOVSR: usize = 101168usize;
pub const REG_GPTRANGE_1GB: usize = 960usize;
pub const REG_U__MONOMORPHIZE_WRITES: usize = 101176usize;
pub const REG_U__EXCLUSIVEMONITORSET: usize = 101184usize;
pub const REG_FEAT_FLAGM_IMPLEMENTED: usize = 101192usize;
pub const REG_TLBTR: usize = 101200usize;
pub const REG_FEAT_SHA3_IMPLEMENTED: usize = 101208usize;
pub const REG_GPTRANGE_512GB: usize = 984usize;
pub const REG_FEAT_TLBIRANGE_IMPLEMENTED: usize = 101216usize;
pub const REG_ISWFISLEEP: usize = 101224usize;
pub const REG_PMSFCR_EL1: usize = 101232usize;
pub const REG_ICC_IGRPEN1_EL1_S: usize = 101240usize;
pub const REG_HDFGRTR2_EL2: usize = 101248usize;
pub const REG_CTIPIDR1: usize = 101256usize;
pub const REG_U_MPIDR: usize = 101264usize;
pub const REG_RECORDS_TGT: usize = 101272usize;
pub const REG_EDPIDR3: usize = 101784usize;
pub const REG_EDDEVID2: usize = 101792usize;
pub const REG_PMIAR_EL1: usize = 101800usize;
pub const REG_GICR_PROPBASER: usize = 101808usize;
pub const REG_V9AP4_IMPLEMENTED: usize = 101816usize;
pub const REG_TTBR0_S: usize = 101824usize;
pub const REG_GICV_CTLR: usize = 101832usize;
pub const REG_PMSICR_EL1: usize = 101840usize;
pub const REG_ID_AA64PFR0_EL1: usize = 101848usize;
pub const REG_FEAT_TTL_IMPLEMENTED: usize = 101856usize;
pub const REG_FEAT_LS64_IMPLEMENTED: usize = 101864usize;
pub const REG_FEAT_HPDS_IMPLEMENTED: usize = 101872usize;
pub const REG_V8AP9_IMPLEMENTED: usize = 101880usize;
pub const REG_U_DBGDTRTXINT: usize = 101888usize;
pub const REG_JIDR: usize = 101896usize;
pub const REG_DBGWFAR: usize = 101904usize;
pub const REG_GICV_AIAR: usize = 101912usize;
pub const REG_ZCR_EL1: usize = 101920usize;
pub const REG_FEAT_ETMV4_IMPLEMENTED: usize = 101928usize;
pub const REG_RMR_EL3: usize = 101936usize;
pub const REG_AMCNTENCLR1_EL0: usize = 101944usize;
pub const REG_PMEVCNTSVR_EL1: usize = 101952usize;
pub const REG_NUM_GIC_PRIORITY_BITS: usize = 102200usize;
pub const REG_U_ICV_HPPIR0: usize = 102216usize;
pub const REG_DEBUGEXCEPTION_VECTORCATCH: usize = 1328usize;
pub const REG_PMLSR: usize = 102224usize;
pub const REG_DCZID_EL0: usize = 102232usize;
pub const REG_U_ICV_IGRPEN1: usize = 102240usize;
pub const REG_U__DCACHE_CCSIDR_RESET: usize = 102248usize;
pub const REG_FEAT_RPRFM_IMPLEMENTED: usize = 102304usize;
pub const REG_DBGVCR32_EL2: usize = 102312usize;
pub const REG_CTIDEVID: usize = 102320usize;
pub const REG_BRBTGTINJ_EL1: usize = 102328usize;
pub const REG_FEAT_DOUBLELOCK_IMPLEMENTED: usize = 102336usize;
pub const REG_U_ID_MMFR3: usize = 102344usize;
pub const REG_U_SDER: usize = 102352usize;
pub const REG_FEAT_SM4_IMPLEMENTED: usize = 102360usize;
pub const REG_MPAMSM_EL1: usize = 102368usize;
pub const REG_M32_HYP: usize = 400usize;
pub const REG_FEAT_TRF_IMPLEMENTED: usize = 102376usize;
pub const REG_PIRE0_EL2: usize = 102384usize;
pub const REG_U_ICC_HPPIR1: usize = 102392usize;
pub const REG_EDCIDR0: usize = 102400usize;
pub const REG_FEAT_CNTSC_IMPLEMENTED: usize = 102408usize;
pub const REG_U__TRICKBOX_ENABLED: usize = 102416usize;
pub const REG_MEMHINT_RA: usize = 504usize;
pub const REG_AMPIDR3: usize = 102424usize;
pub const REG_GPTRANGE_512MB: usize = 952usize;
pub const REG_FEAT_CCIDX_IMPLEMENTED: usize = 102432usize;
pub const REG_U_ICC_DIR: usize = 102440usize;
pub const REG_PMLAR: usize = 102448usize;
pub const REG_GPT_ROOT: usize = 888usize;
pub const REG_FEAT_SM3_IMPLEMENTED: usize = 102456usize;
pub const REG_CFG_RVBAR: usize = 102464usize;
pub const REG_U_FPEXC: usize = 102472usize;
pub const REG_PMU_EVENT_LL_CACHE_MISS: usize = 120usize;
pub const REG_PMU_EVENT_L1D_CACHE_REFILL: usize = 24usize;
pub const REG_ICV_BPR1_EL1: usize = 102480usize;
pub const REG_ACCDATA_EL1: usize = 102488usize;
pub const REG_ERXMISC2_EL1: usize = 102496usize;
pub const REG_FEAT_VHE_IMPLEMENTED: usize = 102504usize;
pub const REG_NSACR: usize = 102512usize;
pub const REG_U__CTIBASE: usize = 102520usize;
pub const REG_CTILSR: usize = 102528usize;
pub const REG_U_ISR: usize = 102536usize;
pub const REG_INGUARDEDPAGE: usize = 102544usize;
pub const REG_SPEADDRPOSDATAPHYSICAL: usize = 1040usize;
pub const REG_ICC_BPR1_EL1_S: usize = 102552usize;
pub const REG_EL2: usize = 432usize;
pub const REG_U_ERRSELR: usize = 102560usize;
pub const REG_GICV_AEOIR: usize = 102568usize;
pub const REG_HCR_EL2: usize = 102576usize;
pub const REG_ZT0_LEN: usize = 824usize;
pub const REG_ID_ISAR2_EL1: usize = 102584usize;
pub const REG_MECID_RL_A_EL3: usize = 102592usize;
pub const REG_FEAT_EL0_IMPLEMENTED: usize = 102600usize;
pub const REG_DSPSR_EL0: usize = 102608usize;
pub const REG_FEAT_D128_IMPLEMENTED: usize = 102616usize;
pub const REG_U_DFSR_NS: usize = 102624usize;
pub const REG_GICD_STATUSR: usize = 102632usize;
pub const REG_FAR_EL2: usize = 102640usize;
pub const REG_PMUSERENR_EL0: usize = 102648usize;
pub const REG_FEAT_SSBS2_IMPLEMENTED: usize = 102656usize;
pub const REG_CFG_ID_AA64PFR0_EL1_EL3: usize = 1248usize;
pub const REG_U_ID_ISAR5: usize = 102664usize;
pub const REG_SPESAMPLECOUNTERPENDING: usize = 102672usize;
pub const REG_SCTLR2_EL2: usize = 102704usize;
pub const REG_POR_EL0: usize = 102712usize;
pub const REG_R12: usize = 102720usize;
pub const REG_FEAT_PRFMSLC_IMPLEMENTED: usize = 102728usize;
pub const REG_R22: usize = 102736usize;
pub const REG_GPT_BLOCK: usize = 856usize;
pub const REG_U_MVFR2: usize = 102744usize;
pub const REG_GICV_PMR: usize = 102752usize;
pub const REG_GICR_INVLPIR: usize = 102760usize;
pub const REG_U_ACTLR_NS: usize = 102768usize;
pub const REG_DEFAULTPMG: usize = 776usize;
pub const REG_FEAT_LOR_IMPLEMENTED: usize = 102776usize;
pub const REG_V9AP1_IMPLEMENTED: usize = 102784usize;
pub const REG_R11: usize = 102792usize;
pub const REG_GPT_ANY: usize = 904usize;
pub const REG_PMICNTSVR_EL1: usize = 102800usize;
pub const REG_HFGRTR2_EL2: usize = 102808usize;
pub const REG_U__NUM_CTX_BREAKPOINTS: usize = 102816usize;
pub const REG_U_ID_AFR0: usize = 102832usize;
pub const REG_U_CONFIGREG: usize = 102840usize;
pub const REG_PAR_NS: usize = 102848usize;
pub const REG_PMDEVID: usize = 102856usize;
pub const REG_PFAR_EL1: usize = 102864usize;
pub const REG_U_HCR: usize = 102872usize;
pub const REG_OSDLR_EL1: usize = 102880usize;
pub const REG_FEAT_SPEV1P4_IMPLEMENTED: usize = 102888usize;
pub const REG_FEATUREIMPL: usize = 102896usize;
pub const REG_ICC_HPPIR0_EL1: usize = 103160usize;
pub const REG_ID_AA64MMFR1_EL1: usize = 103168usize;
pub const REG_CNTHV_CVAL_EL2: usize = 103176usize;
pub const REG_ID_MMFR2_EL1: usize = 103184usize;
pub const REG_U_HCPTR: usize = 103192usize;
pub const REG_SCXTNUM_EL1: usize = 103200usize;
pub const REG_DBGDTRRX_EL0: usize = 103208usize;
pub const REG_U__SETG_MOPS_OPTION_A_SUPPORTED: usize = 103216usize;
pub const REG_CFG_MPAM_V0P1: usize = 1272usize;
pub const REG_U_DSPSR: usize = 103224usize;
pub const REG_EDPRCR: usize = 103232usize;
pub const REG_PMU_EVENT_BR_MIS_PRED_RETIRED: usize = 88usize;
pub const REG_DEBUGHALT_STEP_NOSYNDROME: usize = 1176usize;
pub const REG_FEAT_DIT_IMPLEMENTED: usize = 103240usize;
pub const REG_FEAT_MPAM_IMPLEMENTED: usize = 103248usize;
pub const REG_PMU_EVENT_SAMPLE_FEED_LD: usize = 240usize;
pub const REG_U_ID_ISAR0: usize = 103256usize;
pub const REG_AMEVCNTR1_EL0: usize = 103264usize;
pub const REG_U_HMAIR0: usize = 103392usize;
pub const REG_FEAT_AA32EL1_IMPLEMENTED: usize = 103400usize;
pub const REG_M32_SYSTEM: usize = 416usize;
pub const REG_ERXSTATUS_EL1: usize = 103408usize;
pub const REG_GICH_HCR: usize = 103416usize;
pub const REG_DFSR_S: usize = 103424usize;
pub const REG_M32_FIQ: usize = 360usize;
pub const REG_FEAT_GICV4P1_IMPLEMENTED: usize = 103432usize;
pub const REG_MIDR_EL1: usize = 103440usize;
pub const REG_DBGBVR_EL1: usize = 103448usize;
pub const REG_FEAT_RAS_IMPLEMENTED: usize = 103960usize;
pub const REG_PMSWINC_EL0: usize = 103968usize;
pub const REG_CNTPS_TVAL_EL1: usize = 103976usize;
pub const REG_M32_USER: usize = 352usize;
pub const REG_PMCGCR0: usize = 103984usize;
pub const REG_FEAT_NMI_IMPLEMENTED: usize = 103992usize;
pub const REG_GPTRANGE_64KB: usize = 928usize;
pub const REG_FEAT_LPA2_IMPLEMENTED: usize = 104000usize;
pub const REG_DBGWCR_EL1: usize = 104008usize;
pub const REG_PMICFILTR_EL0: usize = 104520usize;
pub const REG_FEAT_MTE3_IMPLEMENTED: usize = 104528usize;
pub const REG_U_AMCNTENSET1: usize = 104536usize;
pub const REG_U__G1_ACTIVITY_MONITOR_IMPLEMENTED: usize = 104544usize;
pub const REG_FEAT_SPEV1P3_IMPLEMENTED: usize = 104552usize;
pub const REG_GICV_BPR: usize = 104560usize;
pub const REG_FEAT_TTCNP_IMPLEMENTED: usize = 104568usize;
pub const REG_FEAT_LRCPC2_IMPLEMENTED: usize = 104576usize;
pub const REG_U_DBGDCCINT: usize = 104584usize;
pub const REG_SPESAMPLECONTEXTEL1: usize = 104592usize;
pub const REG_U__CNTCONTROLBASE: usize = 104600usize;
pub const REG_GICD_IIDR: usize = 104608usize;
pub const REG_PMPIDR4: usize = 104616usize;
pub const REG_CTIDEVID2: usize = 104624usize;
pub const REG_FEAT_AA32BF16_IMPLEMENTED: usize = 104632usize;
pub const REG_FEAT_BRBE_IMPLEMENTED: usize = 104640usize;
pub const REG_FEAT_AA32I8MM_IMPLEMENTED: usize = 104648usize;
pub const REG_PIR_EL1: usize = 104656usize;
pub const REG_PMOVSSET_EL0: usize = 104664usize;
pub const REG_MDSCR_EL1: usize = 104672usize;
pub const REG_FEAT_ETMV4P3_IMPLEMENTED: usize = 104680usize;
pub const REG_ID_AA64ZFR0_EL1: usize = 104688usize;
pub const REG_U__GICD_TYPER: usize = 1368usize;
pub const REG_U__G1_ACTIVITY_MONITOR_OFFSET_IMPLEMENTED: usize = 104696usize;
pub const REG_ACTLR_EL1: usize = 104704usize;
pub const REG_U_CLIDR: usize = 104712usize;
pub const REG_U__THISINSTR: usize = 104720usize;
pub const REG_U_CNTHVS_CTL: usize = 104728usize;
pub const REG_FEAT_S2POE_IMPLEMENTED: usize = 104736usize;
pub const REG_ID_DFR1_EL1: usize = 104744usize;
pub const REG_U__HAS_SPE_PSEUDO_CYCLES: usize = 104752usize;
pub const REG_PMU_EVENT_REMOTE_ACCESS: usize = 104usize;
pub const REG_FEAT_MTPMU_IMPLEMENTED: usize = 104760usize;
pub const REG_DBGOSLAR: usize = 104768usize;
pub const REG_U__EXTDEBUGBASE: usize = 104776usize;
pub const REG_TFSR_EL2: usize = 104784usize;
pub const REG_TFSR_EL3: usize = 104792usize;
pub const REG_PMCCNTR_EL0: usize = 104800usize;
pub const REG_U_DBGAUTHSTATUS: usize = 104808usize;
pub const REG_SHOULDADVANCEIT: usize = 104816usize;
pub const REG_ID_AA64DFR1_EL1: usize = 104824usize;
pub const REG_AMCNTENSET0_EL0: usize = 104832usize;
pub const REG_U_ICC_BPR1_S: usize = 104840usize;
pub const REG_U_ICC_PMR: usize = 104848usize;
pub const REG_GPTRANGE_4KB: usize = 912usize;
pub const REG_CTIDEVID1: usize = 104856usize;
pub const REG_HFGITR2_EL2: usize = 104864usize;
pub const REG_CYCLE_COUNTER_ID: usize = 0usize;
pub const REG_AMCG1IDR_EL0: usize = 104872usize;
pub const REG_SPESAMPLEEVENTS: usize = 104880usize;
pub const REG_FEAT_DOUBLEFAULT2_IMPLEMENTED: usize = 104888usize;
pub const REG_FAR_EL3: usize = 104896usize;
pub const REG_MDCR_EL2: usize = 104904usize;
pub const REG_PMOVSCLR_EL0: usize = 104912usize;
pub const REG_U__SYNCABORTONWRITENORMNONCACHE: usize = 104920usize;
pub const REG_MVFR1_EL1: usize = 104928usize;
pub const REG_TPIDR2_EL0: usize = 104936usize;
pub const REG_SPNIDEN: usize = 104944usize;
pub const REG_LST_64BV0: usize = 1216usize;
pub const REG_PMSCR_EL2: usize = 104952usize;
pub const REG_HSTR_EL2: usize = 104960usize;
pub const REG_CTICIDR2: usize = 104968usize;
pub const REG_GICV_ABPR: usize = 104976usize;
pub const REG_FEAT_JSCVT_IMPLEMENTED: usize = 104984usize;
pub const REG_FEAT_MPAMV1P1_IMPLEMENTED: usize = 104992usize;
pub const REG_PMU_EVENT_BR_MIS_PRED: usize = 56usize;
pub const REG_M32_IRQ: usize = 368usize;
pub const REG_FEAT_PMUV3P4_IMPLEMENTED: usize = 105000usize;
pub const REG_PMPIDR1: usize = 105008usize;
pub const REG_FEAT_GICV3_TDIR_IMPLEMENTED: usize = 105016usize;
pub const REG_R17: usize = 105024usize;
pub const REG_U_AMAIR0_NS: usize = 105032usize;
pub struct RegisterOffset {
    // Name of the register
    pub name: &'static str,
    // Offset in bytes inside the register
    pub offset: usize,
}
pub fn lookup_register_by_offset(offset: usize) -> Option<RegisterOffset> {
    if offset > core::mem::size_of::<State>() {
        return None;
    }
    Some(
        match REGISTER_NAME_MAP.binary_search_by(|(candidate, _)| candidate.cmp(&offset))
        {
            Ok(idx) => {
                RegisterOffset {
                    name: REGISTER_NAME_MAP[idx].1,
                    offset: 0,
                }
            }
            Err(idx) => {
                let (register_offset, name) = REGISTER_NAME_MAP[idx - 1];
                RegisterOffset {
                    name,
                    offset: offset - register_offset,
                }
            }
        },
    )
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structf71da1a79e9d9d1c {
    pub tuple__pcnt_bool__pcnt_bv320: bool,
    pub tuple__pcnt_bool__pcnt_bv321: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct3cb65eaffadb0720 {
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv10: bool,
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv11: bool,
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv12: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structdc3b504fad5c6519 {
    pub tuple__pcnt_bv__pcnt_bool0: Bits,
    pub tuple__pcnt_bv__pcnt_bool1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct343b2d4a0306013a {
    pub tuple__pcnt_bv__pcnt_union_zoptionzIozK0: Bits,
    pub tuple__pcnt_bv__pcnt_union_zoptionzIozK1: Enumf69731e192b14a6b,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct29530d10a7ab293e {
    pub access: Structce3d4f74f0c035a1,
    pub assuredonly: bool,
    pub debugmoe: u8,
    pub dirtybit: bool,
    pub domain: u8,
    pub extflag: bool,
    pub gpcf: Struct70fb44e0b08fca48,
    pub gpcfs2walk: bool,
    pub ipaddress: Structd1e4e056bb52a442,
    pub level: i128,
    pub merrorstate: u32,
    pub overlay: bool,
    pub paddress: Structd1e4e056bb52a442,
    pub s1tagnotdata: bool,
    pub s2fs1walk: bool,
    pub secondstage: bool,
    pub statuscode: u32,
    pub tagaccess: bool,
    pub toplevel: bool,
    pub write: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structb80fc33b537eeda2 {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv0: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv1: Struct5f3b6da595f30aca,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv2: Struct62e97a2b6f14adb0,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv3: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct9fe441156b18108 {
    pub tuple__pcnt_enum_zConstraint__pcnt_bv2__pcnt_bv1__pcnt_bv1__pcnt_bv20: u32,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv2__pcnt_bv1__pcnt_bv1__pcnt_bv21: u8,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv2__pcnt_bv1__pcnt_bv1__pcnt_bv22: bool,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv2__pcnt_bv1__pcnt_bv1__pcnt_bv23: bool,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv2__pcnt_bv1__pcnt_bv1__pcnt_bv24: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct917b3b33dbf1754b {
    pub bits: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct4fc70f5aac01ee9e {
    pub tuple__pcnt_enum_zConstraint__pcnt_bv50: u32,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv51: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct44caea4e847fe08d {
    pub tuple__pcnt_bv8__pcnt_bool0: u8,
    pub tuple__pcnt_bv8__pcnt_bool1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structb0893c7db9074fbf {
    pub iesb_req: bool,
    pub take_FIQ: bool,
    pub take_IRQ: bool,
    pub take_SE: bool,
    pub take_vFIQ: bool,
    pub take_vIRQ: bool,
    pub take_vSE: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structe89015f692c2dc66 {
    pub attrs: u8,
    pub hints: u8,
    pub transient: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structf0c9a449998805c {
    pub tuple__pcnt_struct_zPhysMemRetStatus__pcnt_bv80: Structc549f9bcfd9a2c5f,
    pub tuple__pcnt_struct_zPhysMemRetStatus__pcnt_bv81: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structc541d5a5aa38657a {
    pub access_kind: Enumfabfd93ef17fdbf2,
    pub pa: Bits,
    pub size: i128,
    pub tag: Enumf69731e192b14a6b,
    pub translation: Enum341d9ce549c82439,
    pub va: Enum3c2b85a331c35c26,
    pub value: Enum3c2b85a331c35c26,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct5f3b6da595f30aca {
    pub fault: Struct29530d10a7ab293e,
    pub mecid: u16,
    pub memattrs: Structad9a367f5ed3ed07,
    pub paddress: Structd1e4e056bb52a442,
    pub s1assured: bool,
    pub s2fs1mro: bool,
    pub tlbcontext: Struct21784028ef9bf8b3,
    pub vaddress: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structcc766a8b1e22a4c4 {
    pub blocksize: i128,
    pub context: Struct21784028ef9bf8b3,
    pub contigsize: i128,
    pub s1descriptor: u128,
    pub s2descriptor: u128,
    pub walkstate: Struct62e97a2b6f14adb0,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structd5e0dcebfc891db1 {
    pub tuple__pcnt_bv__pcnt_bv10: Bits,
    pub tuple__pcnt_bv__pcnt_bv11: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct258b23aa3d588091 {
    pub tuple__pcnt_enum_zConstraint__pcnt_i0: u32,
    pub tuple__pcnt_enum_zConstraint__pcnt_i1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct5de641ba4f5d5b47 {
    pub tuple__pcnt_enum_zFPType__pcnt_bv1__pcnt_real0: u32,
    pub tuple__pcnt_enum_zFPType__pcnt_bv1__pcnt_real1: bool,
    pub tuple__pcnt_enum_zFPType__pcnt_bv1__pcnt_real2: num_rational::Ratio<i128>,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct604e5f7ac44b60ef {
    pub tuple__pcnt_struct_zPhysMemRetStatus__pcnt_bv40: Structc549f9bcfd9a2c5f,
    pub tuple__pcnt_struct_zPhysMemRetStatus__pcnt_bv41: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct51db15130d1af1bf {
    pub aie: bool,
    pub amec: bool,
    pub cmow: bool,
    pub d128: bool,
    pub dc: bool,
    pub dct: bool,
    pub disch: bool,
    pub ds: bool,
    pub e0pd: bool,
    pub ee: bool,
    pub emec: bool,
    pub epan: bool,
    pub ha: bool,
    pub haft: bool,
    pub hd: bool,
    pub hpd: bool,
    pub irgn: u8,
    pub mair: Struct917b3b33dbf1754b,
    pub mair2: Struct917b3b33dbf1754b,
    pub mtx: bool,
    pub nfd: bool,
    pub ntlsmd: bool,
    pub nv1: bool,
    pub orgn: u8,
    pub pie: bool,
    pub pir: Struct917b3b33dbf1754b,
    pub pire0: Struct917b3b33dbf1754b,
    pub pnch: bool,
    pub ps: u8,
    pub sh: u8,
    pub sif: bool,
    pub skl: u8,
    pub t0sz: u8,
    pub t1sz: u8,
    pub tbi: bool,
    pub tbid: bool,
    pub tgx: u32,
    pub txsz: u8,
    pub uwxn: bool,
    pub wxn: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct88c833685d899444 {
    pub or: bool,
    pub or_mmu: bool,
    pub or_rcw: bool,
    pub overlay: bool,
    pub ow: bool,
    pub ow_mmu: bool,
    pub ow_rcw: bool,
    pub ox: bool,
    pub r: bool,
    pub r_mmu: bool,
    pub r_rcw: bool,
    pub toplevel0: bool,
    pub toplevel1: bool,
    pub w: bool,
    pub w_mmu: bool,
    pub w_rcw: bool,
    pub x: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structa4873b0cfdc3568d {
    pub rec: Structcfae909247391754,
    pub shareability: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structf1eb568d383e89ea {
    pub bits: u128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct2912c9c54b07053d {
    pub tuple__pcnt_bv64__pcnt_bv640: u64,
    pub tuple__pcnt_bv64__pcnt_bv641: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structace527123892b9bc {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor0: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor1: Struct5f3b6da595f30aca,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct2499516ce5817d1a {
    pub tuple__pcnt_enum_zConstraint__pcnt_bv0: u32,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structc549f9bcfd9a2c5f {
    pub extflag: bool,
    pub merrorstate: u32,
    pub statuscode: u32,
    pub store64bstatus: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct46a8fa297d4d4711 {
    pub tuple__pcnt_bool__pcnt_bv0: bool,
    pub tuple__pcnt_bool__pcnt_bv1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct674abaa01b77071b {
    pub tuple__pcnt_bv64__pcnt_bool0: u64,
    pub tuple__pcnt_bv64__pcnt_bool1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structc8921d9034b768f1 {
    pub bits: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structe2f620c8eb69267c {
    pub A: bool,
    pub ALLINT: bool,
    pub BTYPE: u8,
    pub C: bool,
    pub D: bool,
    pub DIT: bool,
    pub E: bool,
    pub EL: u8,
    pub EXLOCK: bool,
    pub F: bool,
    pub GE: u8,
    pub I: bool,
    pub IL: bool,
    pub IT: u8,
    pub J: bool,
    pub M: u8,
    pub N: bool,
    pub PAN: bool,
    pub PM: bool,
    pub PPEND: bool,
    pub Q: bool,
    pub SM: bool,
    pub SP: bool,
    pub SS: bool,
    pub SSBS: bool,
    pub T: bool,
    pub TCO: bool,
    pub UAO: bool,
    pub V: bool,
    pub Z: bool,
    pub ZA: bool,
    pub nRW: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct80233863dd0943a0 {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_enum_zSDFType0: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_enum_zSDFType1: Struct5f3b6da595f30aca,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_enum_zSDFType2: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structb73c86e9813fbc6f {
    pub tuple__pcnt_bool__pcnt_bv20: bool,
    pub tuple__pcnt_bool__pcnt_bv21: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structce3d4f74f0c035a1 {
    pub a32lsmd: bool,
    pub acctype: u32,
    pub acqpc: bool,
    pub acqsc: bool,
    pub atomicop: bool,
    pub cacheop: u32,
    pub cachetype: u32,
    pub contiguous: bool,
    pub el: u8,
    pub exclusive: bool,
    pub first: bool,
    pub firstfault: bool,
    pub limitedordered: bool,
    pub ls64: bool,
    pub modop: u32,
    pub mops: bool,
    pub mpam: Structcdab40780616cd2b,
    pub nonfault: bool,
    pub nontemporal: bool,
    pub opscope: u32,
    pub pan: bool,
    pub rcw: bool,
    pub rcws: bool,
    pub read: bool,
    pub relsc: bool,
    pub ss: u32,
    pub streamingsve: bool,
    pub tagaccess: bool,
    pub tagchecked: bool,
    pub toplevel: bool,
    pub transactional: bool,
    pub varange: u32,
    pub write: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structa824a2637dac7ab {
    pub tuple__pcnt_bv4__pcnt_bv1280: u8,
    pub tuple__pcnt_bv4__pcnt_bv1281: u128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct1c4617259271fd9 {
    pub tuple__pcnt_bool__pcnt_bool0: bool,
    pub tuple__pcnt_bool__pcnt_bool1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct3bd767e7a6f24086 {
    pub tuple__pcnt_struct_zGPCFRecord__pcnt_struct_zGPTEntry0: Struct70fb44e0b08fca48,
    pub tuple__pcnt_struct_zGPCFRecord__pcnt_struct_zGPTEntry1: Structe298ee60b912892e,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structa8e779f60a8f0b74 {
    pub tuple__pcnt_enum_zConstraint__pcnt_bv30: u32,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv31: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct17d57f6dfbd87bf {
    pub all_asid: bool,
    pub all_vmid: bool,
    pub asid: u16,
    pub is_asid_valid: bool,
    pub is_vmid_valid: bool,
    pub restriction: u32,
    pub security: u32,
    pub target_el: u8,
    pub vmid: u16,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct57c50f15ba5cdb68 {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_bv1280: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_bv1281: u128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct7089f05824709303 {
    pub tuple__pcnt_enum_zConstraint__pcnt_bv1__pcnt_bv40: u32,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv1__pcnt_bv41: bool,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv1__pcnt_bv42: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct21784028ef9bf8b3 {
    pub asid: u16,
    pub cnp: bool,
    pub ia: u64,
    pub includes_gpt_name: bool,
    pub includes_s1_name: bool,
    pub includes_s2_name: bool,
    pub ipaspace: u32,
    pub isd128: bool,
    pub level: i128,
    pub nG: bool,
    pub regime: u32,
    pub ss: u32,
    pub tg: u32,
    pub vmid: u16,
    pub xs: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct489cd779642cf40c {
    pub tuple__pcnt_bv__pcnt_i0: Bits,
    pub tuple__pcnt_bv__pcnt_i1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct35b62305b7d637a0 {
    pub tuple__pcnt_enum_zConstraint__pcnt_bv80: u32,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv81: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structde0ae26d92ecc34a {
    pub NS: bool,
    pub exceptype: u32,
    pub ipaddress: u64,
    pub ipavalid: bool,
    pub paddress: Structd1e4e056bb52a442,
    pub pavalid: bool,
    pub syndrome: u32,
    pub syndrome2: u32,
    pub trappedsyscallinst: bool,
    pub vaddress: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct6f1e02efdaf6c1e {
    pub tuple__pcnt_bool__pcnt_bv2__pcnt_bv64__pcnt_bv640: bool,
    pub tuple__pcnt_bool__pcnt_bv2__pcnt_bv64__pcnt_bv641: u8,
    pub tuple__pcnt_bool__pcnt_bv2__pcnt_bv64__pcnt_bv642: u64,
    pub tuple__pcnt_bool__pcnt_bv2__pcnt_bv64__pcnt_bv643: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct2b035eb5949c4c68 {
    pub domain: u32,
    pub nXS: bool,
    pub types: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct75f369856b46bcfc {
    pub tuple__pcnt_bv4__pcnt_bv0: u8,
    pub tuple__pcnt_bv4__pcnt_bv1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct721486fe44046f3d {
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv10: bool,
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv11: bool,
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv12: bool,
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv13: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structb8453f579cc11fbc {
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv10: bool,
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv11: bool,
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv12: bool,
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv13: bool,
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv14: bool,
    pub tuple__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv1__pcnt_bv15: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct10cacb7143b8cc5b {
    pub tuple__pcnt_bv24__pcnt_bv110: u32,
    pub tuple__pcnt_bv24__pcnt_bv111: u16,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structf9edbf63b74146af {
    pub ap: u8,
    pub ap_table: u8,
    pub ndirty: bool,
    pub po_index: u8,
    pub ppi: u8,
    pub pxn: bool,
    pub pxn_table: bool,
    pub s2ap: u8,
    pub s2dirty: bool,
    pub s2pi: u8,
    pub s2po_index: u8,
    pub s2tag_na: bool,
    pub s2xn: bool,
    pub s2xnx: bool,
    pub upi: u8,
    pub uxn: bool,
    pub uxn_table: bool,
    pub xn: bool,
    pub xn_table: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structad9a367f5ed3ed07 {
    pub device: u32,
    pub inner: Structe89015f692c2dc66,
    pub memtype: u32,
    pub notagaccess: bool,
    pub outer: Structe89015f692c2dc66,
    pub shareability: u32,
    pub tags: u32,
    pub xs: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structcfae909247391754 {
    pub address: u64,
    pub asid: u16,
    pub attr: u32,
    pub d128: bool,
    pub d64: bool,
    pub end_address_name: u64,
    pub from_aarch64: bool,
    pub ipaspace: u32,
    pub level: u32,
    pub op: u32,
    pub regime: u32,
    pub security: u32,
    pub tg: u8,
    pub ttl: u8,
    pub vmid: u16,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct19c7112689f1b0d9 {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_bool0: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_bool1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structcdab40780616cd2b {
    pub mpam_sp: u32,
    pub partid: u16,
    pub pmg: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct6202352d26f81765 {
    pub tlbrecord: Structcc766a8b1e22a4c4,
    pub valid_name: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct70fb44e0b08fca48 {
    pub gpf: u32,
    pub level: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structd84b34d2bb5d184d {
    pub tuple__pcnt_bool__pcnt_bool__pcnt_bool0: bool,
    pub tuple__pcnt_bool__pcnt_bool__pcnt_bool1: bool,
    pub tuple__pcnt_bool__pcnt_bool__pcnt_bool2: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structe5c48b19f3b32b70 {
    pub acctype: u32,
    pub asid: u16,
    pub cacheop: u32,
    pub cachetype: u32,
    pub cpas: u32,
    pub is_asid_valid: bool,
    pub is_vmid_valid: bool,
    pub level: i128,
    pub opscope: u32,
    pub paddress: Structd1e4e056bb52a442,
    pub regval: u64,
    pub security: u32,
    pub setnum: i128,
    pub shareability: u32,
    pub translated: bool,
    pub vaddress: u64,
    pub vmid: u16,
    pub waynum: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct5d60ad3c5091b98c {
    pub tuple__pcnt_string__pcnt_i0: &'static str,
    pub tuple__pcnt_string__pcnt_i1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct1f6d0e33d85bcca3 {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_bv640: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_bv641: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct62e97a2b6f14adb0 {
    pub baseaddress: Structd1e4e056bb52a442,
    pub contiguous: bool,
    pub disch: bool,
    pub domain: u8,
    pub guardedpage: bool,
    pub istable: bool,
    pub level: i128,
    pub memattrs: Structad9a367f5ed3ed07,
    pub nG: bool,
    pub permissions: Structf9edbf63b74146af,
    pub s1assured: bool,
    pub s2assuredonly: bool,
    pub sdftype: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct87ff22286408f417 {
    pub tuple__pcnt_bool__pcnt_i0: bool,
    pub tuple__pcnt_bool__pcnt_i1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct3287f841d2f7b211 {
    pub access_kind: Enumfabfd93ef17fdbf2,
    pub pa: Bits,
    pub size: i128,
    pub tag: bool,
    pub translation: Enum341d9ce549c82439,
    pub va: Enum3c2b85a331c35c26,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structb49b66991bcde391 {
    pub tuple__pcnt_bv32__pcnt_bv10: u32,
    pub tuple__pcnt_bv32__pcnt_bv11: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structa19ed2e381f7e35b {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_bv0: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_bv1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct81685a93a903c682 {
    pub tuple__pcnt_bv64__pcnt_i0: u64,
    pub tuple__pcnt_bv64__pcnt_i1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structdd965607ed85676a {
    pub tuple__pcnt_struct_zPhysMemRetStatus__pcnt_struct_zAddressDescriptor0: Structc549f9bcfd9a2c5f,
    pub tuple__pcnt_struct_zPhysMemRetStatus__pcnt_struct_zAddressDescriptor1: Struct5f3b6da595f30aca,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct6d81c1183b847fb4 {
    pub tuple__pcnt_enum_zGPCF__pcnt_struct_zGPTEntry0: u32,
    pub tuple__pcnt_enum_zGPCF__pcnt_struct_zGPTEntry1: Structe298ee60b912892e,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structb7bc265102d01f5d {
    pub tuple__pcnt_struct_zPhysMemRetStatus__pcnt_bv0: Structc549f9bcfd9a2c5f,
    pub tuple__pcnt_struct_zPhysMemRetStatus__pcnt_bv1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structfd9a5d5252d38083 {
    pub tuple__pcnt_i__pcnt_i__pcnt_i0: i128,
    pub tuple__pcnt_i__pcnt_i__pcnt_i1: i128,
    pub tuple__pcnt_i__pcnt_i__pcnt_i2: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct5a4c5310d3216a85 {
    pub tuple__pcnt_i__pcnt_i0: i128,
    pub tuple__pcnt_i__pcnt_i1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct90f78381011ed4b4 {
    pub tuple__pcnt_enum_z__InstrEnc__pcnt_bv320: u32,
    pub tuple__pcnt_enum_z__InstrEnc__pcnt_bv321: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structbe856ac00ca579d {
    pub tuple__pcnt_bv1__pcnt_bv10: bool,
    pub tuple__pcnt_bv1__pcnt_bv11: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structecc89dca6559e5c1 {
    pub asid: Enum3c2b85a331c35c26,
    pub memattrs: Structad9a367f5ed3ed07,
    pub regime: u32,
    pub s1level: Enum969da2c83668338c,
    pub s1params: Enumff0cbbffd1014693,
    pub s2info: Enumbc4610d1c4afb05,
    pub s2params: Enum43916b7a3a21b96d,
    pub va: u64,
    pub vmid: Enum3c2b85a331c35c26,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct4484c853bd9837b5 {
    pub gpt_entry: Structe298ee60b912892e,
    pub valid_name: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structa9d8c5953cfa2a25 {
    pub tuple__pcnt_enum_zGPCF__pcnt_struct_zGPTTable0: u32,
    pub tuple__pcnt_enum_zGPCF__pcnt_struct_zGPTTable1: Struct5aa3e121b0be91ee,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct1204798632583a90 {
    pub tuple__pcnt_enum_zConstraint__pcnt_bv20: u32,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv21: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structb3d857da81231c91 {
    pub tuple__pcnt_enum_zSRType__pcnt_i0: u32,
    pub tuple__pcnt_enum_zSRType__pcnt_i1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct1c2679588071f943 {
    pub tuple__pcnt_bv16__pcnt_bool0: u16,
    pub tuple__pcnt_bv16__pcnt_bool1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct188a1c3bf231c64b {
    pub tuple__pcnt_bv__pcnt_bv40: Bits,
    pub tuple__pcnt_bv__pcnt_bv41: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct84fe8b3b2be054b8 {
    pub strength: u32,
    pub variety: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct10a4e724e2377a6f {
    pub tuple__pcnt_enum_zConstraint__pcnt_bv40: u32,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv41: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structd7b2f350a23f9906 {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zTTWState0: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zTTWState1: Struct62e97a2b6f14adb0,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structa79c7f841a890648 {
    pub tuple__pcnt_bv__pcnt_bv0: Bits,
    pub tuple__pcnt_bv__pcnt_bv1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structd1583c616319206 {
    pub tuple__pcnt_bv32__pcnt_bv320: u32,
    pub tuple__pcnt_bv32__pcnt_bv321: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct1eec22ff33779e77 {
    pub tuple__pcnt_i__pcnt_i__pcnt_i640: i128,
    pub tuple__pcnt_i__pcnt_i__pcnt_i641: i128,
    pub tuple__pcnt_i__pcnt_i__pcnt_i642: i64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structa6a13401b33f65e4 {
    pub tuple__pcnt_bv4__pcnt_bv640: u8,
    pub tuple__pcnt_bv4__pcnt_bv641: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct743d0656c8e2b3b3 {
    pub tuple__pcnt_i__pcnt_bv10: i128,
    pub tuple__pcnt_i__pcnt_bv11: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structd26fb3afa197aaf4 {
    pub tuple__pcnt_i__pcnt_bv320: i128,
    pub tuple__pcnt_i__pcnt_bv321: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structe852b87ae24edd79 {
    pub tuple__pcnt_i__pcnt_bv160: i128,
    pub tuple__pcnt_i__pcnt_bv161: u16,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structdac334e66d8ace61 {
    pub tuple__pcnt_bv32__pcnt_bool0: u32,
    pub tuple__pcnt_bv32__pcnt_bool1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structa1f3c754524ed819 {
    pub tuple__pcnt_enum_zSecurityState__pcnt_bv20: u32,
    pub tuple__pcnt_enum_zSecurityState__pcnt_bv21: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct260b95b938846c29 {
    pub tuple__pcnt_bv1__pcnt_bv140: bool,
    pub tuple__pcnt_bv1__pcnt_bv141: u16,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structb087a10901f763f8 {
    pub tuple__pcnt_bv32__pcnt_bv40: u32,
    pub tuple__pcnt_bv32__pcnt_bv41: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct709429a982f3daf1 {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv1280: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv1281: Struct5f3b6da595f30aca,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv1282: Struct62e97a2b6f14adb0,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv1283: u128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structe298ee60b912892e {
    pub contig_size: i128,
    pub gpi: u8,
    pub level: i128,
    pub pa: u64,
    pub size: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct74cbd850c10c35cd {
    pub tuple__pcnt_bv25__pcnt_bv240: u32,
    pub tuple__pcnt_bv25__pcnt_bv241: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structfef2fde4e5b411a3 {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_bv320: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_bv321: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structfeee1f220d723f51 {
    pub tuple__pcnt_bv64__pcnt_bv40: u64,
    pub tuple__pcnt_bv64__pcnt_bv41: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structe04bc78b84625153 {
    pub tuple__pcnt_enum_zConstraint__pcnt_bv60: u32,
    pub tuple__pcnt_enum_zConstraint__pcnt_bv61: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct5aa3e121b0be91ee {
    pub address: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct3961b7c2b54bd49e {
    pub tuple__pcnt_struct_zPhysMemRetStatus__pcnt_bv640: Structc549f9bcfd9a2c5f,
    pub tuple__pcnt_struct_zPhysMemRetStatus__pcnt_bv641: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structac221b9646824496 {
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv640: Struct29530d10a7ab293e,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv641: Struct5f3b6da595f30aca,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv642: Struct62e97a2b6f14adb0,
    pub tuple__pcnt_struct_zFaultRecord__pcnt_struct_zAddressDescriptor__pcnt_struct_zTTWState__pcnt_bv643: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structa32f3d4dde1ac19f {
    pub assuredonly: bool,
    pub cmow: bool,
    pub d128: bool,
    pub ds: bool,
    pub ee: bool,
    pub emec: bool,
    pub fwb: bool,
    pub ha: bool,
    pub haft: bool,
    pub hd: bool,
    pub irgn: u8,
    pub nsa: bool,
    pub nsw: bool,
    pub orgn: u8,
    pub ps: u8,
    pub ptw: bool,
    pub s: bool,
    pub s2pie: bool,
    pub s2pir: Struct917b3b33dbf1754b,
    pub sa: bool,
    pub sh: u8,
    pub skl: u8,
    pub sl0: u8,
    pub sl2: bool,
    pub sw: bool,
    pub t0sz: u8,
    pub tgx: u32,
    pub tl0: bool,
    pub tl1: bool,
    pub txsz: u8,
    pub vm: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structd1e4e056bb52a442 {
    pub address: u64,
    pub paspace: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Structe3eca5d92e533b43 {
    pub A: bool,
    pub D: bool,
    pub F: bool,
    pub FFR: u64,
    pub FPCR: u64,
    pub FPSR: u64,
    pub GCSPR_ELx: u64,
    pub I: bool,
    pub ICC_PMR_EL1: u64,
    pub P: [u64; 16usize],
    pub Rt: i128,
    pub SP: u64,
    pub X: [u64; 31usize],
    pub Z: [u64; 32usize],
    pub depth: i128,
    pub nPC: u64,
    pub nzcv: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct5b79d102506bcdb3 {
    pub gcs: bool,
    pub or: bool,
    pub overlay: bool,
    pub ow: bool,
    pub ox: bool,
    pub r: bool,
    pub w: bool,
    pub wxn: bool,
    pub x: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Struct948aff668ade9f1e {
    pub tuple__pcnt_enum_zSRType__pcnt_i640: u32,
    pub tuple__pcnt_enum_zSRType__pcnt_i641: i64,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enumff0cbbffd1014693 {
    None_RS1TTWParams_(()),
    Some_RS1TTWParams_(Struct51db15130d1af1bf),
}
impl Default for Enumff0cbbffd1014693 {
    fn default() -> Self {
        Self::None_RS1TTWParams_(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enumf433495b989ea3fe {
    None_RFaultRecord_(()),
    Some_RFaultRecord_(Struct29530d10a7ab293e),
}
impl Default for Enumf433495b989ea3fe {
    fn default() -> Self {
        Self::None_RFaultRecord_(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enumc101867477176eb7 {
    Err_Uoption_o____EFault_pcnt__(u32),
    Ok_Uoption_o____EFault_pcnt__(Enumf69731e192b14a6b),
}
impl Default for Enumc101867477176eb7 {
    fn default() -> Self {
        Self::Err_Uoption_o____EFault_pcnt__(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enum969da2c83668338c {
    None_i_(()),
    Some_i_(i128),
}
impl Default for Enum969da2c83668338c {
    fn default() -> Self {
        Self::None_i_(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enum341d9ce549c82439 {
    None_RTranslationInfo_(()),
    Some_RTranslationInfo_(Structecc89dca6559e5c1),
}
impl Default for Enum341d9ce549c82439 {
    fn default() -> Self {
        Self::None_RTranslationInfo_(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enum34719cbce221d72f {
    None_EInterruptID_pcnt__(()),
    Some_EInterruptID_pcnt__(u32),
}
impl Default for Enum34719cbce221d72f {
    fn default() -> Self {
        Self::None_EInterruptID_pcnt__(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enume8fa6f7e2c2caabf {
    Barrier_DMB(Struct2b035eb5949c4c68),
    Barrier_DSB(Struct2b035eb5949c4c68),
    Barrier_ISB(()),
    Barrier_PSSBB(()),
    Barrier_SB(()),
    Barrier_SSBB(()),
}
impl Default for Enume8fa6f7e2c2caabf {
    fn default() -> Self {
        Self::Barrier_DMB(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enum41fb7833ab6da036 {
    Error_ConstrainedUnpredictable(()),
    Error_ExceptionTaken(()),
    Error_ImplementationDefined(&'static str),
    Error_ReservedEncoding(()),
    Error_SError(bool),
    Error_See(&'static str),
    Error_Undefined(()),
    Error_Unpredictable(()),
}
impl Default for Enum41fb7833ab6da036 {
    fn default() -> Self {
        Self::Error_ConstrainedUnpredictable(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enum43916b7a3a21b96d {
    None_RS2TTWParams_(()),
    Some_RS2TTWParams_(Structa32f3d4dde1ac19f),
}
impl Default for Enum43916b7a3a21b96d {
    fn default() -> Self {
        Self::None_RS2TTWParams_(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enumf69731e192b14a6b {
    None_o_(()),
    Some_o_(bool),
}
impl Default for Enumf69731e192b14a6b {
    fn default() -> Self {
        Self::None_o_(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enum3c2b85a331c35c26 {
    None_b_(()),
    Some_b_(Bits),
}
impl Default for Enum3c2b85a331c35c26 {
    fn default() -> Self {
        Self::None_b_(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enumad583a36a667a94a {
    SAcc_ASIMD(bool),
    SAcc_AT(()),
    SAcc_DC(()),
    SAcc_DCZero(()),
    SAcc_GCS(()),
    SAcc_GPTW(()),
    SAcc_IC(()),
    SAcc_NV2(()),
    SAcc_SME(bool),
    SAcc_SPE(()),
    SAcc_SVE(bool),
}
impl Default for Enumad583a36a667a94a {
    fn default() -> Self {
        Self::SAcc_ASIMD(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enumbc4610d1c4afb05 {
    None__b_i__(()),
    Some__b_i__(Struct489cd779642cf40c),
}
impl Default for Enumbc4610d1c4afb05 {
    fn default() -> Self {
        Self::None__b_i__(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enumaae6f3004d2cf576 {
    Err__b_Uoption_o_____EFault_pcnt__(u32),
    Ok__b_Uoption_o_____EFault_pcnt__(Struct343b2d4a0306013a),
}
impl Default for Enumaae6f3004d2cf576 {
    fn default() -> Self {
        Self::Err__b_Uoption_o_____EFault_pcnt__(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Enumfabfd93ef17fdbf2 {
    AK_arch_Uarm_acc_type___(Enumad583a36a667a94a),
    AK_explicit_Uarm_acc_type___(Struct84fe8b3b2be054b8),
    AK_ifetch_Uarm_acc_type___(()),
    AK_ttw_Uarm_acc_type___(()),
}
impl Default for Enumfabfd93ef17fdbf2 {
    fn default() -> Self {
        Self::AK_arch_Uarm_acc_type___(Default::default())
    }
}
// Variable length bitvector implementation
//
// Operations must zero unused bits before returning
#[derive(Clone, Copy, Debug)]
pub struct Bits {
    value: u128,
    length: u16,
}
impl Default for Bits {
    fn default() -> Self {
        Self::new(0, 128)
    }
}
impl Bits {
    pub fn new(value: u128, length: u16) -> Self {
        Self { value, length }.normalize()
    }
    pub fn value(&self) -> u128 {
        self.value
    }
    pub fn length(&self) -> u16 {
        self.length
    }
    fn normalize(self) -> Self {
        let mask = 1u128
            .checked_shl(u32::from(self.length()))
            .map(|i| i - 1)
            .unwrap_or(!0);
        Self {
            value: self.value() & mask,
            length: self.length(),
        }
    }
    pub fn zero_extend(&self, i: i128) -> Self {
        let length = u16::try_from(i).unwrap();
        Self {
            value: self.value(),
            length,
        }
            .normalize()
    }
    pub fn sign_extend(&self, i: i128) -> Self {
        let length = u16::try_from(i).unwrap();
        let shift_amount = 128 - self.length();
        Self {
            value: (((self.value() as i128) << shift_amount) >> shift_amount) as u128,
            length,
        }
            .normalize()
    }
    pub fn truncate(&self, i: i128) -> Self {
        Self {
            value: self.value(),
            length: u16::try_from(i).unwrap(),
        }
            .normalize()
    }
    // Returns the current value with `bits` inserted beginning at index
    // `start`
    pub fn insert(&self, insert: Bits, start: i128) -> Self {
        let shifted = insert.normalize().value() << start;
        if start > 128 {
            panic!();
        }
        if start + i128::from(insert.length()) > 128 {
            panic!();
        }
        let insert_mask = 1u128
            .checked_shl(u32::from(insert.length()))
            .map(|x| x - 1)
            .unwrap_or(!0);
        let mask = !(insert_mask << start);
        let result_value = (self.value() & mask) | shifted;
        let result_length = core::cmp::max(
            self.length(),
            insert.length() + u16::try_from(start).unwrap(),
        );
        Self::new(result_value, result_length)
    }
    pub fn arithmetic_shift_right(&self, amount: i128) -> Self {
        let length = self.length();
        let value = self.value();
        let signed_value = value as i128;
        let sign_extended = (signed_value << (128 - length)) >> (128 - length);
        let shifted = sign_extended >> amount;
        Bits::new(shifted as u128, length)
    }
}
impl core::ops::Shl<i128> for Bits {
    type Output = Self;
    fn shl(self, rhs: i128) -> Self::Output {
        Self {
            value: self.value().checked_shl(u32::try_from(rhs).unwrap()).unwrap_or(0),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::Shr<i128> for Bits {
    type Output = Self;
    fn shr(self, rhs: i128) -> Self::Output {
        Self {
            value: self.value().checked_shr(u32::try_from(rhs).unwrap()).unwrap_or(0),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::Shl for Bits {
    type Output = Self;
    fn shl(self, rhs: Bits) -> Self::Output {
        Self {
            value: self
                .value()
                .checked_shl(u32::try_from(rhs.value()).unwrap())
                .unwrap_or(0),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::BitAnd for Bits {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value() & rhs.value(),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::BitOr for Bits {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value() | rhs.value(),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::BitXor for Bits {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value() ^ rhs.value(),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::Add for Bits {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value().wrapping_add(rhs.value()),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::Sub for Bits {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value().wrapping_sub(rhs.value()),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::Not for Bits {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self {
            value: !self.value(),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::cmp::PartialEq for Bits {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl core::cmp::Eq for Bits {}
pub trait Tracer {
    fn begin(&self, instruction: u32, pc: u64);
    fn end(&self);
    fn read_register(&self, offset: usize, value: &dyn core::fmt::Debug);
    fn write_register(&self, offset: usize, value: &dyn core::fmt::Debug);
    fn read_memory(&self, address: usize, value: &dyn core::fmt::Debug);
    fn write_memory(&self, address: usize, value: &dyn core::fmt::Debug);
}
pub trait RatioExt {
    fn powi(&self, i: i32) -> Self;
    fn sqrt(&self) -> Self;
    fn abs(&self) -> Self;
}
impl RatioExt for num_rational::Ratio<i128> {
    fn powi(&self, i: i32) -> Self {
        self.pow(i)
    }
    fn sqrt(&self) -> Self {
        todo!();
    }
    fn abs(&self) -> Self {
        let n = *self.numer();
        let d = *self.denom();
        Self::new(n.abs(), d)
    }
}
#[derive(Debug)]
pub enum ExecuteResult {
    Ok,
    EndOfBlock,
    UndefinedInstruction,
}
