

pub struct Dot {
    pub id: usize,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub force: (f32, f32),
    pub mass: f32,
    pub fixed: bool,
}

pub struct Edge {
    pub id: usize,
    pub connections: (usize, usize),
    pub stiffness: f32,
    pub len: f32,
}


/*

Ok aso uh wahts the plan here then? 

I THINK that I'm going to have the first position hardcoded and then I'll just calculate where the pendulum ends up
then I can print that out and bobs your uncle everythings fine and dandy like a hard candy christmas!

*/
pub fn step(dots: & mut Vec<Dot>, edges: & Vec<Edge>) {

    let t_step = 0.000001; // sec/ Frame
    let g = 500.0; // (Pixels/ sec^2)

    /*
        General Flow: 
        iterate through each dot and zero out the total force (to neutral with gravity)
        iterate through each edge and add on the force to said edge
        iterate through each dot and apply these changes
    */

    // Resets the force on each dot so that the edges can be applied
    for dot in &mut *dots {
        dot.force = (0.0, dot.mass * g);
        
    };

    // Sums the rest of the forces that are applied to each mass
    for edge in edges {
        let d1:& Dot = & dots[edge.connections.0];
        let d2:& Dot = & dots[edge.connections.1];
        if d1.id != edge.connections.0 || d2.id != edge.connections.1 {
            panic!("Dots are not being stored at the proper position!");
        }
        let x_delt:f32 = d1.position.0 - d2.position.0;
        let y_delt:f32 = d1.position.1 - d2.position.1;
       
        let curr_len:f32 = ( x_delt.powi(2) + y_delt.powi(2) ).powf(0.5);
        let total_force:f32 = (curr_len - edge.len) * edge.stiffness;

        

       /* Notes for implementation below:
        * x_delt > 0 when d1 is further to the right. Force on d1 is NEGATIVE when x_delt is positive.
        *   Because it should be pulled to the left
        * y_delt > 0 when d1 is further down. Force on d1 is NEGATIVE when y_delt is positive.
        * This is because a negative force is an upwards force, as per the implementation of screen coordinates.
        */

        let d1:&mut Dot = &mut dots[edge.connections.0];
        d1.force.0 -= x_delt / curr_len * total_force;
        d1.force.1 -= y_delt / curr_len * total_force;
        

        let d2:&mut Dot = &mut dots[edge.connections.1];
        d2.force.0 += x_delt / curr_len * total_force;
        d2.force.1 += y_delt / curr_len * total_force;
    };

    // Calculates the new positions and velocities for the masses
    for dot in dots {
        
        if ! dot.fixed { 
            dot.velocity.0 += dot.force.0 / dot.mass * t_step;
            dot.velocity.1 += dot.force.1 / dot.mass * t_step;
            dot.position.0 += dot.velocity.0 * t_step;
            dot.position.1 += dot.velocity.1 * t_step;
        }
    };
}

/*

Ok SO, this is going to parse through my grammar literally just so that I can prove that I'm a badass

*/
pub fn read_scene() -> (Vec<Dot>, Vec<Edge>) {
    
}