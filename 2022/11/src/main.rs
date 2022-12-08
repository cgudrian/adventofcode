use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Operation {
    Times(Item),
    Add(Item),
    Square,
}

#[derive(Clone, Copy)]
enum WorryManagement {
    DivideByThree,
    Modulo(Item),
}

type Item = u128;
type Items = VecDeque<Item>;
type Monkeys = Vec<RefCell<Monkey>>;

struct Monkey {
    items: Items,
    operation: Operation,
    divisor: Item,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: usize,
}

impl Monkey {
    fn play_turn(&mut self, others: &Monkeys, worry_management: WorryManagement) {
        while let Some(old) = self.items.pop_front() {
            self.inspection_count += 1;
            let new = match self.operation {
                Operation::Times(num) => old.wrapping_mul(num),
                Operation::Add(num) => old.wrapping_add(num),
                Operation::Square => old.wrapping_mul(old),
            };

            let new = match worry_management {
                WorryManagement::DivideByThree => new / 3,
                WorryManagement::Modulo(num) => new % num,
            };

            let receiver = if new % self.divisor == 0 {
                self.true_monkey
            } else {
                self.false_monkey
            };

            others[receiver].borrow_mut().catches(new);
        }
    }

    fn catches(&mut self, item: Item) {
        self.items.push_back(item);
    }
}

fn input_monkeys() -> Monkeys {
    vec![
        RefCell::new(Monkey {
            items: [50, 70, 89, 75, 66, 66].into(),
            operation: Operation::Times(5),
            divisor: 2,
            true_monkey: 2,
            false_monkey: 1,
            inspection_count: 0,
        }),
        RefCell::new(Monkey {
            items: [85].into(),
            operation: Operation::Square,
            divisor: 7,
            true_monkey: 3,
            false_monkey: 6,
            inspection_count: 0,
        }),
        RefCell::new(Monkey {
            items: [66, 51, 71, 76, 58, 55, 58, 60].into(),
            operation: Operation::Add(1),
            divisor: 13,
            true_monkey: 1,
            false_monkey: 3,
            inspection_count: 0,
        }),
        RefCell::new(Monkey {
            items: [79, 52, 55, 51].into(),
            operation: Operation::Add(6),
            divisor: 3,
            true_monkey: 6,
            false_monkey: 4,
            inspection_count: 0,
        }),
        RefCell::new(Monkey {
            items: [69, 92].into(),
            operation: Operation::Times(17),
            divisor: 19,
            true_monkey: 7,
            false_monkey: 5,
            inspection_count: 0,
        }),
        RefCell::new(Monkey {
            items: [71, 76, 73, 98, 67, 79, 99].into(),
            operation: Operation::Add(8),
            divisor: 5,
            true_monkey: 0,
            false_monkey: 2,
            inspection_count: 0,
        }),
        RefCell::new(Monkey {
            items: [82, 76, 69, 69, 57].into(),
            operation: Operation::Add(7),
            divisor: 11,
            true_monkey: 7,
            false_monkey: 4,
            inspection_count: 0,
        }),
        RefCell::new(Monkey {
            items: [65, 79, 86].into(),
            operation: Operation::Add(5),
            divisor: 17,
            true_monkey: 5,
            false_monkey: 0,
            inspection_count: 0,
        }),
    ]
}

fn play_round(monkeys: &Monkeys, worry_management: WorryManagement) {
    for m in monkeys {
        m.borrow_mut().play_turn(monkeys, worry_management);
    }
}

fn play(monkeys: &Monkeys, num_rounds: usize, worry_management: WorryManagement) -> usize {
    for _ in 1..=num_rounds {
        play_round(&monkeys, worry_management);
    }

    let mut counts: Vec<usize> = monkeys
        .iter()
        .map(|e| e.borrow().inspection_count)
        .collect();
    counts.sort();

    counts[counts.len() - 1] * counts[counts.len() - 2]
}

fn main() {
    let monkeys = input_monkeys();
    let level = play(&monkeys, 20, WorryManagement::DivideByThree);
    println!("Solution 1: {level}");

    let monkeys = input_monkeys();
    let modulo = monkeys.iter().fold(1, |m, e| m * e.borrow().divisor);
    let level = play(&monkeys, 10000, WorryManagement::Modulo(modulo));
    println!("Solution 2: {level}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_monkeys() -> Monkeys {
        vec![
            RefCell::new(Monkey {
                items: [79, 98].into(),
                operation: Operation::Times(19),
                divisor: 23,
                true_monkey: 2,
                false_monkey: 3,
                inspection_count: 0,
            }),
            RefCell::new(Monkey {
                items: [54, 65, 75, 74].into(),
                operation: Operation::Add(6),
                divisor: 19,
                true_monkey: 2,
                false_monkey: 0,
                inspection_count: 0,
            }),
            RefCell::new(Monkey {
                items: [79, 60, 97].into(),
                operation: Operation::Square,
                divisor: 13,
                true_monkey: 1,
                false_monkey: 3,
                inspection_count: 0,
            }),
            RefCell::new(Monkey {
                items: [74].into(),
                operation: Operation::Add(3),
                divisor: 17,
                true_monkey: 0,
                false_monkey: 1,
                inspection_count: 0,
            }),
        ]
    }

    #[test]
    fn test_example1() {
        let monkeys = example_monkeys();
        let level = play(&monkeys, 20, WorryManagement::DivideByThree);
        assert_eq!(level, 105 * 101);
    }

    #[test]
    fn test_example2() {
        let monkeys = example_monkeys();
        let modulo = monkeys.iter().fold(1, |m, e| m * e.borrow().divisor);
        let level = play(&monkeys, 10000, WorryManagement::Modulo(modulo));
        assert_eq!(level, 52013 * 52166);
    }
}
