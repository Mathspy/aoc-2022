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

pub fn part1_inner(input: &str) -> Result<String> {
    const INTERESTING_CYCLES: &[usize] = &[20, 60, 100, 140, 180, 220];

    let cpu = Cpu::from_input(input)?;

    let sum = cpu
        .enumerate()
        .map(|(cycle_0_indexed, register)| (cycle_0_indexed + 1, register))
        .filter(|(cycle, _)| INTERESTING_CYCLES.contains(cycle))
        .map(|(cycle, register)| isize::try_from(cycle).expect("small number") * register)
        .sum::<isize>()
        .to_string();

    Ok(sum)
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

    use super::{part1_inner, Cpu};
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

    const MORE_REALISTIC: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn can_calculate_result() -> Result<()> {
        assert_eq!(part1_inner(MORE_REALISTIC)?, "13140");

        Ok(())
    }
}
