pub struct Dot {
    pub id: i32,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
}


/*

Ok aso uh wahts the plan here then? 

I THINK that I'm going to have the first position hardcoded and then I'll just calculate where the pendulum ends up
then I can print that out and bobs your uncle everythings fine and dandy like a hard candy christmas!



*/
pub fn step(dots: & mut Vec<Dot>) {
    /*
        I'll comment this all way better after i get a proof of concept working, but first I'm going to hack together a single pendulum deal
    */

    // Note to self, this vector will currently only have a single entry in it, but ill make it better one day!

    let t_step = 0.0001; // sec/ Frame
    let g = 500.0; // (Pixels/ sec^2)
    let rad = 100.0; // Pixels (rod length)
    let stiff = 10.0; // Force/pixel (rod stiffness)
    let mass = 1.0; // mass
    /*
        Ok so im about to go way too hard and determine the actualy fricken properties of a rod. This is going to suck but will be ok
    */

    let grav_force = mass * g; // be positive 
    // call tension positive then this wrong
    let rod_force = stiff * (((400.0 - dots[0].position.0).powi(2) + (400.0 - dots[0].position.1).powi(2)).powf(0.5) - rad);
    let rod_force_x = rod_force * (400.0 - dots[0].position.0)/rad; //be positive
    let rod_force_y = rod_force * (400.0 - dots[0].position.1)/rad; // be negative
    // First, we will solve for the new acceleration
    // this points downwards and needs to be covered by the centripital

    // Add the acceleration to the position
    let this_dot = & mut dots[0];
    this_dot.velocity.0 += rod_force_x / mass * t_step;
    this_dot.velocity.1 += ( grav_force + rod_force_y) / mass * t_step;
    //println!("Rod Force Y: {}", rod_force_y);

    this_dot.position.0 += this_dot.velocity.0 * t_step;
    this_dot.position.1 += this_dot.velocity.1 * t_step;
    //println!("Pos is {:?}", this_dot.position);
    // Repeat?
}