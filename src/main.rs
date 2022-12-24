use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};


struct Monkey {
    inventory: VecDeque<Item>,
    worry_function: fn(x: i32)->i32,
    test: fn(x: i32)->bool,
    action_positive: usize,
    action_negative: usize,
    activity: i32
}

struct Item {
    worry_level: i32,
}

impl Monkey{

    fn mess_around(&mut self)  -> (usize, Item) {
        let mut item = self.inventory.pop_front().unwrap();
        item.worry_level = (self.worry_function)(item.worry_level);
        item.worry_level = item.worry_level / 3;

        self.activity += 1;

        if (self.test)(item.worry_level) {
            (self.action_positive, item)
        } else{
            (self.action_negative, item)
        }
    }

    fn to_string(&self) -> String {
        let mut res = "Inventory: ".to_string();

        for i in self.inventory.iter() {
            res.push_str(format!("{},", i.worry_level).as_str());
        }

        res
    }
}

fn main() {

    let mut monkeys: Vec<Monkey> = vec![];

    monkeys.push(Monkey{
        inventory: VecDeque::from([Item{worry_level: 79}, Item{worry_level: 98}]),
        worry_function: |w| w * 19,
        test: |w| w % 23 == 0,
        action_positive: 2,
        action_negative: 3,
        activity: 0
    });

    monkeys.push(Monkey{
        inventory: VecDeque::from([
            Item{worry_level: 54}, Item{worry_level: 65}, Item{worry_level:75}, Item{worry_level:74}]),
        worry_function: |w| w + 6,
        test: |w| w % 19 == 0,
        action_positive: 2,
        action_negative: 0,
        activity: 0
    });

    monkeys.push(Monkey{
        inventory: VecDeque::from([
            Item{worry_level:79}, Item{worry_level:60}, Item{worry_level:97}
        ]),
        worry_function: |w| w * w,
        test: |w| w % 13 == 0,
        action_positive: 1,
        action_negative: 3,
        activity: 0
    });

    monkeys.push(Monkey{
        inventory: VecDeque::from([Item{worry_level: 74}]),
        worry_function: |w| w + 3,
        test: |w| w % 17 == 0,
        action_positive: 0,
        action_negative: 1,
        activity: 0
    });

    let rounds = 20;

    for round in 0..rounds {

        for monkey_index in 0..monkeys.len() {

            let monkey = &mut monkeys[monkey_index];

            let mut cache = VecDeque::new();

            while !(monkey).inventory.is_empty() {
                let (throw_to, item) = monkey.mess_around();

                println!("Monkey {} throws item with worry level {} to monkey {}", monkey_index, item.worry_level, throw_to);

                cache.push_front((throw_to, item));
            }

            while !cache.is_empty(){
                let (throw_to, item) = cache.pop_back().unwrap();
                (&mut monkeys[throw_to]).inventory.push_back(item);
            }

        }

        println!("after round {}", round);

        for (i, m) in monkeys.iter().enumerate() {
            println!("Monkey {}: {}", i, m.to_string());
        }
    }

    monkeys.sort_by(|a, b| a.activity.cmp(&b.activity));

    println!("selected monkey activity: 1st {} 2nd {} 3rd {} 4th {} total {}",
             monkeys[0].activity, monkeys[1].activity, monkeys[2].activity, monkeys[3].activity, monkeys[2].activity * monkeys[3].activity);
}