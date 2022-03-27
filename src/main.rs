use std::env;
use std::mem;
use std::fs;
use std::cmp;
use std::fs::File;
use std::io::{self, prelude::*,BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut string_file = contents.replace("\n", " ");

    let mut splitted_data = string_file.split(" ");
    let data_vec: Vec<&str> = splitted_data.collect();

    struct Product {
        type_of_product: String,
        min_time_A: u32,
        min_time_B: u32,
        profit: u32,
        arrival_time: u32,
        end_time: u32,
    }

    let mut products : Vec<Product> = Vec::new();

    let data_size = data_vec.len() / 5;

    for i in 0..data_size {
        let type_of = data_vec[i].to_string();
        let min_time_a = data_vec[i+data_size].parse::<u32>().unwrap();
        let min_time_b = data_vec[i+data_size*2].parse::<u32>().unwrap();
        let profit = data_vec[i+data_size*3].parse::<u32>().unwrap();
        let arrival_time = data_vec[i+data_size*4].parse::<u32>().unwrap();
        let end_time = if type_of == "s" {arrival_time + min_time_a} else {arrival_time + min_time_b};
        
        let mut product = Product {
            type_of_product: type_of,
            min_time_A: min_time_a,
            min_time_B: min_time_b,
            profit: profit,
            arrival_time: arrival_time,
            end_time: end_time,
        };

        products.push(product);
    }

    products.sort_by_key(|key| key.end_time);

    let mut table: Vec<u32> = Vec::new();
    for i in 0..products.len() {
        table.push(products[i].profit);
    }
    
    for i in 1..products.len() {
        let mut prof = products[i].profit;

        for j in 0..i {
            if products[j].end_time > products[i].arrival_time {
                continue;
            } else {
                let mut total = table[j] + products[i].profit;
                let old_total = table[i];
                let got = std::mem::replace(&mut table[i], cmp::max(total, old_total));
            }
        }        
    }

    let result = table[products.len()-1];
    println!("{}", result);
}
