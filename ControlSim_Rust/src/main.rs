
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
    let mut edges:Vec<simulation::Edge> = Vec::new();

    dots.push(simulation::Dot{
        position:(400.0, 400.0),
        velocity:(0.0, 0.0),
        force:(0.0, 0.0),
        fixed: true,
        mass: 1.0,
        id:0,
    });

    dots.push(simulation::Dot{
        position:(300.0, 400.0),
        velocity:(0.0, 0.0),
        fixed: false,
        force:(0.0, 0.0),
        mass: 4.0,
        id:1,
    });

    dots.push(simulation::Dot{
        position:(300.0, 300.0),
        velocity:(0.0, 0.0),
        fixed: false,
        force:(0.0, 0.0),
        mass: 1.0,
        id:2,
    });

    dots.push(simulation::Dot{
        position:(200.0, 300.0),
        velocity:(0.0, 0.0),
        fixed: false,
        force:(0.0, 0.0),
        mass: 1.0,
        id:3,
    });

    edges.push(simulation::Edge{
        id:0,
        connections:(0, 1),
        stiffness:100000.0,
        len: 100.0,
    });

    edges.push(simulation::Edge{
        id:1,
        connections:(1, 2),
        stiffness:100000.0,
        len: 100.0,
    });

    edges.push(simulation::Edge{
        id:1,
        connections:(2, 3),
        stiffness:10.0,
        len: 100.0,
    });

    for ind in 1..100_000_000 {
        simulation::step(& mut dots, &edges);
        if ind % 10000 == 0 {
            write_dots(&dots, & mut file);
        }
    };




}

fn write_dots(dots: &Vec<simulation::Dot>, file: & mut File) {

    match write!(file, "({:.2}, {:.2}), ({:.2}, {:.2}), ({:.2}, {:.2})\n", dots[1].position.0, dots[1].position.1, dots[2].position.0, dots[2].position.1, dots[3].position.0, dots[3].position.1) {
        Err(_) => panic!("Couldn't write"),
        Ok(file) => file,
    };

    
}