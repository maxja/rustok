// use std::{thread};

#[allow(dead_code)]
fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let china = "你好，世界";
    let india = "नमस्ते दुनिया";
    let ukraine = "Привіт, світ!";
    let finland = "Hei maailma!";

    let regions = [southern_germany, japan, china, india, ukraine, finland];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

#[allow(dead_code)]
fn print_penguin_profiles() {
    let penguin_data = "\
        common name,length (cm),weight (kg),sex,age (years)
        Little penguin,33,1.4,female,1
        Yellow-eyed penguin,65,6.5,male,1.5
        Fiordland penguin,60,6.5,male,2
        Emperor penguin,110,8,male,5
        Invalid data,,,";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>()
            && let Ok(weight) = fields[2].parse::<f32>()
            && let sex = fields[3]
            && let Ok(age) = fields[4].parse::<f32>()
        {
            println!(
                "{}: length={}cm, weight={}kg, sex={}, age={}",
                name, length, weight, sex, age
            );
        }
    }
}

/*
#[derive(Debug)]
enum Cereal {
    Oatmeal,
    Wheat,
    Rice,
    Corn,
    Barley,
    Buckwheat,
}

#[allow(dead_code)]
fn memory_safety() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Oatmeal);
    drop(grains);
    println!("{:?}", grains);
}
*/

/*
fn thread_safety() {
    let mut data = 100;
    thread::spawn(|| {
        data = 500;
    });
    thread::spawn(|| {
        data = 1_000;
    });
    println!("{}", data);
}
*/

fn buffer_safety() {
    let fruit = vec!['_', '_', '_'];
    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, '_');
}

fn main() {
    // greet_world();
    // print_penguin_profiles();
    // memory_safety();
    // thread_safety();

    buffer_safety();
}
