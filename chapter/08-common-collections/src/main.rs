use std::{collections::HashMap, io::stdin};

fn main() {
    given_integers_use_a_vector_to_return_the_median_and_mode();
    convert_strings_to_pig_latin();
    text_interface_to_add_employee_names_to_departments();
}

fn given_integers_use_a_vector_to_return_the_median_and_mode() {
    use rand::{distributions::Uniform, Rng};

    let range = Uniform::from(0..20);
    let values: Vec<u64> = rand::thread_rng().sample_iter(&range).take(10).collect();
    println!(
        "Values: {:?}, Median: {}, Mode: {}",
        values,
        median(&values),
        mode(&values)
    );

    fn median(values: &Vec<u64>) -> u64 {
        // When sorted, the middle value.
        let mut sorted: Vec<u64> = values.clone();
        sorted.sort();

        let middle = sorted.len() / 2;
        return sorted[middle];
    }

    fn mode(values: &Vec<u64>) -> u64 {
        // The value that occurs the most often.
        let mut count: HashMap<u64, u64> = HashMap::new();
        for &i in values {
            let count = count.entry(i).or_insert(0);
            *count += 1;
        }

        let mut mode = 0;
        let mut most = 0;
        for (n, amount) in &count {
            if amount > &most {
                most = *amount;
                mode = *n;
            }
        }

        return mode;
    }
}

fn convert_strings_to_pig_latin() {
    let strings = ["hello", "apple", "giraffe", "calculating", "Здравствуйте"];

    for s in strings {
        println!("{}: {}", s, convert(s));
    }

    fn is_vowel(c: char) -> bool {
        return ['a', 'e', 'i', 'o', 'u'].contains(&c);
    }

    fn convert(string: &str) -> String {
        if string.is_empty() {
            return string.to_string();
        }

        // If the word starts with a vowel, "hay" is added to the end.
        if is_vowel(string.chars().nth(0).unwrap()) {
            let mut word = String::from(string);
            word.push_str("hay");
            return word;
        } else {
            // Find first consonant, and move it to the end of the word, suffixed with "ay".
            // i.e. "first" becomes" "irstfay".
            for (i, c) in string.chars().enumerate() {
                if c.is_ascii_alphabetic() && !is_vowel(c) {
                    let mut word = String::from(string);
                    word.remove(i);
                    word.push_str(&(c.to_string() + "ay"));
                    return word;
                }
            }
        }

        return string.to_string();
    }
}

fn text_interface_to_add_employee_names_to_departments() {
    println!("Welcome to EMPLOYEE MANAGER!");

    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        if employees.is_empty() {
            println!("Enter a new department (e.g. Engineering), or 0 to quit.");
        } else {
            for (i, d) in employees.keys().enumerate() {
                println!("{}: {}", i + 1, d);
            }
            println!("Enter an existing department number, new department (e.g. Engineering), or 0 to quit.");
        }

        let mut line = String::new();
        stdin().read_line(&mut line).expect("Could not read line");

        if line.is_empty() {
            continue;
        }

        if line.trim_end().eq("0") {
            break;
        }

        let department: String = match line.trim_end().parse::<usize>() {
            Ok(i) => {
                let (_, key) = employees.keys().enumerate().nth(i - 1).unwrap();
                key.trim_end().to_string()
            }
            Err(_) => line.trim_end().to_string(),
        };

        line.clear();
        println!("Enter employee name or just hit enter to abort");
        stdin().read_line(&mut line).expect("Could not read line");

        if line.is_empty() {
            continue;
        } else {
            line = line.trim_end().to_string();
        }

        println!("Adding {} to {}", line, department);
        employees.entry(department).or_insert(Vec::new()).push(line);
    }

    println!("Thanks for using EMPLOYEE MANAGER!");
    dbg!(employees);
}
