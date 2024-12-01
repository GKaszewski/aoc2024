use std::{collections::{HashMap, HashSet}, fs};

fn pair_up(left: &Vec<i32>, right: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut pairs = Vec::new();

    let mut left_sorted = left.clone();
    left_sorted.sort();
    
    let mut right_sorted = right.clone();
    right_sorted.sort();

    for i in 0..left.len() {
        pairs.push((left_sorted[i], right_sorted[i]));
    }

    pairs
}

fn distance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let pairs = pair_up(left, right);

    let mut total_distance = 0;

    for pair in pairs {
        total_distance += (pair.0 - pair.1).abs();
    }

    total_distance
}

fn part_one() -> i32 {
let file_path = "input/day1.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        left_list.push(numbers.next().unwrap().parse::<i32>().unwrap());
        right_list.push(numbers.next().unwrap().parse::<i32>().unwrap());
    }

    let total_distance = distance(&left_list, &right_list);

    total_distance
}

fn get_similarity(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut appearance_count: HashMap<i32, i32> = HashMap::new();
    let unique_left: HashSet<i32> = left.iter().cloned().collect();

    for left_element in unique_left {
        for right_element in right {
            if left_element == *right_element {
                let count = appearance_count.entry(left_element).or_insert(0);
                *count += 1;
            }
        }
    }


    let similiarity = left.iter().map(|element| {
        let count = appearance_count.get(element).unwrap_or(&0);

        element * count
    });


    similiarity.sum()
}

fn part_two() -> i32 {
    let file_path = "input/day1.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        left_list.push(numbers.next().unwrap().parse::<i32>().unwrap());
        right_list.push(numbers.next().unwrap().parse::<i32>().unwrap());
    }


    let similarity = get_similarity(&left_list, &right_list);
    
    similarity
}

pub fn day1() {
    let total_distance = part_one();
    println!("Total distance: {}", total_distance);

    let similarity_score = part_two();
    println!("Similarity score: {}", similarity_score);
}