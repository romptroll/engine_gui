/*
 *   Copyright (c) 2020 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

use engine_core::{window::{Window, Key, Mouse, Action}};
use engine_renderer::color::Color;
use engine_renderer::graphics::Graphics;
use engine_renderer::texture::TextureRegion;

use crate::comps::*;

pub enum TextAlign {
    Center,
    LowerLeft,
    LowerRight,
    UpperLeft,
    UpperRight,
}

pub struct GUIStyle {
    pub foreground_color: Color,
    pub background_color: Color,
    pub text_color: Color,

    pub foreground_texture: TextureRegion,
    pub background_texture: TextureRegion,

    pub text_align: TextAlign,

    pub check_box_foreground_color: Color,
    pub check_box_background_color: Color,

    pub check_box_foreground_texture: TextureRegion,
    pub check_box_background_texture: TextureRegion,
}

impl GUIStyle {
    pub fn new() -> GUIStyle {
        GUIStyle {
            foreground_color: Color::from(0x666666FFu32),
            background_color: Color::from(0xAAAAAAFFu32),
            text_color: Color::from(0xFFFFFFFF),

            foreground_texture: TextureRegion::new_invalid(),
            background_texture: TextureRegion::new_invalid(),

            check_box_foreground_color: Color::from(0x666666FFu32),
            check_box_background_color: Color::from(0xAAAAAAFFu32),

            check_box_foreground_texture: TextureRegion::new_invalid(),
            check_box_background_texture: TextureRegion::new_invalid(),

            text_align: TextAlign::LowerLeft,
        }
    }
}

pub struct GUI {
    pub graphics: Graphics,
    pub style: GUIStyle,

    key_actions: Vec<Key>,

    key_listener: bus::BusReader::<(Key, Action)>,

    mouse_move_listener : bus::BusReader::<(f32, f32)>,
    mouse_listener      : bus::BusReader::<(Mouse, Action)>,

    mouse_x: f32,
    mouse_y: f32,

    mouse_is_pressed: bool,
    mouse_just_pressed: bool,
}

impl GUI {
    pub fn new(win: &mut Window) -> GUI {
        GUI {
            graphics: Graphics::new(win),
            style: GUIStyle::new(),

            key_actions: Vec::new(),

            key_listener: win.create_key_listener(),

            mouse_move_listener : win.create_mouse_move_listener(),
            mouse_listener      : win.create_mouse_listener(),

            mouse_x: 0.0,
            mouse_y: 0.0,

            mouse_is_pressed: false,
            mouse_just_pressed: false,
        }
    }

    pub fn button(&mut self, button: &mut Button) {
        if self.mouse_just_pressed && self.mouse_in_rect(button.bounds()) {
            button.pressed = true;
        } else if !self.mouse_is_pressed {
            button.pressed = false;
        }

        if button.pressed {
            self.graphics.set_color(self.style.foreground_color);
            self.graphics.texture(self.style.foreground_texture.clone());
        } else {
            self.graphics.set_color(self.style.background_color);
            self.graphics.texture(self.style.background_texture.clone());
        }

        self.graphics.fill_rect(button.x, button.y, button.width, button.height);

        self.graphics.set_color(self.style.text_color);
        self.draw_text_align(button.x, button.y, button.width, button.height, &button.text);
    }

    pub fn check_box(&mut self, check_box: &mut CheckBox) {
        if self.mouse_just_pressed && self.mouse_in_rect(check_box.bounds()) {
            check_box.pressed = !check_box.pressed;
        } 

        if check_box.pressed {
            self.graphics.set_color(self.style.check_box_foreground_color);
            self.graphics.texture(self.style.check_box_foreground_texture.clone());
        } else {
            self.graphics.set_color(self.style.check_box_background_color);
            self.graphics.texture(self.style.check_box_background_texture.clone());
        }

        self.graphics.fill_rect(check_box.x, check_box.y, check_box.width, check_box.height);
    }

    pub fn slider(&mut self, slider: &mut Slider) {
        let box_w = slider.height*4.0;
        let box_h = slider.height*4.0;
        let box_x = slider.x+(slider.width-box_w)*slider.val;
        let box_y = slider.y-slider.height*2.0+slider.height/2.0;

        if self.mouse_just_pressed && self.mouse_in_rect((box_x, box_y, box_w, box_h)) {
            slider.selected = true;
        } else if !self.mouse_is_pressed {
            slider.selected = false;
        }

        self.graphics.texture(self.style.background_texture.clone());
        self.graphics.set_color(self.style.background_color);
        self.graphics.fill_rect(slider.x, slider.y, slider.width, slider.height);
        
        if slider.selected {
            slider.val = ((self.mouse_x-box_w/2.0).max(slider.x).min(slider.x+slider.width-box_w) - slider.x) / (slider.width - box_w);
            self.graphics.set_color(self.style.foreground_color);
            self.graphics.texture(self.style.foreground_texture.clone());
        } else {
            self.graphics.set_color(self.style.background_color);
            self.graphics.texture(self.style.background_texture.clone());
        }
        
        self.graphics.fill_rect(box_x, box_y, box_w, box_h);
    }

    pub fn text_box(&mut self, text_box: &mut TextBox) {
        if self.mouse_just_pressed && self.mouse_in_rect(text_box.bounds()) { 
            text_box.selected = true;
        } 
        
        if self.mouse_just_pressed && !self.mouse_in_rect(text_box.bounds()) {
            text_box.selected = false;
        }

        if text_box.selected {
            self.graphics.set_color(self.style.foreground_color);
            self.graphics.texture(self.style.foreground_texture.clone());

            for ka in &self.key_actions {
                text_box.keys.push(ka.clone());
            }
        } else {
            self.graphics.set_color(self.style.background_color);
            self.graphics.texture(self.style.background_texture.clone());
        }

        self.graphics.fill_rect(text_box.x, text_box.y, text_box.width, text_box.height);

        self.graphics.set_color(self.style.text_color);

        self.draw_text_align(text_box.x, text_box.y, text_box.width, text_box.height, &text_box.text);
    }

    fn draw_text_align(&mut self, x: f32, y: f32, width: f32, height: f32, text: &str) {
        let mut x = x;
        let mut y = y;

        let total_text_width = self.graphics.font().text_width(text) / self.graphics.frame_width() as f32;
        
        let mut num_chars = text.len();
        let mut text_width = total_text_width;

        while text_width > width {
            num_chars -= 1;
            text_width = self.graphics.font().text_width(&text[..num_chars]) / self.graphics.frame_width() as f32;
        } 

        match self.style.text_align {
            TextAlign::Center => {
                x += width / 2.0 - text_width / 2.0;
                y += height / 2.0 - self.graphics.font().height() as f32 / self.graphics.frame_height() as f32 / 4.0;
            }
            TextAlign::LowerLeft => {
                y += self.graphics.font().height() as f32 / self.graphics.frame_height() as f32;
            }
            TextAlign::LowerRight => {
                x += width - text_width;
                y += height - self.graphics.font().height() as f32 / self.graphics.frame_height() as f32;
            }
            TextAlign::UpperLeft => {
                y += height - self.graphics.font().height() as f32 / self.graphics.frame_height() as f32; //TODO add a function that gets the height of the font y += height - font.text_height() as f32 / self.graphics.frame_height() as f32;
            }
            TextAlign::UpperRight => {
                x += width - text_width;
                y += height - self.graphics.font().height() as f32 / self.graphics.frame_height() as f32;
            }
        }

        self.graphics.draw_string(&text[..num_chars], x, y);
    }

    pub fn clear(&mut self) {
        self.graphics.clear(Color::from((0.0, 0.0, 0.0, 1.0)));
    }

    fn mouse_in_rect(&self, bounds: (f32, f32, f32, f32)) -> bool {
        let x = bounds.0;
        let y = bounds.1;
        let width = bounds.2;
        let height = bounds.3;
        self.mouse_x >= x && self.mouse_x < x + width && 
        self.mouse_y >= y && self.mouse_y < y + height 
    }

    pub fn update(&mut self) {
        //check if the mouse has been moved
        let mut loop_done = false;
        while !loop_done {
            match self.mouse_move_listener.try_recv() {
                Ok((x, y)) => {
                    self.mouse_x = x;
                    self.mouse_y = y;
                },
                Err(_) => loop_done = true
            }
        }


        self.mouse_just_pressed = false;

        //check if any button on the mouse has been pressed or released
        let mut loop_done = false;
        while !loop_done {
            match self.mouse_listener.try_recv() {
                Ok((mouse, action)) => {
                    match mouse {
                        Mouse::Button1 => { //LEFT MOUSE BUTTON
                            match action {
                                Action::Press => {

                                    if !self.mouse_is_pressed {
                                        self.mouse_just_pressed = true;
                                    }

                                    self.mouse_is_pressed = true;
                                },
                                Action::Release => {
                                    self.mouse_is_pressed = false;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                },
                Err(_) => loop_done = true
            }
        }

        //Make sure key pressed from last update is not used again
        self.key_actions.clear();

        //check if any key has been pressed
        let mut loop_done = false;
        while !loop_done {
            match self.key_listener.try_recv() {
                Ok((key, action)) => {
                    match action {
                        Action::Press => {
                            self.key_actions.push(key);
                        }
                        _ => {}
                    }
                },
                Err(_) => loop_done = true
            }
        }
        
        self.graphics.update();
        self.graphics.flush();
    }
}