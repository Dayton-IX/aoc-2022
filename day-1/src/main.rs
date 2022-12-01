fn main() {
    let elf_snack_bags: Vec<Vec<u32>> = vec![vec![8, 2, 10], vec![9, 11, 14]];
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
