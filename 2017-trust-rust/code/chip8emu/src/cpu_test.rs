
use super::*;

/// Tests all instructions except those doing I/O.

#[test]
fn test_jump(){
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x1055
    // Jump to 0x55
    c.mem[0] = 0x10;
    c.mem[1] = 0x55;   
    c.execute_insn();
    assert_eq!(c.pc, 0x55);
}

#[test]
fn test_call() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x2134
    // Call subroutine at 0x134
    c.mem[0] = 0x21;
    c.mem[1] = 0x34;
    c.execute_insn();
    assert_eq!(c.pc, 0x134);
    assert_eq!(c.sp, SP_BOTTOM + 2);
    assert_eq!(c.mem[c.sp], 0x0);
    assert_eq!(c.mem[c.sp + 1], 0x2);
}

#[test]
fn test_ret() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x2134
    // Call subroutine at 0x134
    c.mem[0] = 0x21;
    c.mem[1] = 0x34;

    // The subroutine has only one instruction: a "ret".
    c.mem[0x134] = 0x00;
    c.mem[0x135] = 0xee;

    c.execute_insn(); // call 0x134   
    c.execute_insn(); // ret

    assert_eq!(c.sp, SP_BOTTOM);
    assert_eq!(c.pc, 0x2);    
}

#[test]
fn test1_skip_if_vx_eq_nn() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x3a24
    // Skip next instruction if self.v[0xa] == 0x24

    c.v[0xa] = 0x24;
    c.mem[0] = 0x3a;
    c.mem[1] = 0x24;

    c.execute_insn();
    assert_eq!(c.pc, 0x4);
}

#[test]
fn test2_skip_if_vx_eq_nn() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x3a24
    // Skip next instruction if self.v[0xa] == 0x24

    c.v[0xa] = 0x8;
    c.mem[0] = 0x3a;
    c.mem[1] = 0x24;

    c.execute_insn();
    assert_eq!(c.pc, 0x2);
}

#[test]
fn test1_skip_if_vx_ne_nn() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x4a24
    // Skip next instruction if self.v[0xa] != 0x24

    c.v[0xa] = 0x8;
    c.mem[0] = 0x4a;
    c.mem[1] = 0x24;

    c.execute_insn();
    assert_eq!(c.pc, 0x4);
}

#[test]
fn test2_skip_if_vx_ne_nn() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x4a24
    // Skip next instruction if self.v[0xa] == 0x24

    c.v[0xa] = 0x24;
    c.mem[0] = 0x4a;
    c.mem[1] = 0x24;

    c.execute_insn();
    assert_eq!(c.pc, 0x2);
}

#[test]
fn test1_skip_if_vx_eq_vy() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x52b0
    // Skip next instruction if self.v[0x2] == self.v[0xb]

    c.v[0x2] = 0xff;
    c.v[0xb] = 0xff;
    c.mem[0] = 0x52;
    c.mem[1] = 0xb0;

    c.execute_insn();
    assert_eq!(c.pc, 0x4);
}

#[test]
fn test2_skip_if_vx_eq_vy() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x52b0
    // Skip next instruction if self.v[0x2] == self.v[0xb]

    c.v[0x2] = 0xff;
    c.v[0xb] = 0xfe;
    c.mem[0] = 0x52;
    c.mem[1] = 0xb0;

    c.execute_insn();
    assert_eq!(c.pc, 0x2);
}

#[test]
fn test_set_vx_to_nn() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x6c2b
    // Set self.v[0xc] to 0x2b

    c.mem[0] = 0x6c;
    c.mem[1] = 0x2b;
    
    c.execute_insn();
    assert_eq!(c.v[0xc], 0x2b);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_add_nn_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x7405
    // Add 0x5 to self.v[0x4] without changing carry.

    let t = c.v[0xf]; 
    c.v[0x4] = 255;
    c.mem[0] = 0x74;
    c.mem[1] = 0x05;

    c.execute_insn();
    assert_eq!(t, c.v[0xf]);
    assert_eq!(c.v[0x4], 4);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_assign_vy_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x82b0
    // v[2] = v[0xb]

    c.v[2] = 10;
    c.v[0xb] = 20;
    c.mem[0] = 0x82;
    c.mem[1] = 0xb0;

    c.execute_insn();
    assert_eq!(c.v[2], c.v[0xb]);    
    assert_eq!(c.v[2], 20);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_assign_vx_or_vy_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x85c1
    // v[5] = v[5] | v[0xc]

    c.v[5] = 0x8;
    c.v[0xc] = 0x1;
    c.mem[0] = 0x85;
    c.mem[1] = 0xc1;

    c.execute_insn();
    assert_eq!(c.v[5], 0x9);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_assign_vx_and_vy_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Insruction: 0x85c2
    // v[5] = v[5] & v[0xc] 

    c.v[5] = 0x8;
    c.v[0xc] = 0x1;
    c.mem[0] = 0x85;
    c.mem[1] = 0xc2;

    c.execute_insn();
    assert_eq!(c.v[5], 0);
    assert_eq!(c.pc, 2);   
}

#[test]
fn test_assign_vx_xor_vy_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x85c3
    // v[5] = v[5] ^ v[0xc]

    c.v[5] = 0x9;
    c.v[0xc] = 0x9;
    c.mem[0] = 0x85;
    c.mem[1] = 0xc3;

    c.execute_insn();
    assert_eq!(c.v[5], 0);
    assert_eq!(c.pc, 2);
}

#[test]
fn test1_assign_vx_plus_vy_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x85c4
    // v[5] = v[5] + v[0xc]

    c.v[5] = 255;
    c.v[0xc] = 10; // Generate overflow
    c.mem[0] = 0x85;
    c.mem[1] = 0xc4;

    c.execute_insn();
    assert_eq!(c.v[5], 9);
    assert_eq!(c.v[0xf], 1);
    assert_eq!(c.pc, 2);
}

#[test]
fn test2_assign_vx_plus_vy_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x85c4
    // v[5] = v[5] + v[0xc]

    c.v[5] = 245;
    c.v[0xc] = 10; // No overflow
    c.mem[0] = 0x85;
    c.mem[1] = 0xc4;

    c.execute_insn();
    assert_eq!(c.v[5], 255);
    assert_eq!(c.v[0xf], 0);
    assert_eq!(c.pc, 2);
}

#[test] 
fn test1_assign_vx_minus_vy_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x89d5
    // v[9] = v[9] - v[0xd]

    c.v[9] = 100;
    c.v[0xd] = 90; // No borrow
    c.mem[0] = 0x89;
    c.mem[1] = 0xd5;

    c.execute_insn();
    assert_eq!(c.v[9], 10); 
    assert_eq!(c.v[0xf], 1); 
    assert_eq!(c.pc, 2);
}

#[test] 
fn test2_assign_vx_minus_vy_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x89d5
    // v[9] = v[9] - v[0xd]

    c.v[9] = 100;
    c.v[0xd] = 110; // Borrow
    c.mem[0] = 0x89;
    c.mem[1] = 0xd5;

    c.execute_insn();
    assert_eq!(c.v[9], 246); 
    assert_eq!(c.v[0xf], 0); 
    assert_eq!(c.pc, 2);
}

#[test]
fn test_shr_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x8706
    // v[7] = v[7] >> 1

    c.v[7] = 3;
    c.mem[0] = 0x87;
    c.mem[1] = 0x06;

    c.execute_insn();
    assert_eq!(c.v[7], 1);
    assert_eq!(c.v[0xf], 1);
    assert_eq!(c.pc, 2);
}

#[test]
fn test1_assign_vy_minus_vx_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x89e7
    // v[9] = v[0xe] - v[0x9]

    c.v[9] = 10;
    c.v[0xe] = 13;
    c.mem[0] = 0x89;
    c.mem[1] = 0xe7;

    c.execute_insn();
    assert_eq!(c.v[9], 3);
    assert_eq!(c.v[0xf], 1);
    assert_eq!(c.pc, 2);

}

#[test]
fn test2_assign_vy_minus_vx_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x89e7
    // v[9] = v[0xe] - v[0x9]

    c.v[9] = 13;
    c.v[0xe] = 10;
    c.mem[0] = 0x89;
    c.mem[1] = 0xe7;

    c.execute_insn();
    assert_eq!(c.v[9], 253);
    assert_eq!(c.v[0xf], 0);
    assert_eq!(c.pc, 2);

}

#[test]
fn test_shl_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x870e
    // v[7] = v[7] << 1

    c.v[7] = 2;
    c.mem[0] = 0x87;
    c.mem[1] = 0x0e;

    c.execute_insn();
    assert_eq!(c.v[7], 4);
    assert_eq!(c.pc, 2);
}

#[test]
fn test1_skip_if_vx_ne_vy() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x9560
    // skip if v[5] != v[6]

    c.v[5] = 1;
    c.v[6] = 1;
    c.mem[0] = 0x95;
    c.mem[1] = 0x60;
    
    c.execute_insn();
    assert_eq!(c.pc, 2);
}

#[test]
fn test2_skip_if_vx_ne_vy() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0x9560
    // skip if v[5] != v[6]

    c.v[5] = 1;
    c.v[6] = 2;
    c.mem[0] = 0x95;
    c.mem[1] = 0x60;
    
    c.execute_insn();
    assert_eq!(c.pc, 4);
}

#[test]
fn test_assign_address_to_ireg() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0xa123
    // i = 0x123

    c.mem[0] = 0xa1;
    c.mem[1] = 0x23;

    c.execute_insn();
    assert_eq!(c.i, 0x123);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_jmp_to_address_plus_v0() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0xb123
    // pc = 0x123 + v[0]

    c.v[0] = 0x3;
    c.mem[0] = 0xb1;
    c.mem[1] = 0x23;

    c.execute_insn();
    assert_eq!(c.pc, 0x126);
}

#[test]
fn test_assign_rand_bitand_const_to_vx() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0xc75a
    // v[7] = rand() & 0x5a
    // When the test code is being executed,
    // value of rand() will be 0xff.

    c.mem[0] = 0xc7;
    c.mem[1] = 0x5a;

    c.execute_insn();
    assert_eq!(c.v[7], 0x5a);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_assign_i_plus_vx_to_i() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0xf31e
    // i += v[3]

    c.i = 4;
    c.v[3] = 10;
    c.mem[0] = 0xf3;
    c.mem[1] = 0x1e;

    c.execute_insn();
    assert_eq!(c.i, 14);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_store_bcd_of_vx_to_mem() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0xf133
    
    c.i = 6;
    c.v[1] = 123;
    c.mem[0] = 0xf1;
    c.mem[1] = 0x33;

    c.execute_insn();
    assert_eq!(c.mem[6], 1);
    assert_eq!(c.mem[7], 2);
    assert_eq!(c.mem[8], 3);
    assert_eq!(c.pc, 2);
}

#[test]
fn test_store_v0_to_vx_to_mem() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0xff55
    // Store v[0] to v[0xf] to mem,
    // starting from c.mem[c.i]

    for i in 0..0x10usize {
        c.v[i] = i as u8;
    }
    c.i = 0;
    c.mem[0] = 0xff;
    c.mem[1] = 0x55;

    c.execute_insn();
    for i in 0..0x10usize {
        assert_eq!(c.mem[i], c.v[i]);
    }
    assert_eq!(c.pc, 2);
}

#[test]
fn test_fill_v0_to_vx_from_mem() {
    let mut c = CPU::new(None);
    c.pc = 0;
    // Instruction: 0xff65
    // Store from c.mem[i], c.mem[i+1], ..., c.mem[i+0xf]
    // to c.v[0], c.v[1], ..., c.v[0xf]

    c.i = 20;
    for n in 0..0x10usize {
        c.mem[c.i + n] = n as u8;
    }
    c.mem[0] = 0xff;
    c.mem[1] = 0x65;

    c.execute_insn();
    for i in 0..0x10usize {
        assert_eq!(c.v[i], i as u8);
    }
    assert_eq!(c.pc, 2);
}