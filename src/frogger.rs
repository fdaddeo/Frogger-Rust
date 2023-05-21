use std::any::Any;
use std::cmp::{min, max};

use crate::actor::*;
use crate::rand::*;

pub struct Vehicle 
{
    pos: Pt,
    aspect: i32,
    size: Pt,
    speed: i32
}
impl Vehicle 
{
    pub fn new(pos: Pt, speed: i32) -> Vehicle 
    {
        let aspect = randint(0, 2);
        let size = if aspect !=2 { pt(32, 26) } else { pt(62, 24) };

        Vehicle { pos: pos, aspect: aspect, size: size, speed: speed }
    }
}
impl Actor for Vehicle 
{
    fn act(&mut self, arena: &mut ArenaStatus) 
    {
        let scr = arena.size();
        self.pos.x = self.pos.x + self.speed;
        
        self.pos.x = if self.pos.x > scr.x + 70 && self.speed > 0 { - 70 } else { self.pos.x };
        self.pos.x = if self.pos.x < - 70 && self.speed < 0{ scr.x } else { self.pos.x };
    }

    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
    
    fn sprite(&self) -> Option<Pt> 
    { 
        if self.aspect == 0  // Yellow Vehicle Sprite
        {
            if self.speed >= 0 
            {
                Some(pt(192, 4))
            }
            else 
            {
                Some(pt(192, 36))
            }
        }
        else if self.aspect == 1  // White Vehicle Sprite
        {
            if self.speed >= 0 
            {
                Some(pt(224, 4))
            }
            else 
            {
                Some(pt(224, 36))
            }
        }
        else // Camion Sprite
        {
            if self.speed >= 0 
            {
                Some(pt(258, 68))
            }
            else 
            {
                Some(pt(192, 68))
            }
        }
    }
}

pub struct Raft 
{
    pos: Pt,
    size: Pt,
    speed: i32
}
impl Raft
{
    pub fn new(pos: Pt, speed: i32) -> Raft
    {
        let size = pt(96, 20);

        Raft { pos: pos, size: size, speed: speed }
    }     
}
impl Actor for Raft 
{
    fn act(&mut self, arena: &mut ArenaStatus) 
    {
        let scr = arena.size();
        self.pos.x = self.pos.x + self.speed;
        
        self.pos.x = if self.pos.x > scr.x + self.size.x && self.speed > 0 { - self.size.x } else { self.pos.x };
        self.pos.x = if self.pos.x < - self.size.x && self.speed < 0 { scr.x } else { self.pos.x };
    }

    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
    fn sprite(&self) -> Option<Pt> { Some(pt(192, 102)) }
}

pub struct Water
{
    pos: Pt,
    size: Pt
}
impl Water 
{
    pub fn new(pos: Pt) -> Water
    {
        Water { pos: pos, size: pt(640, 156) }
    }    
}
impl Actor for Water 
{
    fn act(&mut self, _arena: &mut ArenaStatus) { }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
    fn sprite(&self) -> Option<Pt> { None }
}

pub struct Frog
{
    pos: Pt,
    starting_pos: Pt,
    size: Pt,
    starting_size: Pt,
    step: Pt,
    speed: i32,
    sprite: Pt,
    starting_sprite: Pt,
    lives: i32,
    blinking: i32,
    in_water: bool,
    wins: Vec<bool>
}
impl Frog
{
    pub fn new(pos: Pt) -> Frog
    {
        let starting_size = pt(24, 18);
        let starting_sprite = pt(68, 6);
        let mut wins: Vec<bool> = vec![];

        for _ in 0..5
        {
            wins.push(false);
        }

        Frog { pos: pos,
               starting_pos: pos,
               size: starting_size,
               starting_size: starting_size,
               step: pt(0, 0),
               speed: 32,
               sprite: starting_sprite,
               starting_sprite: starting_sprite,
               lives: 3,
               blinking: 0,
               in_water: false,
               wins: wins }
    }

    pub fn get_wins(&self) -> &Vec<bool> { &self.wins }
    pub fn get_starting_sprite(&self) -> Pt { self.starting_sprite }
    pub fn get_starting_size(&self) -> Pt { self.starting_size }

    fn reset_position(&mut self)
    {
        self.pos = self.starting_pos;
        self.size = self.starting_size;
        self.sprite = self.starting_sprite;
    }

    fn lose_life(&mut self)
    {
        self.blinking = 60;
        self.in_water = false;
        self.lives = self.lives - 1;
        self.pos = self.starting_pos;
        self.size = self.starting_size;
        self.sprite = self.starting_sprite;
    }
}
impl Actor for Frog
{
    fn act(&mut self, arena: &mut ArenaStatus) 
    {
        let scr = arena.size();
        let current_keys = arena.current_keys();
        let previous_keys = arena.previous_keys();

        self.step = pt(0, 0);

        if self.blinking == 0
        {
            for other in arena.collisions()
            {
                if let Some(_) = other.as_any().downcast_ref::<Vehicle>()  {
                    self.lose_life();
                }

                if let Some(_) = other.as_any().downcast_ref::<Water>() {
                    self.in_water = true;
                }

                if let Some(raft) = other.as_any().downcast_ref::<Raft>() {
                    self.in_water = false;
                    self.step.x = raft.speed;
                }
            }

            // Check whatever it is colliding with water
            if self.in_water { self.lose_life(); }
        }

        // Check win, from left slot to right slot.
        if !self.wins[0] && self.pos.x + (self.size.x / 2) > 48 && self.pos.x + (self.size.x / 2) < 80 && self.pos.y + (self.size.y / 2) > 58 && self.pos.y + (self.size.y / 2) < 80
        {
            self.wins[0] = true;
            self.reset_position();
        }
        else if !self.wins[1] && self.pos.x + (self.size.x / 2) > 176 && self.pos.x + (self.size.x / 2) < 208 && self.pos.y + (self.size.y / 2) > 58 && self.pos.y + (self.size.y / 2) < 80
        {
            self.wins[1] = true;
            self.reset_position();
        }
        else if !self.wins[2] && self.pos.x + (self.size.x / 2) > 304 && self.pos.x + (self.size.x / 2) < 336 && self.pos.y + (self.size.y / 2) > 58 && self.pos.y + (self.size.y / 2) < 80
        {
            self.wins[2] = true;
            self.reset_position();
        }
        else if !self.wins[3] && self.pos.x + (self.size.x / 2) > 432 && self.pos.x + (self.size.x / 2) < 464 && self.pos.y + (self.size.y / 2) > 58 && self.pos.y + (self.size.y / 2) < 80
        {
            self.wins[3] = true;
            self.reset_position();
        }
        else if !self.wins[4] && self.pos.x + (self.size.x / 2) > 560 && self.pos.x + (self.size.x / 2) < 592 && self.pos.y + (self.size.y / 2) > 58 && self.pos.y + (self.size.y / 2) < 80
        {
            self.wins[4] = true;
            self.reset_position();
        }

        if current_keys.contains(&"ArrowUp") && !previous_keys.contains(&"ArrowUp")
        {
            // Change sprite
            self.size = pt(24, 18);
            self.sprite = pt(68, 6);
            // Update the step to be done
            self.step.y = - self.speed;
        }
        else if current_keys.contains(&"ArrowDown") && !previous_keys.contains(&"ArrowDown")
        {
            // Change sprite
            self.size = pt(24, 18);
            self.sprite = pt(98, 40);
            // Update the step to be done
            self.step.y = self.speed;
        }

        if current_keys.contains(&"ArrowLeft") && !previous_keys.contains(&"ArrowLeft")
        {
            // Change sprite
            self.size = pt(18, 24);
            self.sprite = pt(166, 4);
            // Update the step to be done
            self.step.x = -self.speed;
        }
        else if current_keys.contains(&"ArrowRight") && !previous_keys.contains(&"ArrowRight")
        {
            // Change sprite
            self.size = pt(18, 24);
            self.sprite = pt(6, 36);
            // Update the step to be done
            self.step.x = self.speed;
        }

        self.pos = self.pos + self.step;

        self.pos.x = min(max(self.pos.x, 0), scr.x - self.size.x);  // clamp the x-val to arena dimension
        self.pos.y = min(max(self.pos.y, 0), scr.y - self.size.y);  // clamp the y-val to arena dimension
        self.blinking = max(self.blinking - 1, 0);
    }

    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn alive(&self) -> bool { self.lives > 0 }
    fn as_any(&self) -> &dyn Any { self }
    
    fn sprite(&self) -> Option<Pt> 
    { 
        if self.blinking > 0 && (self.blinking / 2) % 2 == 0 { None }
        else { Some(self.sprite) }
    }
}

pub struct FroggerGame 
{
    arena: Arena,
    playtime: i32,
}
impl FroggerGame 
{
    pub fn new(size: Pt, n_vehicle_per_row: i32, n_raft_per_row: i32) -> FroggerGame 
    {
        let mut arena = Arena::new(size);

        arena.spawn(Box::new(Water::new(pt(0, 82))));

        for i in 0..n_vehicle_per_row
        {
            arena.spawn(Box::new(Vehicle::new(pt(i * 150 + randint(10, 50), 276), 4)));  // First row
            arena.spawn(Box::new(Vehicle::new(pt(i * 150 + randint(10, 50), 308), -4)));  // Second row
            arena.spawn(Box::new(Vehicle::new(pt(i * 150 + randint(10, 50), 340), 4)));  // Third row
            arena.spawn(Box::new(Vehicle::new(pt(i * 150 + randint(10, 50), 372), -4)));  // Fourth row
            arena.spawn(Box::new(Vehicle::new(pt(i * 150 + randint(10, 50), 404), 4)));  // Fifth row
        }

        for i in 0..n_raft_per_row
        {
            arena.spawn(Box::new(Raft::new(pt(i * 200 + randint(50, 200), 87), 2)));  // First row
            arena.spawn(Box::new(Raft::new(pt(i * 200 + randint(50, 200), 119), -2)));  // Second row
            arena.spawn(Box::new(Raft::new(pt(i * 200 + randint(50, 200), 151), 2)));  // Third row
            arena.spawn(Box::new(Raft::new(pt(i * 200 + randint(50, 200), 183), -2)));  // Fourth row
            arena.spawn(Box::new(Raft::new(pt(i * 200 + randint(50, 200), 215), 2)));  // Fifth row
        }

        arena.spawn(Box::new(Frog::new(pt(308, 440))));

        FroggerGame{arena: arena, playtime: 0}
    }

    pub fn game_over(&self) -> bool { self.remaining_lives() <= 0 }

    pub fn game_won(&self) -> bool
    {
        let mut game_won: bool = false;

        for actor in self.actors()
        {
            if let Some(hero) = actor.as_any().downcast_ref::<Frog>()
            {
                game_won = if hero.wins.contains(&false) { false } else { true };
                break;
            }
        }

        game_won
    }

    pub fn playing_time(&self) -> i32
    {
        self.playtime + self.arena.count() / 30
    }

    pub fn remaining_lives(&self) -> i32
    {
        let mut lives: i32 = 0;

        for actor in self.actors()
        {
            if let Some(hero) = actor.as_any().downcast_ref::<Frog>() 
            {
                lives = hero.lives;
                break;
            }
        }
        lives
    }

    pub fn tick(&mut self, keys: String) { self.arena.tick(keys); }
    pub fn size(&self) -> Pt { self.arena.size() }
    pub fn actors(&self) -> &Vec<Box<dyn Actor>> { self.arena.actors() }
}
