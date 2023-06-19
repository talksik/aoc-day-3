pub mod priority;
use priority::*;
use std::collections::HashMap;

// part one
fn get_sum_priorities_errors(rucksacks: Vec<String>) -> u32 {
    // get a rucksack
    // split the items into two compartments
    // note: there is only one error per rucksack
    // go through the items in first compartment, put in a map
    // go through the second compartment, check if the item is in the map
    // if this item is in the map, then we have found the error
    // find the priority of the item

    let mut total_priority_errors = 0;
    for rucksack in rucksacks.into_iter() {
        let length = rucksack.len();
        if length % 2 == 1 {
            panic!("rucksacks must be even");
        }

        // [1, 2, 3, 4]
        // 0 -> 2
        // 2 -> 4
        let first_compartment = rucksack[0..(length / 2)].to_string();
        let second_compartment = rucksack[(length / 2)..length].to_string();

        let mut first_compartment_map: HashMap<Item, bool> = HashMap::new();
        for item in first_compartment.chars() {
            // convert char to &str
            let char = item.to_string();
            let item = Item::from_str(&char);
            first_compartment_map.insert(item, true);
        }

        for item in second_compartment.chars() {
            // convert char to &str
            let char = item.to_string();
            let item = Item::from_str(&char);
            if first_compartment_map.contains_key(&item) {
                // we have found the error
                let priority = item.get_priority();
                total_priority_errors += priority;
                // break out so that we don't double count the error if it appears twice
                break;
            }
        }
    }

    total_priority_errors
}

struct Group {
    rucksacks: [String; 3],
}

impl Group {
    // get first occurence of the first rucksack's commonality with other two rucksacks
    pub fn get_common_item(&self) -> Item {
        for item in self.rucksacks[0].chars() {
            // convert char to &str
            let item_string = item.to_string();
            let item = Item::from_str(&item_string);
            if self.rucksacks[1].contains(&item_string) && self.rucksacks[2].contains(&item_string)
            {
                return item;
            }
        }

        panic!("no common item found");
    }
}

// get the item of each group that is common to all rucksacks
// get the priority of that item
fn get_priority_all_groups_badge(rucksacks: Vec<String>) -> u32 {
    let mut total_badges_priorities = 0;

    // organize the rucksacks into groups of 3
    let mut i = 0;
    while i < rucksacks.len() {
        let group = Group {
            rucksacks: [
                rucksacks[i].to_string(),
                rucksacks[i + 1].to_string(),
                rucksacks[i + 2].to_string(),
            ],
        };

        let common_item = group.get_common_item();
        let prioriy = common_item.get_priority();
        total_badges_priorities += prioriy;

        i += 3;
    }

    total_badges_priorities
}

fn read_input_file(file_name: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(file_name).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut rucksacks: Vec<String> = Vec::new();
    for line in contents.lines() {
        rucksacks.push(line.to_string());
    }
    rucksacks
}

fn main() {
    println!("Hello, world!");

    let input = read_input_file("input.txt");

    let total_priority_errors = get_sum_priorities_errors(input.clone());
    println!("total priority errors: {}", total_priority_errors);

    let total_badges_priorities = get_priority_all_groups_badge(input);
    println!("total badges priorities: {}", total_badges_priorities);
}

#[cfg(test)]
mod tests {
    use super::*;

    // part 1
    #[test]
    fn test_total_errors() {
        let input: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(get_sum_priorities_errors(input), 157);
    }

    // part 2
    #[test]
    fn test_badge_priorities() {
        let input: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(get_priority_all_groups_badge(input), 70);
    }
}
