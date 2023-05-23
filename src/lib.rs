use wasm_bindgen::prelude::*;
use std::cell::RefCell;

pub mod actor;
pub mod frogger;
pub mod g2d;
pub mod pt2d;
pub mod rand;

pub struct FroggerGui 
{
    game: frogger::FroggerGame
}
impl FroggerGui 
{
    pub fn new() -> FroggerGui 
    {
        let game = frogger::FroggerGame::new(pt2d::pt(640, 480), 5, 2);
        FroggerGui{game}
    }

    pub fn setup(&self) 
    {
        g2d::init_canvas(self.game.size());
        g2d::main_loop(30);
    }

    pub fn tick(&mut self) 
    {
        g2d::clear_canvas();
        g2d::draw_image_clip("frogger-bg.png".to_string(), pt2d::pt(0, 0), pt2d::pt(0, 0), pt2d::pt(640, 480));
        
        for b in self.game.actors() 
        {
            if let Some(img) = b.sprite() 
            {
                g2d::draw_image_clip("frogger.png".to_string(), b.pos(), img, b.size());
            }

            if let Some(hero) = b.as_any().downcast_ref::<frogger::Frog>() 
            {
                let iter = hero.get_wins().iter();

                iter.fold(0, 
                          |idx, val| if *val == true
                                                    {
                                                        if idx == 0 { g2d::draw_image_clip("frogger.png".to_string(), pt2d::pt(52, 54), hero.get_win_sprite(), hero.get_win_sprite_size()); }
                                                        if idx == 1 { g2d::draw_image_clip("frogger.png".to_string(), pt2d::pt(180, 54), hero.get_win_sprite(), hero.get_win_sprite_size()); }
                                                        if idx == 2 { g2d::draw_image_clip("frogger.png".to_string(), pt2d::pt(308, 54), hero.get_win_sprite(), hero.get_win_sprite_size()); }
                                                        if idx == 3 { g2d::draw_image_clip("frogger.png".to_string(), pt2d::pt(436, 54), hero.get_win_sprite(), hero.get_win_sprite_size()); }
                                                        if idx == 4 { g2d::draw_image_clip("frogger.png".to_string(), pt2d::pt(564, 54), hero.get_win_sprite(), hero.get_win_sprite_size()); }
                                                        idx + 1
                                                    }
                                                    else { idx + 1 }
                         );
            }
        }

        let txt = format!("Lives: {} Time: {}", self.game.remaining_lives(), self.game.playing_time());
        g2d::set_color(255, 0, 0);
        g2d::draw_text(txt, pt2d::pt(0, 0), 24);

        if self.game.game_over() 
        {
            g2d::alert("Game over".to_string());
            g2d::close_canvas();
        }
        else if self.game.game_won()
        {
            g2d::alert(format!("You won! Remaining lives: {} - Elapsed time: {}", self.game.remaining_lives(), self.game.playing_time()));
            g2d::close_canvas();
        }
        else 
        {
            self.game.tick(g2d::current_keys());  // Game logic
        }
    }
}

thread_local! 
{
    static GUI: RefCell<FroggerGui> = RefCell::new(FroggerGui::new());
}

#[wasm_bindgen]
pub fn tick() 
{
    GUI.with(|g| {
        g.borrow_mut().tick();
    });
}

#[wasm_bindgen]
pub fn setup() {
    GUI.with(|g| {
        g.borrow_mut().setup();
    });
}
