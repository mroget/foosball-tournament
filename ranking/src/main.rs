mod collision;
mod genetic;
mod naif;
mod heuristic;
use std::env;
use std::time::Instant;
use std::time::Duration;
use std::io;
use tqdm::tqdm;
use collision::collision;
use naif::naif;
use genetic::*;
use heuristic::heuristic;

fn gen_run(inst : &Instance, debug : bool, version : i32, seconds : u64) -> Indiv {
    let duration = Duration::from_secs(seconds);
    match version {
                0 => {genetic(&inst, 100, 100, 100, duration, debug, false)},
                1 => {genetic(&inst, 100, 100, 100, duration, debug, true)},
                2 => {genetic(&inst, 100, 100, 10, duration, debug, false)},
                3 => {genetic(&inst, 100, 100, 10, duration, debug, true)},
                4 => {genetic(&inst, 500, 500, 100, duration, debug, false)},
                5 => {genetic(&inst, 500, 500, 100, duration, debug, true)},
                6 => {genetic(&inst, 10, 10, 10, duration, debug, false)},
                7 => {genetic(&inst, 10, 10, 10, duration, debug, true)},
                8 => {genetic(&inst, 10, 10, 20, duration, debug, false)},
                9 => {genetic(&inst, 20, 20, 10, duration, debug, false)},
                10 => {genetic(&inst, 20, 20, 20, duration, debug, false)},
                11 => {genetic(&inst, 500, 500, 10, duration, debug, false)},
                12 => {genetic(&inst, 100, 100, 10, duration, debug, false)},
                13 => {genetic(&inst, 50, 50, 10, duration, debug, false)},
                13 => {genetic(&inst, 30, 30, 10, duration, debug, false)},
                _ => {genetic(&inst, 500, 500, 100, duration, debug, false)},
            }
}


fn tester(inst : &Instance, debug : bool, version : i32, seconds : u64, nb : usize) -> f64{
    tqdm((0..nb)
        .map(|_x| gen_run(inst, debug, version, seconds).score as f64))
        .fold(0., |acc, x| acc+x ) / (nb as f64)
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let debug = args.contains(&String::from("-d"));
    let exact = args.contains(&String::from("-e"));
    let use_heuristic = args.contains(&String::from("-h"));
    let parallel = args.contains(&String::from("-p"));
    let mut version = -1;
    for i in 0..args.len() {
        if args[i] == String::from("-v") && i < args.len()-1 {
            version = args[i+1].parse().expect("version must be an integer !");
            break;
        }
    }
    let mut tests = 0;
    for i in 0..args.len() {
        if args[i] == String::from("-t") && i < args.len()-1 {
            tests = args[i+1].parse().expect("tests number must be an integer !");
            break;
        }
    }
    let mut seconds = 1;
    for i in 0..args.len() {
        if args[i] == String::from("-s") && i < args.len()-1 {
            seconds = args[i+1].parse().expect("seconds must be an integer !");
            break;
        }
    }
    
    let inst = Instance::read();

    if exact {
        println!("{}", naif(&inst, parallel));
    }
    else if use_heuristic {
        println!("{}", heuristic(&inst));
    }
    else {
        if tests == 0 {
            let res = gen_run(&inst, debug, version, seconds);
            println!("{}", res);
        }
        else {
            println!("{}", tester(&inst, debug, version, seconds, tests));
        }
    }

}
