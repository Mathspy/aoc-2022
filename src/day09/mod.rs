use anyhow::{Context, Result};

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Add(isize),
}

impl Instruction {
    fn cycles(&self) -> u8 {
        match self {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        }
    }
}

struct Cpu<I> {
    instructions: I,
    current_instruction: Option<Instruction>,
    subcycle: u8,
    register: isize,
}

impl Cpu<std::vec::IntoIter<Instruction>> {
    fn from_input(input: &str) -> Result<Self> {
        let instructions = input
            .lines()
            .enumerate()
            .map(|(line_num, line)| {
                let mut parts = line.split_whitespace();
                match (parts.next(), parts.next()) {
                    (None, _) => Err(anyhow::format_err!(
                        "missing instruction at line {line_num}"
                    )),
                    (Some("noop"), None) => Ok(Instruction::Noop),
                    (Some("addx"), Some(amount)) => {
                        Ok(Instruction::Add(amount.parse().with_context(|| {
                            "invalid amount {amount} for add instruction at line {line_num}"
                        })?))
                    }
                    (Some(instruction), _) => {
                        Err(anyhow::format_err!("unknown instruction {instruction}"))
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter();

        Ok(Cpu {
            instructions,
            current_instruction: None,
            subcycle: 0,
            register: 1,
        })
    }

    fn register(&self) -> isize {
        self.register
    }
}

impl<I> Iterator for Cpu<I>
where
    I: Iterator<Item = Instruction>,
{
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.subcycle += 1;
        let instruction_finished = match &self.current_instruction {
            Some(instruction) if instruction.cycles() <= self.subcycle => {
                match instruction {
                    Instruction::Noop => {}
                    Instruction::Add(amount) => self.register += amount,
                };

                true
            }
            Some(_) => false,
            None => true,
        };
        if instruction_finished {
            self.current_instruction = self.instructions.next();
            self.subcycle = 0;
        }

        match self.current_instruction {
            Some(_) => Some(self.register),
            None => None,
        }
    }
}

pub fn part1() -> Result<String> {
    todo!()
}

pub fn part2() -> Result<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day09::Instruction;

    use super::Cpu;
    use anyhow::Result;

    const LIGHT_WEIGHT_INPUT: &str = "noop
addx 3
addx -5";

    #[test]
    fn can_parse() -> Result<()> {
        let Cpu {
            mut instructions, ..
        } = Cpu::from_input(LIGHT_WEIGHT_INPUT)?;

        assert_eq!(instructions.next(), Some(Instruction::Noop));
        assert_eq!(instructions.next(), Some(Instruction::Add(3)));
        assert_eq!(instructions.next(), Some(Instruction::Add(-5)));

        Ok(())
    }

    #[test]
    fn can_execute() -> Result<()> {
        let mut cpu = Cpu::from_input(LIGHT_WEIGHT_INPUT)?;

        assert_eq!(cpu.next(), Some(1));
        assert_eq!(cpu.next(), Some(1));
        assert_eq!(cpu.next(), Some(1));
        assert_eq!(cpu.next(), Some(4));
        assert_eq!(cpu.next(), Some(4));
        assert_eq!(cpu.next(), None);
        assert_eq!(cpu.register(), -1);

        Ok(())
    }
}
