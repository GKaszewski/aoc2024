use std::fs;

fn check_if_report_is_safe(report: &Vec<i32>) -> bool {
   // report is safe if the difference between adjacent numbers is no more than 3
    for i in 0..report.len() - 1 {
         if (report[i] - report[i + 1]).abs() > 3 {
              return false;
         }
    }

    // it is also safe if the numbers are in increasing order or decreasing order nothing else
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..report.len() - 1 {
        if report[i] < report[i + 1] {
            decreasing = false;
        } else if report[i] > report[i + 1] {
            increasing = false;
        }
        else if report[i] == report[i + 1] {
            increasing = false;
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn check_if_report_is_safe_with_single_bad_level(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut report_copy = report.clone();
        report_copy.remove(i);

        if check_if_report_is_safe(&report_copy) {
            return true;
        }
    }

    false
}

fn part_one() {
    let input = fs::read_to_string("input/day2.txt").unwrap();

    let mut reports = Vec::new();

    
    for line in input.lines() {
        let numbers = line.split_whitespace();
        let report: Vec<i32> = numbers.map(|n| n.parse().unwrap()).collect();
        reports.push(report);
    }

  
    reports.retain(|report| check_if_report_is_safe(report));
    println!("Number of safe reports: {}", reports.len());
}

fn part_two() {
    let input = fs::read_to_string("input/day2.txt").unwrap();

    let mut reports = Vec::new();

    for line in input.lines() {
        let numbers = line.split_whitespace();
        let report: Vec<i32> = numbers.map(|n| n.parse().unwrap()).collect();
        reports.push(report);
    }

    reports.retain(|report| check_if_report_is_safe_with_single_bad_level(report));
    println!("Number of safe reports with a single bad level: {}", reports.len());

}

pub fn day2() {
    part_one();
    part_two();
}