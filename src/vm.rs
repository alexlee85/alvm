use crate::instruction::*;

#[derive(Debug, Default)]
pub struct VM {
    pub registers: [i32; 32],
    // program counter, track which byte is executing
    pc: usize,
    pub program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = i32::from(number);
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 == register2;
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 != register2;
                self.next_8_bits();
            }
            Opcode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 >= register2;
                self.next_8_bits();
            }
            Opcode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 <= register2;
                self.next_8_bits();
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 < register2;
                self.next_8_bits();
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 > register2;
                self.next_8_bits();
            }
            Opcode::JMPE => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            _ => {
                println!("unrecognized opcode found! Terminating!");
                return true;
            }
        }

        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let first_8_bits = u16::from(self.program[self.pc]);
        let next_8_bits =  u16::from(self.program[self.pc + 1]);
        let result = (first_8_bits << 8) | next_8_bits;
        self.pc += 2;
        result
    }

    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::default();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::default();
        test_vm.program = vec![0, 0, 1, 244];

        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::default();
        let load1_bytes = vec![0, 0, 0, 100];
        let load2_bytes = vec![0, 1, 0, 200];
        let add_bytes = vec![1, 0, 1, 2];
        let test_bytes = vec![load1_bytes, load2_bytes, add_bytes]
            .iter()
            .flatten()
            .cloned()
            .collect();
        test_vm.program = test_bytes;

        test_vm.run();
        assert_eq!(test_vm.registers[0], 100);
        assert_eq!(test_vm.registers[1], 200);
        assert_eq!(test_vm.registers[2], 300);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::default();
        let load1_bytes = vec![0, 0, 0, 200];
        let load2_bytes = vec![0, 1, 0, 100];
        let sub_bytes = vec![2, 0, 1, 2];
        let test_bytes = vec![load1_bytes, load2_bytes, sub_bytes]
            .iter()
            .flatten()
            .cloned()
            .collect();
        test_vm.program = test_bytes;

        test_vm.run();
        assert_eq!(test_vm.registers[0], 200);
        assert_eq!(test_vm.registers[1], 100);
        assert_eq!(test_vm.registers[2], 100);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 20;
        let test_bytes = vec![3, 0, 1, 2];
        test_vm.program = test_bytes;

        test_vm.run();
        assert_eq!(test_vm.registers[2], 200);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 200;
        test_vm.registers[1] = 30;
        let test_bytes = vec![4, 0, 1, 2];
        test_vm.program = test_bytes;

        test_vm.run();
        assert_eq!(test_vm.registers[2], 6);
        assert_eq!(test_vm.remainder, 20);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::default();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;

        test_vm.run();
        assert_eq!(test_vm.pc, 1)
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 2;
        let test_bytes = vec![7, 0, 0, 0, 200, 0, 0, 0];
        test_vm.program = test_bytes;

        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::default();
        test_vm.pc = 4;
        test_vm.registers[0] = 6;
        let test_bytes = vec![200, 0, 0, 0, 8, 0, 0, 0];
        test_vm.program = test_bytes;

        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 4;
        test_vm.registers[1] = 4;
        let test_bytes = vec![9, 0, 1, 0];
        test_vm.program = test_bytes;

        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 4;
        test_vm.registers[1] = 5;
        let test_bytes = vec![10, 0, 1, 0];
        test_vm.program = test_bytes;

        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_gte() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 4;
        let test_bytes = vec![11, 0, 1, 0];
        test_vm.program = test_bytes;

        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_lte() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 3;
        test_vm.registers[1] = 4;
        let test_bytes = vec![12, 0, 1, 0];
        test_vm.program = test_bytes;

        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_lt() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 4;
        test_vm.registers[1] = 4;
        let test_bytes = vec![13, 0, 1, 0];
        test_vm.program = test_bytes;

        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 4;
        test_vm.registers[1] = 4;
        let test_bytes = vec![14, 0, 1, 0];
        test_vm.program = test_bytes;

        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_jmpe() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 4;
        test_vm.equal_flag = true;
        let test_bytes = vec![15, 0, 0, 0, 200, 0, 0, 0];
        test_vm.program = test_bytes;

        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::default();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;

        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
}
