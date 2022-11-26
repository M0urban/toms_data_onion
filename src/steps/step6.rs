use super::super::base85;

pub fn run_step(payload: &[u8]) -> Vec<u8> {
    let encode = base85(payload);
    step6_extension(encode)
}

fn step6_extension(payload: Vec<u8>) -> Vec<u8> {
    let mut cpu = TomtelI69::new(payload.len(), payload);
    let mut ret: Option<Vec<u8>> = None;
    while ret.is_none() {
        ret = cpu.run_cycle();
    }
    ret.unwrap()
}

struct TomtelI69 {
    reg: Registers,
    mem: Vec<u8>,
    out: Vec<u8>,
}
impl TomtelI69 {
    pub fn new(memsize: usize, mem: Vec<u8>) -> Self {
        TomtelI69 {
            reg: Registers::new(),
            mem,
            out: Vec::with_capacity(memsize),
        }
    }
    pub fn run_cycle(&mut self) -> Option<Vec<u8>> {
        let ins = self.fetch_decode();
        if self.execute(ins) {
            Some(self.out.clone())
        } else {
            None
        }
    }
    fn fetch_decode(&mut self) -> Instruction {
        let pc: usize = self.reg.pc.try_into().unwrap();
        let op_code = self.mem[pc];
        use Instruction::*;
        let ins = match op_code {
            0xC2 => Add,
            0xE1 => Aptr(self.mem[pc + 1]),
            0xC1 => Cmp,
            0x01 => Halt,
            0x21 => Jez(u32::from_le_bytes(
                self.mem[pc + 1..pc + 5].try_into().unwrap(),
            )),
            0x22 => Jnz(u32::from_le_bytes(
                self.mem[pc + 1..pc + 5].try_into().unwrap(),
            )),
            0x02 => Out,
            0xC3 => Sub,
            0xC4 => Xor,
            cd if cd & 0b0100_0111 == 0b0100_0000 => {
                Mvi(RegU8::from_opcode(cd, DestSrc::Dest), self.mem[pc + 1])
            }
            cd if cd & 0b0100_0000 == 0b0100_0000 => Mv(
                RegU8::from_opcode(cd, DestSrc::Dest),
                RegU8::from_opcode(cd, DestSrc::Src),
            ),
            cd if cd & 0b1000_0111 == 0b1000_0000 => Mvi32(
                RegU32::from_opcode(cd, DestSrc::Dest),
                u32::from_le_bytes(self.mem[pc + 1..pc + 5].try_into().unwrap()),
            ),
            cd if cd & 0b1000_0000 == 0b1000_0000 => Mv32(
                RegU32::from_opcode(cd, DestSrc::Dest),
                RegU32::from_opcode(cd, DestSrc::Src),
            ),
            _ => panic!("invalid instruction"),
        };
        //println!("abc");
        self.reg.pc += ins.get_instruction_size();
        ins
    }
    fn execute(&mut self, ins: Instruction) -> bool {
        match ins {
            Instruction::Add => self.reg.a = self.reg.a.overflowing_add(self.reg.b).0,
            Instruction::Aptr(imm8) => self.reg.ptr += imm8 as u32,
            Instruction::Cmp => self.reg.f = (self.reg.a != self.reg.b) as u8,
            Instruction::Halt => return true,
            Instruction::Jez(imm32) => {
                if self.reg.f == 0 {
                    self.reg.pc = imm32
                }
            }
            Instruction::Jnz(imm32) => {
                if self.reg.f != 0 {
                    self.reg.pc = imm32
                }
            }
            Instruction::Mv(dest, src) => self.write_u8_re(dest, self.read_u8_re(src)),
            Instruction::Mv32(dest, src) => self.write_u32_re(dest, self.read_u32_re(src)),
            Instruction::Mvi(dest, imm8) => self.write_u8_re(dest, imm8),
            Instruction::Mvi32(dest, imm32) => self.write_u32_re(dest, imm32),
            Instruction::Out => self.out.push(self.reg.a),
            Instruction::Sub => self.reg.a = self.reg.a.overflowing_sub(self.reg.b).0,
            Instruction::Xor => self.reg.a ^= self.reg.b,
        }
        false
    }

    fn read_u8_re(&self, reg: RegU8) -> u8 {
        match reg {
            RegU8::a => self.reg.a,
            RegU8::b => self.reg.b,
            RegU8::c => self.reg.c,
            RegU8::d => self.reg.d,
            RegU8::e => self.reg.e,
            RegU8::f => self.reg.f,
            RegU8::ptr_c => {
                let ind: usize = self.reg.ptr_c().try_into().unwrap();
                self.mem[ind]
            }
        }
    }

    fn write_u8_re(&mut self, reg: RegU8, val: u8) {
        match reg {
            RegU8::a => self.reg.a = val,
            RegU8::b => self.reg.b = val,
            RegU8::c => self.reg.c = val,
            RegU8::d => self.reg.d = val,
            RegU8::e => self.reg.e = val,
            RegU8::f => self.reg.f = val,
            RegU8::ptr_c => {
                let ind: usize = self.reg.ptr_c().try_into().unwrap();
                self.mem[ind] = val;
            }
        }
    }

    fn read_u32_re(&self, reg: RegU32) -> u32 {
        match reg {
            RegU32::la => self.reg.la,
            RegU32::lb => self.reg.lb,
            RegU32::lc => self.reg.lc,
            RegU32::ld => self.reg.ld,
            RegU32::ptr => self.reg.ptr,
            RegU32::pc => self.reg.pc,
        }
    }

    fn write_u32_re(&mut self, reg: RegU32, val: u32) {
        match reg {
            RegU32::la => self.reg.la = val,
            RegU32::lb => self.reg.lb = val,
            RegU32::lc => self.reg.lc = val,
            RegU32::ld => self.reg.ld = val,
            RegU32::ptr => self.reg.ptr = val,
            RegU32::pc => self.reg.pc = val,
        }
    }
}

struct Registers {
    a: u8,    //Accumulator
    b: u8,    //Opperand register
    c: u8,    //Count/offset
    d: u8,    //GP Reg
    e: u8,    //Gp Reg
    f: u8,    //Flag
    la: u32,  //GP reg
    lb: u32,  //GP reg
    lc: u32,  //GP reg
    ld: u32,  //GP reg
    ptr: u32, //pointer to memory
    pc: u32,  //programm counter
}

#[derive(Clone, Copy)]
enum DestSrc {
    Src = 0,
    Dest = 1,
}
enum RegU8 {
    a = 1,
    b = 2,
    c = 3,
    d = 4,
    e = 5,
    f = 6,
    ptr_c = 7,
}
impl RegU8 {
    fn from_opcode(op: u8, dest_src: DestSrc) -> Self {
        let reg = (op & (0b0000_0111 << 3 * (dest_src as u8))) >> (3 * (dest_src as u8));
        match reg {
            1 => Self::a,
            2 => Self::b,
            3 => Self::c,
            4 => Self::d,
            5 => Self::e,
            6 => Self::f,
            7 => Self::ptr_c,
            _ => panic!("Invalid register in opcode"),
        }
    }
}

enum RegU32 {
    la = 1,
    lb = 2,
    lc = 3,
    ld = 4,
    ptr = 5,
    pc = 6,
}
impl RegU32 {
    fn from_opcode(op: u8, dest_src: DestSrc) -> Self {
        let reg = (op & (0b0000_0111 << 3 * (dest_src as u8))) >> (3 * (dest_src as u8));
        match reg {
            1 => Self::la,
            2 => Self::lb,
            3 => Self::lc,
            4 => Self::ld,
            5 => Self::ptr,
            6 => Self::pc,
            _ => panic!("Invalid register in opcode"),
        }
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            la: 0,
            lb: 0,
            lc: 0,
            ld: 0,
            ptr: 0,
            pc: 0,
        }
    }
    pub fn ptr_c(&self) -> u32 {
        self.ptr + self.c as u32
    }
}

enum Instruction {
    Add,
    Aptr(u8),
    Cmp,
    Halt,
    Jez(u32),
    Jnz(u32),
    Mv(RegU8, RegU8),
    Mv32(RegU32, RegU32),
    Mvi(RegU8, u8),
    Mvi32(RegU32, u32),
    Out,
    Sub,
    Xor,
}

impl Instruction {
    fn get_instruction_size(&self) -> u32 {
        match self {
            Self::Add => 1,
            Self::Aptr(_) => 2,
            Self::Cmp => 1,
            Self::Halt => 1,
            Self::Jez(_) => 5,
            Self::Jnz(_) => 5,
            Self::Mv(_, _) => 1,
            Self::Mv32(_, _) => 1,
            Self::Mvi(_, _) => 2,
            Self::Mvi32(_, _) => 5,
            Self::Out => 1,
            Self::Sub => 1,
            Self::Xor => 1,
        }
    }
}
