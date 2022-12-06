use std::{convert, fs};

fn main() {
    let elf_snack_bags: Vec<Vec<u32>> = read_snack_bags();
    let mut elf_totals: Vec<u32> = vec![];

    for snack_bag in elf_snack_bags {
        let total: u32 = get_total_calories(snack_bag);
        println!("total: {}", total);
        elf_totals.push(total);
    }

    elf_totals.sort_by(|a, b| b.cmp(a));
    let highest_calorie_bag: u32 = elf_totals[0];
    println!("highest_calorie_bag: {}", highest_calorie_bag)
}

fn get_total_calories(calorie_list: Vec<u32>) -> u32 {
    let mut total: u32 = 0;
    for calorie_count in calorie_list.iter() {
        println!("calorie_count: {}", calorie_count);
        total += calorie_count;
    }

    return total;
}

fn read_snack_bags() -> Vec<Vec<u32>> {
    let file_content = fs::read_to_string("input.txt").expect("Should read the file");
    println!("file_content: {}", file_content);

    let string_bags = file_content.split("\n\n");
    let mut int_bags: Vec<Vec<u32>> = vec![];

    for string_bag in string_bags {
        println!("string_bag: {}", string_bag);
        let mut int_bag = vec![];
        let string_snacks = string_bag.split("\n");
        for string_snack in string_snacks {
            if string_snack.len() <= 0 {
                break;
            }
            println!("string_snack: {}", string_snack);
            let int_snack: u32 = string_snack.parse().unwrap();
            println!("int_snack: {}", int_snack);
            int_bag.push(int_snack);
        }
        int_bags.push(int_bag);
    }

    return int_bags;
}
