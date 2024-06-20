use std::collections::HashMap;
use std::vec::Vec;
use rand::Rng;

// end of chapter challenges
// 1 given list of ints, use a vector and return the mean, median, and mode
// 2 convert strings to pig latin
// 3 use hashmap and vector to add employees to departments and retrieve a list of employees in departments
fn main() {
    // mean, median, mode
    let mut scoresArr: [u32; 100] = [0; 100];
    let mut scoresVec: Vec<u32> = Vec::new();
    let mut numCount = HashMap::new();
    let mut largest: u32 = 0;
    println!("Hello, world!");
    let mut rng = rand::thread_rng();

    for n in 0..100 {
        let addNum = rng.gen_range(0..50);
        scoresArr[n] = addNum;
        scoresVec.push(addNum);
    }
    
    scoresVec.sort_unstable();
    let mut currLargest = 0;
    let mut mean = 0f32;
    for i in 0..scoresVec.len() {
        println!("Element #{}: {}", i, scoresVec[i]);
        mean += scoresVec[i] as f32;
        let count = numCount.entry(scoresVec[i]).or_insert(0);
        *count += 1;
        if *count > largest {
            largest = *count;
            currLargest = scoresVec[i];
        }

    }

    for (key, val) in numCount.iter() {
        let mut extra: String;
        if *key == currLargest {
            extra = String::from("\tLARGEST---------------------------------");
        } else {
            extra = String::from("");
        }
        println!("Key: {}\t\tValue: {}{}", key, val, extra);
    }

    // median = half way in odd # of elements or average of both elements in the middle
    println!("Median = {}", (scoresVec[49] as f32+scoresVec[50] as f32)/2.0);
    // mean
    println!("Mean = {}", mean/100f32);
    // mode
    println!("Mode+Counts = {}/{}", currLargest, largest);
    employer_loop();

}
fn add_emp(dpt_hash: &mut HashMap<String, Vec<String>>) {
    let mut name: String = String::new();
    while name.chars().count() < 1 {
        println!("Please enter emp name:");
        std::io::stdin().read_line(&mut name)
            .expect("Couldn't parse name");
        match name.trim() {
            "exit" => return,
            _ => {}
        }
    }


    

    let mut input = String::new();
    while input.chars().count() < 1 {
        println!("Please enter a department name:");
        std::io::stdin().read_line(&mut input)
            .expect("Couldn't parse line");

        match input.trim() {
            "exit" => break,
            _ => {}
        }

        let key = input.trim().to_string();
        dpt_hash.entry(key)
            .or_insert_with(Vec::new)
            .push(name.trim().to_string());
    }
}

fn sort_emps(dpt_hash: &mut HashMap<String, Vec<String>>) {
    for (key, value) in dpt_hash.clone().iter() {
        dpt_hash.entry(key.clone())
            .or_insert_with(Vec::new)
            .sort();
    }
}

fn display_emps(dpt_hash: &HashMap<String, Vec<String>>) {
    let mut dpt = String::new();
    while dpt.chars().count() < 1 {
        println!("Enter what you wanna check (\"company\" or a department name):");
        std::io::stdin().read_line(&mut dpt)
            .expect("Couldn't read what employees you wanna check on");
    }


    match dpt.trim() {
        "company" => println!("{:#?}", dpt_hash),
        _ => println!("{:#?}", dpt_hash.get(&dpt.trim().to_string()))
    }
}

fn employer_loop() {
    let mut dpt_list:HashMap<String, Vec<String>> = HashMap::new();

    let mut inpoot = String::new();
    while true{
        println!("What to do?\n1. Add emp\n2. Check emp\n3. exit");
        std::io::stdin().read_line(&mut inpoot)
            .expect("Couldn't read desired action");

        println!("You entered: {}", inpoot);
        match inpoot.trim() {
            "1" => {
                println!("Adding emp");
                add_emp(&mut dpt_list);
                inpoot = String::new();
                continue;
            },
            "2" => {
                println!("checking emp");
                sort_emps(&mut dpt_list);
                display_emps(&dpt_list);
                inpoot = String::new();
                continue;
            },
            "3" => break,
            _ => continue
        }
    }
}