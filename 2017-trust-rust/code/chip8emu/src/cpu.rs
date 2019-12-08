/// cpu.rs

/// References: 
/// (1) <http://devernay.free.fr/hacks/chip8/C8TECH10.HTM>
/// (2) <https://en.wikipedia.org/wiki/CHIP-8>

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use rand;

use screen;

/// CHIP-8 Memory is 4K bytes in size
const MEM_SIZE: usize = 4096;

/// There are 16 general purpose registers in the CHIP-8,
/// named V0 to VF. VF is used as a flag register in some
/// instructions and it is better to avoid using it for 
/// other purposes.
const NUM_REGS: usize = 16; 

/// The stack has 24 entries, each one 2 bytes long, for a 
/// total size of 48 bytes.
/// 
/// It occupies locations from 0xea0 to 0xeff, both inclusive.
/// 
/// A push operation first increments the stack pointer by 2
/// and then stores a value at the location that is now pointed
/// to by the stack pointer. The first push operation (after reset)
/// should store the value at 0xea0.
const SP_BOTTOM: usize = 0xe9e;

/// Machine code is stored in memory starting at location
/// 0x200.
pub const PC_START: usize = 0x200;

/// The Instruction Pointer type 
type InsnPtr = fn(&mut CPU) -> ();

pub struct CPU {
    /// 4K Memory. 2 byte objects are stored in big-endian
    /// format.
    mem: [u8; MEM_SIZE],
    
    /// The 16 general purpose registers, 8 bits wide.
    v: [u8; NUM_REGS],

    /// The address register.
    i: usize,

    /// The Program Counter, not directly accessible
    /// from CHIP-8 programs.
    pc: usize,

    /// The Stack Pointer, not directly accessible 
    /// from CHIP-8 programs.
    sp: usize,

    /// The CPU has a screen attached to it. During
    /// unit tests, the value of this field will be None.
    screen: Option<screen::Screen>,

    /// The delay timer.
    delay: u8,

    /// The sound timer.
    sound: u8,

} 

impl CPU {
    pub fn new(screen: Option<screen::Screen>) -> Self {
        CPU { 
            mem: [0; MEM_SIZE],
            v: [0; NUM_REGS],
            i: 0,
            pc: PC_START,
            sp: SP_BOTTOM,
            screen: screen,
            delay: 0,
            sound: 0,
        }
    }

    /// Increment the program counter.
    /// download?logged_out=1&lang=en
    /// Each instruction is 2 bytes long, so 
    /// incrementing by 1 will add 2 to the PC.
    fn inc_pc(&mut self, n: usize) {
        self.pc += 2 * n;
    }
    
    /// Get the 12 bit memory address encoded as part of 
    /// the instruction.
    /// 
    /// If you have an instruction of the form "1nnn", you
    /// need to get "nnn", the 12 bit address.
    fn get_address(&self) -> usize {
        let address_high_nibble = self.mem[self.pc] & 0xf;
        let address_low_byte = self.mem[self.pc + 1];
        ((address_high_nibble as usize) << 8) + address_low_byte as usize    

    }

    /// Get the 8 bit constant encoded as part of the instruction.
    /// 
    /// If you have an instruction say "7xnn", this function 
    /// returns "nn".
    fn get_constant(&self) -> u8 {
        self.mem[self.pc + 1] 
    }

    /// Copy 2 bytes from a usize value to top-of-stack.
    /// Value stored on stack is in big endian format.
    fn copy_16bits_to_tos(&mut self, src: usize) {
        self.mem[self.sp] = ((src >> 8) & 0xffusize) as u8;
        self.mem[self.sp + 1] = (src & 0xffusize) as u8;        
    }

    /// Return the 2 byte value taken from top-of-stack.
    fn get_16bits_from_tos(&self) -> usize {
        ((self.mem[self.sp] as usize) << 8) | (self.mem[self.sp + 1] as usize)
    }

    /// Return the lower nibble of the high byte of the 
    /// instruction pointed to by PC.
    fn nibble_x(&self) -> usize {
        (self.mem[self.pc] & 0xf) as usize
    }

    /// Return the high nibble of the low byte of the
    /// instruction pointed to by PC.
    fn nibble_y(&self) -> usize {
        ((self.mem[self.pc + 1] >> 4) & 0xf) as usize
    }

    /// Execute a jump instruction of the form "1nnn"
    /// where nnn represents a memory address.
    fn jmp(&mut self) {
        self.pc = self.get_address();
    }

    /// Call subroutine.
    /// 
    /// The call instruction is of the form "2nnn".
    /// The instruction first increments
    /// the stack pointer by 2 and copies the address of the next
    /// instruction to the new location on the stack. It then sets
    /// the program counter to "nnn".
    fn call(&mut self) {
        let target_address = self.get_address();
        let next_insn_address = self.pc + 2;
        self.sp += 2;
        self.copy_16bits_to_tos(next_insn_address);
        self.pc = target_address;
    }

    /// Subroutine return. Opcode "0x00ee".
    fn ret(&mut self) {
        self.pc = self.get_16bits_from_tos();
        self.sp -= 2;
    }

    /// Skip next instruction if v[x] == nn.
    /// 
    /// This instruction is of the form "3xnn".
    fn skip_if_vx_eq_nn(&mut self) {
        if self.v[self.nibble_x()] == self.get_constant() {
            self.inc_pc(2); // skip next instruction
            return;
        }
        self.inc_pc(1);
    }

    /// Skip next instruction if v[x] != nn.
    /// 
    /// This instruction is of the form "4xnn".
    fn skip_if_vx_ne_nn(&mut self) {
        if self.v[self.nibble_x()] != self.get_constant() {
            self.inc_pc(2); // skip next instruction
            return;
        }
        self.inc_pc(1);
    }

    /// Skip the next instruction if v[x] == v[y].
    /// 
    /// This instruction is of the form "5xy0".
    fn skip_if_vx_eq_vy(&mut self) {
        if self.v[self.nibble_x()] == self.v[self.nibble_y()] {
            self.inc_pc(2); // skip next instruction
            return;
        }
        self.inc_pc(1);
    }

    /// Set v[x] to nn.
    /// 
    /// This instruction is of the form "6xnn".
    fn set_vx_to_nn(&mut self) {
        self.v[self.nibble_x()] = self.get_constant();
        self.inc_pc(1);
    }

    /// Add nn to v[x] without changing carry.
    /// 
    /// This instruction is of the form "7xnn"
    fn add_nn_to_vx(&mut self) {
        self.v[self.nibble_x()] = self.v[self.nibble_x()]
                                      .wrapping_add(self.get_constant());
        self.inc_pc(1);
    }

    /// Assign v[y] to v[x]
    /// 
    /// This instruction is of the form "8xy0"
    fn assign_vy_to_vx(&mut self) {
        self.v[self.nibble_x()] = self.v[self.nibble_y()];
        self.inc_pc(1);
    }

    /// v[x] = v[x] | v[y]
    /// 
    /// This instruction is of the form "8xy1"
    fn assign_vx_or_vy_to_vx(&mut self) {
        self.v[self.nibble_x()] = self.v[self.nibble_x()] | self.v[self.nibble_y()];
        self.inc_pc(1);
    }

    /// v[x] = v[x] & v[y]
    /// 
    /// This instruction is of the form "8xy2"
    fn assign_vx_and_vy_to_vx(&mut self) {
        self.v[self.nibble_x()] = self.v[self.nibble_x()] & self.v[self.nibble_y()];
        self.inc_pc(1);
    }

    /// v[x] = v[x] ^ v[y]
    /// 
    /// This instruction is of the form "8xy3"
    fn assign_vx_xor_vy_to_vx(&mut self) {
        self.v[self.nibble_x()] = self.v[self.nibble_x()] ^ self.v[self.nibble_y()];
        self.inc_pc(1);
    }

    /// v[x] = v[x] + v[y]. v[f] set to 1 if there is a carry,
    /// otherwise set to 0.
    /// 
    /// This instruction is of the form "8xy4"
    fn assign_vx_plus_vy_to_vx(&mut self) {
        let r = u16::from(self.v[self.nibble_x()]) + u16::from(self.v[self.nibble_y()]);
        self.v[0xf] = 0;
        if r > 255 {
            self.v[self.nibble_x()] = (r - 256u16) as u8;
            self.v[0xf] = 1; // carry flag set to 1
        } else {
            self.v[self.nibble_x()] = r as u8;
        }
        self.inc_pc(1);
    }

    /// v[x] = v[x] - v[y].
    /// v[f] is set to 1 if there is NO borrow. Set to 0 otherwise.
    /// 
    /// This instruction has the form "8xy5".
    fn assign_vx_minus_vy_to_vx(&mut self) {
        let (vx, vy) = (self.v[self.nibble_x()], self.v[self.nibble_y()]);
        if vx >= vy { // No borrow
            self.v[self.nibble_x()] = vx - vy;
            self.v[0xf] = 1;
        } else {
            self.v[self.nibble_x()] = vx.wrapping_sub(vy);
            self.v[0xf] = 0;
        }
        self.inc_pc(1);
    }

    /// v[x] = v[x] >> 1.
    /// Before shifting, the least significant bit of v[x]
    /// is copied to v[f].
    /// 
    /// This instruction is of the form: "8x06"
    /// 
    /// Note: There is some difference between this implementation
    /// and the instruction described in the Wikipedia page. This
    /// implementation follows the Python version available here:
    /// <https://github.com/craigthomas/Chip8Python/blob/master/chip8/cpu.py>
    fn shr_vx(&mut self) {
        let vx = self.v[self.nibble_x()];
        self.v[0xf] = vx & 1;
        self.v[self.nibble_x()] = vx >> 1;
        self.inc_pc(1);
    }
    
    /// v[x] = v[y] - v[x]. Set v[f] to 1 if there is NO borrow,
    /// otherwise set to 0.
    /// 
    /// This instruction is of the form: "8xy7"
    fn assign_vy_minus_vx_to_vx(&mut self) {
        let (vx, vy) = (self.v[self.nibble_x()], self.v[self.nibble_y()]);
        if vy >= vx { // No borrow 
            self.v[self.nibble_x()] = vy - vx;
            self.v[0xf] = 1;
        } else {
            self.v[self.nibble_x()] = vy.wrapping_sub(vx);
            self.v[0xf] = 0;
        }
        self.inc_pc(1);
    }

    /// v[x] = v[x] << 1.
    /// Before shifting, the most significant bit of v[x] is
    /// copied to v[f].
    /// 
    /// This instruction has the form: "8x0e".
    /// 
    /// Note: Similar to the "shift right" instruction, this
    /// instruction too is implemented differently from what
    /// is given in the Wikipedia page. This implementation is
    /// based on the Python project whose URL is given in the
    /// comment to the "shr_vx" function.
    fn shl_vx(&mut self) {
        let vx = self.v[self.nibble_x()];
        self.v[0xf] = (vx >> 7) & 1; 
        self.v[self.nibble_x()] = vx << 1;
        self.inc_pc(1);
    }

    /// Skip the next instruction if v[x] not equal to v[y].
    /// 
    /// This instruction has the form: "9xy0".
    fn skip_if_vx_ne_vy(&mut self) {
        if self.v[self.nibble_x()] != self.v[self.nibble_y()] {
            self.inc_pc(2);
        } else {
            self.inc_pc(1);
        }
    }

    /// Assign the 12 bit address encoded as part of the
    /// instruction to the i register.
    /// 
    /// This instruction has the form "annn".
    fn assign_address_to_ireg(&mut self) {
        self.i = self.get_address();
        self.inc_pc(1);
    }

    /// Get the 12 bit address encoded as part of the 
    /// instruction, add v[0] to it and jump to that
    /// location.
    /// 
    /// This instruction has the form: "bnnn".
    fn jmp_to_address_plus_v0(&mut self) {
        self.pc = usize::from(self.v[0]) + self.get_address();
    }

    /// v[x] = rand() & nn
    /// Assign to v[x] the result obtained by doing a
    /// bitwise and of the constant encoded in the 
    /// instruction with a 1 byte random number.
    /// 
    /// This instruction has the form: "cxnn".
    fn assign_rand_bitand_const_to_vx(&mut self) {
        let randval = if cfg!(test) { 0xff } else { rand::random::<u8>() };
        self.v[self.nibble_x()] = self.get_constant() & randval;
        self.inc_pc(1);
    }

    /// i += v[x]
    /// Assign to i the sum of v[x] and i
    /// 
    /// This instruction has the form: "fx1e"
    fn assign_i_plus_vx_to_i(&mut self) {
        self.i += usize::from(self.v[self.nibble_x()]);
        self.inc_pc(1);
    }

    /// Store the three digits obtained by converting v[x]
    /// to BCD at three consecutive locations starting from
    /// the one whose address is stored in the "i" register.
    /// The most significant BCD digit is stored at the lowest
    /// memory address.
    /// 
    /// This instruction has the form: "0xfx33".
    fn store_bcd_of_vx_to_mem(&mut self) {
        let vx = self.v[self.nibble_x()];
        let (a, b, c) = (vx / 100, (vx / 10) % 10, vx % 10);
        self.mem[self.i] = a; // most significant digit at lowest address
        self.mem[self.i + 1] = b;
        self.mem[self.i + 2] = c;
        self.inc_pc(1);
    }

    /// Store content of v[0] to v[x] (including v[x])
    /// to memory locations starting from the one whose
    /// address is stored in the "i" register.
    /// 
    /// This instruction has the form: "0xfx55".
    fn store_v0_to_vx_to_mem(&mut self) {
        for n in 0..self.nibble_x() + 1 {
            self.mem[self.i + n] = self.v[n];
        }
        self.inc_pc(1);
    }

    /// Copy the contents of memory locations starting from
    /// the one whose address is stored in the "i" register
    /// to registers v[0], v[1], ..., v[x].
    /// 
    /// This instruction has the form: 0xfx65.
    fn fill_v0_to_vx_from_mem(&mut self) {
        for n in 0..self.nibble_x() + 1 {
            self.v[n] = self.mem[self.i + n];
        }
        self.inc_pc(1);
    }
    

    /// Draw a sprite on the screen at location (v[x], v[y]). 
    /// 
    /// This instruction has the form: "dxyn". A sprite has a
    /// width of 8 pixels and height (rows) of "n" pixels. Each pixel
    /// is either ON or OFF, so, it can be encoded as either 1 or
    /// 0, a single bit. A byte can thus hold one row of a sprite,
    /// the leftmost (most significant) bit representing the value
    /// of the first pixel on the row, and so on. "n" such bytes
    /// will represent a single sprite. These bytes are stored 
    /// starting at the memory location whose address is stored in the 
    /// "i" register.
    /// 
    /// v[f] is set to 1 if any screen pixels are flipped from set to 
    /// unset, it is otherwise set to 0.
    /// 
    /// A pixel is drawn by Xoring it to the value already present on
    /// the screen at that location.
    /// 
    /// References:
    /// (1) <http://www.emulator101.com/chip-8-sprites.html>
    /// (2) <http://tibasicdev.wikidot.com/68k:sprites> (Explains the Xor logic)
    fn draw_sprite(&mut self) {
        let mut flipped = false;
        let mut r:bool;
        let (x, y) = (self.v[self.nibble_x()], self.v[self.nibble_y()]);
        let n = usize::from(self.mem[self.pc + 1] & 0xf);

        self.v[0xf] = 0;
        if let Some(ref mut scr) = self.screen {
            for y_index in 0usize .. n {
                let val = self.mem[self.i + y_index];
                r = CPU::draw_sprite_row(
                        scr,    
                        val, u32::from(x), 
                        (u32::from(y) + y_index as u32) % u32::from(screen::SCREEN_HEIGHT));
                if r { flipped = true; }
            }
            scr.canvas.present();
            if flipped { 
                self.v[0xf] = 1;
            }
        } else {
            panic!("Error: screen not attached!")
        }
        self.inc_pc(1);
    }

    /// Draw a row of the sprite at position x, y.
    /// 
    /// "val" represents the byte to be drawn.
    /// Each bit of "val", starting from the leftmost
    /// one, will be plotted at (x, y), (x+1, y), (x+2, y)
    /// etc. If "x" exceeds the screen width, it will wrap
    /// to 0.
    /// 
    /// Pixel plotting is done by Xoring the current pixel
    /// color with the sprite color.
    fn draw_sprite_row(scr: &mut screen::Screen, val: u8, x: u32, y: u32) -> bool {
        let mut flipped = false;
        for i in 0..8 {
            let _x = (x + i) % u32::from(screen::SCREEN_WIDTH);
            let current_color = scr.get_pixel(_x, y);
            let sprite_color = (val >> (7 - i)) & 1;
            let new_color = current_color ^ sprite_color;
            if (current_color == 1) && (new_color == 0) {
                flipped = true;
            } 
            scr.draw_pixel(_x, y, screen::PIXEL_COLORS[new_color as usize]);
        }
        flipped
    }

    /// Get key press. Pressed key stored in v[x]. Operation
    /// is blocking.
    /// 
    /// This instruction has the form "fx0a".
    fn get_key(&mut self) {
        let x = self.nibble_x();
        if let Some(ref mut scr) = self.screen {
            self.v[x] = scr.read_key_blocking().expect("error: invalid key pressed!");
        } else {
            panic!("get_key: screen not attached!")
        }
        self.inc_pc(1);
    }

    /// Skip the next instruction if the read key
    /// is equal to the one whose code is stored in
    /// v[x].
    /// 
    /// This instruction has the form: "ex9e".
    fn skip_if_key_eq_vx(&mut self) {
        let x = self.nibble_x();
        let mut n = 1;
        if let Some(ref mut scr) = self.screen {
            let key = scr.read_key_noblocking();
            if let Some(k) = key {
                if k == self.v[x] {
                    n = 2; // skip next instruction
                }
            }
        } else {
            panic!("error: screen not attached!");
        }
        self.inc_pc(n);
    }

    /// Skip the next instruction if the read key
    /// is not equal to the one whose code is stored
    /// in v[x].
    /// 
    /// This instruction has the form: "exa1".
    fn skip_if_key_ne_vx(&mut self) {
        let x = self.nibble_x();
        let mut n = 2;
        if let Some(ref mut scr) = self.screen {
            let key = scr.read_key_noblocking();
            if let Some(k) = key {
                if k == self.v[x] {
                    n = 1; // don't skip next instruction
                }
            }
        } else {
            panic!("error: screen not attached!");
        }
        self.inc_pc(n);
    }

    /// Set v[x] to value of delay timer register.
    /// 
    /// This instruction has the form: "fx07".
    fn copy_delay_reg_to_vx(&mut self){
        self.v[self.nibble_x()] = self.delay;
        self.inc_pc(1);
    }

    /// Copy the value in v[x] to the delay timer register.
    /// 
    /// This instruction has the form: "fx15".
    fn copy_vx_to_delay_reg(&mut self){
        self.delay = self.v[self.nibble_x()];
        self.inc_pc(1);
    }

    /// Copy the value in v[x] to the sound timer register.
    /// 
    /// This instruction has the form: "fx18".
    fn copy_vx_to_sound_reg(&mut self){
        self.sound = self.v[self.nibble_x()];
        self.inc_pc(1);
    }

    /// Set the "i" register to address of the sprite
    /// character stored in v[x]. The sprite characters
    /// are from 0 to 0xf. Each character is represented
    /// by 5 bytes in memory.
    /// 
    /// This instruction has the form: "fx29".
    fn set_ireg_to_sprite_address(&mut self){
        self.i = usize::from(self.v[self.nibble_x()] * 5);
        self.inc_pc(1);
    } 

    /// Load program code / font data into memory starting
    /// at the location mem[offset].
    pub fn load_rom(&mut self, filename: &str, offset: usize) {
        let mut f = File::open(filename).
                    expect(&format!("load_rom: failed to load {}", filename));
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).expect("load_rom: failed to read from file");
        for (index, val) in buf.iter().enumerate() {
            self.mem[offset + index] = *val;
        }
    }

    pub fn decrement_counters(&mut self) {
        if self.sound > 0 {
            self.sound -= 1;
        }
        if self.delay > 0 {
            self.delay -= 1;
        }
    }

    /// Execute the instruction pointed to by the PC.
    pub fn execute_insn(&mut self) {
        // Return from subroutine.
        // Instruction: 0x00ee
        if (self.mem[self.pc] == 0x0) && (self.mem[self.pc + 1] == 0xee) {
            self.ret();
            return;
        }
        // Clear the screen.
        // Instruction format: 0x00e0
        if (self.mem[self.pc] == 0x0) && (self.mem[self.pc + 1] == 0xe0) {
            if let Some(ref mut scr) = self.screen {
                scr.canvas.set_draw_color(screen::PIXEL_COLORS[0]);
                scr.canvas.clear();
            } else {
                panic!("Error: screen not attached!");
            }
            self.inc_pc(1);
            return;    
        }
        // Skip next instruction if key whose code is stored in
        // v[x] is pressed.
        if (((self.mem[self.pc] >> 4) & 0xf) == 0xe) && 
            (self.mem[self.pc + 1] == 0x9e) {
                self.skip_if_key_eq_vx();
                return;
        }
        // Skip next instruction if key whose code is stored in 
        // v[x] is not pressed.
        if (((self.mem[self.pc] >> 4) & 0xf) == 0xe) &&
            (self.mem[self.pc + 1] == 0xa1) {
                self.skip_if_key_ne_vx();
                return;
            }

        // Get the leftmost nibble
        let t = (self.mem[self.pc] >> 4) & 0xf;
        if ((t >= 1) && (t <= 7)) || ((t >= 9) && (t <= 0xd)) {
            let insn = INSN_LUT1.get(&t).expect("Bad Instruction");
            insn(self);
            return;            
        }         
        if t == 8 {
            // Get the rightmost nibble
            let t = self.mem[self.pc + 1] & 0xf;
            let insn = INSN_LUT2.get(&t).expect("Bad Instruction");
            insn(self);
            return;
        }
        if t == 0xf {
            let t = self.mem[self.pc + 1];
            let insn = INSN_LUT3.get(&t).expect("Bad Instruction");
            insn(self);
            return;
        }
    }
} 

/// INSN_LUT1 is an instruction lookup table; used for decoding an 
/// instruction based on its leftmost nibble.
lazy_static! {
     static  ref  INSN_LUT1:HashMap<u8, InsnPtr> = hashmap! {
        1 => CPU::jmp as InsnPtr,
        2 => CPU::call as InsnPtr,
        3 => CPU::skip_if_vx_eq_nn as InsnPtr,
        4 => CPU::skip_if_vx_ne_nn as InsnPtr,
        5 => CPU::skip_if_vx_eq_vy as InsnPtr,
        6 => CPU::set_vx_to_nn as InsnPtr,
        7 => CPU::add_nn_to_vx as InsnPtr,
        9 => CPU::skip_if_vx_ne_vy as InsnPtr,
        0xa => CPU::assign_address_to_ireg as InsnPtr,
        0xb => CPU::jmp_to_address_plus_v0 as InsnPtr,
        0xc => CPU::assign_rand_bitand_const_to_vx as InsnPtr,
        0xd => CPU::draw_sprite as InsnPtr,
    };
}

/// There are multiple instructions whose opcodes start with
/// the leftmost nibble equal to 8. These instructions are uniquely
/// identified based on the value of their rightmost nibble. INSN_LUT2
/// is used to perform this identification.
lazy_static! {
    static ref INSN_LUT2: HashMap<u8, InsnPtr> = hashmap! {
        0 => CPU::assign_vy_to_vx as InsnPtr,
        1 => CPU::assign_vx_or_vy_to_vx as InsnPtr,
        2 => CPU::assign_vx_and_vy_to_vx as InsnPtr,
        3 => CPU::assign_vx_xor_vy_to_vx as InsnPtr,
        4 => CPU::assign_vx_plus_vy_to_vx as InsnPtr,
        5 => CPU::assign_vx_minus_vy_to_vx as InsnPtr,
        6 => CPU::shr_vx as InsnPtr,
        7 => CPU::assign_vy_minus_vx_to_vx as InsnPtr,
        0xe => CPU::shl_vx as InsnPtr,
    };
}

/// There are multiple instructions whose opcodes start
/// with the leftmost nibble equal to 0xf. INSN_LUT3 identifies
/// these instructions based on the value of their least 
/// significant byte. 
lazy_static! {
    static ref INSN_LUT3: HashMap<u8, InsnPtr> = hashmap! {
        0x1e => CPU::assign_i_plus_vx_to_i as InsnPtr,
        0x33 => CPU::store_bcd_of_vx_to_mem as InsnPtr,
        0x55 => CPU::store_v0_to_vx_to_mem as InsnPtr,
        0x65 => CPU::fill_v0_to_vx_from_mem as InsnPtr,
        0x0A => CPU::get_key as InsnPtr,
        0x07 => CPU::copy_delay_reg_to_vx as InsnPtr,
        0x15 => CPU::copy_vx_to_delay_reg as InsnPtr,
        0x18 => CPU::copy_vx_to_sound_reg as InsnPtr,
        0x29 => CPU::set_ireg_to_sprite_address as InsnPtr,
    };
}

#[cfg(test)]
#[path="./cpu_test.rs"]
mod cpu_test;
    