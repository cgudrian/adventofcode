use std::{ops::Index, slice::Iter};

type Register = isize;

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    AddX(Register),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts.as_slice() {
            ["addx", arg] => Instruction::AddX(arg.parse().unwrap()),
            ["noop"] => Instruction::Noop,
            _ => panic!("Invalid Op"),
        }
    }
}

struct Program(Vec<Instruction>);

impl Program {
    #[cfg(test)]
    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> Iter<'_, Instruction> {
        self.0.iter()
    }
}

impl Index<usize> for Program {
    type Output = Instruction;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<&'static str> for Program {
    fn from(src: &'static str) -> Self {
        src.lines().collect()
    }
}

impl FromIterator<&'static str> for Program {
    fn from_iter<T: IntoIterator<Item=&'static str>>(iter: T) -> Self {
        let mut instructions = Vec::new();
        for i in iter {
            instructions.push(i.into());
        }
        Program(instructions)
    }
}

struct CPU;

impl CPU {
    fn new() -> CPU {
        CPU {}
    }

    fn run_program(&self, program: &Program) -> Vec<Register> {
        program
            .iter()
            .fold((vec![1, 1], 1), |(mut res, mut x), instruction| {
                match instruction {
                    Instruction::Noop => res.push(x),

                    Instruction::AddX(arg) => {
                        res.push(x);
                        x += arg;
                        res.push(x);
                    }
                }
                (res, x)
            })
            .0
    }
}

fn run_program(program: &Program) -> Vec<Register> {
    let cpu = CPU::new();
    cpu.run_program(program)
}

fn solve1(program: &Program) -> (Vec<Register>, isize) {
    let res = run_program(&program);
    let sol = (20..=220)
        .step_by(40)
        .map(|idx| res[idx] * idx as isize)
        .sum();
    (res, sol)
}

fn solve2(regs: &Vec<Register>) {
    for row in 0..=5 {
        for col in 0..=39 {
            let idx = 1 + (row * 40 + col);
            let reg = regs[idx];
            if (reg - col as isize).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    let program: Program = INPUT.into();
    let (regs, sol1) = solve1(&program);
    println!("Answer 1: {}", sol1);
    println!("Answer 2:");
    solve2(&regs);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EX1: &str = include_str!("example1.txt");

    #[test]
    fn load_program_from_string() {
        let program: Program = EX1.into();
        assert_eq!(program.len(), 146);
        assert_eq!(program[23], Instruction::AddX(1));
    }

    #[test]
    fn test_example1() {
        let program: Program = EX1.into();
        let res = run_program(&program);
        assert_eq!(res.len(), 242);
        assert_eq!(res[20], 21, "20");
        assert_eq!(res[60], 19, "60");
        assert_eq!(res[100], 18, "100");
        assert_eq!(res[140], 21, "140");
        assert_eq!(res[180], 16, "180");
        assert_eq!(res[220], 18, "220");
    }

    #[test]
    fn test_solve1() {
        let program: Program = EX1.into();
        let (_, sol1) = solve1(&program);
        assert_eq!(sol1, 13140);
    }
}
