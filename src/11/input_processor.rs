use lib::input_parser;

pub type InputType = String;
type WorryType = u64;
type InspectType = u64;

pub fn get_input(file: &str) -> InputType {
    input_parser::read_file_to_string(file!(), file)
}

pub fn new_game(input: &str) -> MonkeyGame {
    let monkeys: Vec<Monkey> = input_parser::group_lines_iter(&input)
        .map(|group| group.parse::<Monkey>().unwrap())
        .collect();
    let size = monkeys.len();
    let worry_lcm: WorryType = monkeys
        .iter()
        .map(|monkey| monkey.test_factor)
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap();
    MonkeyGame {
        monkeys,
        inspections: vec![0; size],
        worry_adjustment: Worry::Lcm(worry_lcm),
    }
}

struct Monkey {
    items: Vec<WorryType>,
    operation: Box<dyn Fn(WorryType) -> WorryType>,
    test: Box<dyn Fn(WorryType) -> usize>,
    test_factor: WorryType,
}

impl std::str::FromStr for Monkey {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        lines.next(); // first line is Monkey label, assume well-formed (in order, starting with 0)
        let items = lines
            .next()
            .unwrap()
            .trim()
            .replace("Starting items: ", "")
            .split(", ")
            .map(|x| x.parse::<WorryType>().unwrap())
            .collect();

        let operation_str = lines
            .next()
            .unwrap()
            .trim()
            .replace("Operation: new = ", "");
        let operation_str_split: Vec<String> = operation_str
            .split_whitespace()
            .map(str::to_string)
            .collect();

        let operation = Box::new(move |old: WorryType| -> WorryType {
            let x = match operation_str_split[0].as_str() {
                "old" => old,
                n => n.parse::<WorryType>().unwrap(),
            };
            let y = match operation_str_split[2].as_str() {
                "old" => old,
                n => n.parse::<WorryType>().unwrap(),
            };
            match operation_str_split[1].as_str() {
                "*" => x * y,
                "+" => x + y,
                _ => panic!("Operator not supported. Add support."),
            }
        });

        let test_factor = lines
            .next()
            .unwrap()
            .trim()
            .replace("Test: divisible by ", "")
            .parse::<WorryType>()
            .unwrap();
        let test_true_result = lines
            .next()
            .unwrap()
            .trim()
            .replace("If true: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();
        let test_false_result = lines
            .next()
            .unwrap()
            .trim()
            .replace("If false: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();
        let test = Box::new(move |s: WorryType| -> usize {
            if s % test_factor == 0 {
                test_true_result
            } else {
                test_false_result
            }
        });
        Ok(Monkey {
            items,
            operation,
            test,
            test_factor,
        })
    }
}

pub enum Worry {
    Lcm(WorryType),
    Factor(WorryType),
}

pub struct MonkeyGame {
    monkeys: Vec<Monkey>,
    inspections: Vec<InspectType>,
    pub worry_adjustment: Worry,
}

impl MonkeyGame {
    pub fn get_monkey_business(&self) -> InspectType {
        std::collections::BinaryHeap::from(self.inspections.to_owned())
            .into_iter_sorted()
            .take(2)
            .product()
    }

    pub fn play_round(&mut self) {
        for i in 0..self.monkeys.len() {
            self.inspect(i);
        }
    }

    fn inspect(&mut self, i: usize) {
        let items = std::mem::take(&mut self.monkeys[i].items);
        items.iter().for_each(|&x| {
            let mut worry = (self.monkeys[i].operation)(x);
            match self.worry_adjustment {
                Worry::Lcm(lcm) => worry %= lcm,
                Worry::Factor(factor) => worry /= factor,
            };
            let next = (self.monkeys[i].test)(worry);
            self.monkeys[next].items.push(worry);
        });

        self.inspections[i] += items.len() as InspectType;
    }
}
