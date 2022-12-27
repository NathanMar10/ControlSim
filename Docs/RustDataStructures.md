1) Will have global environment conditions (screen size, gravity, forces, and so on)
    These will be stored inside of a hashmap from string -> property type, where property can be any of a boolean, int tuple, so on
2) Vector of bodies/ points that will store masses and positions of them all as well as the applied forces
3) Vector of all of the edges in the scene -> Will contain references to the positions in the bodies array 
4) Second vector with all fixed-length edges to constrain the solution

The way that I will solve:
Tally of the forces that are being applied to each point/mass (and obviously their directions/ application position later (for rotation))
As long as I'm not solving perpetually, the answer should be well-defined at each moment and not require some sort of matrix manipulation

So basically, iterate through the connections array and find all of the positions and forces on each point -> but this wouldn't work with fixed connections

HOW TO ENFORCE FIXED-LENGTH CONNECTIONS:
My first idea is to do something kinematics-y like FABRIK, but that isn't exactly what im looking for
Finding an equilibirum would be easy enough with an iterative-like solution, but maybe I could do it with velocity constraints?!?!?!
If i have two spots connected, 0---------0, I know that their NET velocity must be tangent to the circle -> This would require an integration, right??
^ What if, when I have a connected system, I think of it as a purely rotational entity
wait, am i a genius?
I call any body with fixed supports a new type, where It is a compound body
Inside of this compound body, I would sum all of the forces to determine a "center-of-mass" velocity (Which is where the center-of-mass will move to) NOTE: If something is fixed, this breaks? -> So write a special case for it i guess
If not fixed, the center of mass will move and then I can calulate the forces on the components and rotate them a certain angle from the COM? FUCK i need dynamics notes
    This is basically just rigidbody motion kind of stuff but not actually if I have multiple rotating elements -> but thats doable in the fixed case


SCOPE OF WORK:
1) Write up a fake data file that rust will read in
    a) This will basically be a hardcoded proof-of-concept double pendulum
2) Read the file and run the kinematics and output a list of positions that the two points get to 
3) Head to python and draw the dots and see how it works!
4) Then reevaluate the solution path and see where we get