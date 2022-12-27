
mod simulation;

use std::fs::File;
use std::path::Path;
use std::io::Write;

fn main() {


    let path = Path::new("../Outputs/output.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };


    let mut dots:Vec<simulation::Dot> = Vec::new();
    dots.push(simulation::Dot{
        position:(300.0, 400.0),
        velocity:(0.0, 0.0),
        id:0,
    });

    for ind in 1..100000 {
        simulation::step(& mut dots);
        if ind % 100 == 0 {
            write_dots(&dots, & mut file);
        }
    };




}

fn write_dots(dots: &Vec<simulation::Dot>, file: & mut File) {

    match write!(file, "({:.2}, {:.2})\n", dots[0].position.0, dots[0].position.1) {
        Err(_) => panic!("Couldn't write"),
        Ok(file) => file,
    };

    
}