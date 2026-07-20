






use macroquad::prelude::*;

#[macroquad::main("particle stuff")]
async fn main() {


    //particle object
    struct Particle {
        x: f32,
        y: f32,
        r: f32,
        xspeed: f32,
        yspeed: f32,
        xaccel: f32,
        yaccel: f32,
        fixed: bool,
        color: Color,
        
    }

    //particle functions
    impl Particle {

        //constructor
        fn new(x: f32, y: f32, r: f32) -> Particle {
            Particle {
                x,
                y,
                r,
                xspeed: 0.0,
                yspeed: 0.0,
                xaccel: 0.0,
                yaccel: 0.0,
                fixed: false,
                color: BLUE,
            }
        }

        //calculate distance
        fn distance(&self, particle: &Particle) -> f32{
            return ((self.x - particle.x).powf(2.0) + (self.y - particle.y).powf(2.0)).powf(0.5);
        }

        //check for collision
        fn collide(&self, particle: &Particle) -> bool{
            if self.distance(particle) >= self.r + particle.r{
                return true;
            }
            else{
                return false;
            }
        }

        //move
        fn on_tick(&mut self, time_diff: f64, particle: &Particle){

            self.speed(particle);

            if !self.fixed && time_diff >= 0.001 && self.collide(particle){
                self.xspeed = 0.0;
                self.yspeed = 0.0;

                self.xspeed += self.xaccel;
                self.yspeed += self.yaccel;

                self.xaccel = 0.0;
                self.yaccel = 0.0;

                self.x += &self.xspeed;
                self.y += &self.yspeed;
            }
        }

        //calculate acceleration
        fn speed(&mut self, particle: &Particle){
            self.xaccel += (particle.x + particle.r + self.r*4.0 - self.x)/100.0;
            self.yaccel += (particle.y + particle.r + self.r*4.0 - self.y)/100.0;
        }

    }

    


    //time management
    let mut prev_time = 0.0f64;
    let mut time_diff: f64;





    //test particles
    let mut p1 = Particle::new(200.0, 300.0, 30.0);
        p1.color = GREEN;
    let mut p2 = Particle::new(700.0, 300.0, 30.0);
        p2.color = YELLOW;
    let mut p3 = Particle::new(150.0, 350.0, 30.0);
        p3.color = RED;
    let mut p4 = Particle::new(550.0, 250.0, 30.0);


    //particle array
    let mut particles: [Particle; 4] = [p1, p2, p3, p4];


    //main loop
    loop {

        clear_background(BLACK);


        
        time_diff = get_time()-prev_time;



        //iteration over particles to calculate acceleration/collisions
        let n = particles.len();
        for i in 0..n {
            for j in 0..n {
                if j == i {
                    continue;
                }

                let [particle_a, particle_b] = particles.get_disjoint_mut([i, j]).unwrap();
                particle_a.on_tick(time_diff, particle_b);
            }

            let particle_a = &particles[i];
            draw_circle(particle_a.x, particle_a.y, particle_a.r, particle_a.color);
        }





        if time_diff >= 0.001{
            prev_time = get_time();
        }
        

        

        next_frame().await
    }
}
