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

#[macro_use]
extern crate engine_core;

pub mod gui;
pub mod comps;

#[cfg(test)]
mod tests {
    use engine_core::window::{Window, Key};
    use engine_renderer::{font::Font, renderer::*, texture::{Texture, TextureRegion}};
    use engine_renderer::renderer::std_renderer::*;
    use engine_renderer::color::Color;
    use crate::comps::{Button, CheckBox, Slider, TextBox};
    use crate::gui;

    #[test]
    fn window() {
        let mut win  = Window::new(600, 400, "").unwrap();
        win.make_current();
        init_gl(&mut win);
        unsafe {
            enable(Capability::Blending);
            blend_func(BlendMode::SrcAlpha, BlendMode::OneMinusSrcAlpha);
        }
        let mut gui = gui::GUI::new(&mut win);

        let mut but_1 = Button {
            x: 0.0,
            y: 0.0,
            width: 0.5,
            height: 0.5,
            text: String::from("press 1"),
            pressed: false,
        };

        let mut but_2 = Button {
            x: 0.0,
            y: -0.6,
            width: 0.5,
            height: 0.5,
            text: String::from("press 2"),
            pressed: false,
        };

        let mut sli_1 = Slider {
            x: -0.6,
            y: 0.0,
            width: 0.5,
            height: 0.05,
            val: 0.0,
            selected: false,
        };

        let mut text_box = TextBox {
            x: -0.6,
            y: -0.6,
            width: 0.5,
            height: 0.5,
            text: String::new(),
            selected: false,
        };

        let mut check_box = CheckBox {
            x: -0.6,
            y: 0.5,
            width: 0.5,
            height: 0.5,
            pressed: false,
        };

        let texture = Texture::from_file("res/textures/tile_sheet.png");

        gui.graphics.set_font(Font::new("res/fonts/arial.ttf", 64));

        gui.style.foreground_texture = TextureRegion::new(0, texture.height()-16, 16, 16, &texture);
        gui.style.background_texture = TextureRegion::new(16, texture.height()-16, 16, 16, &texture);

        gui.style.text_align = gui::TextAlign::Center;

        while !win.should_close() {
            gui.clear();
            
            gui.button(&mut but_1);
            gui.button(&mut but_2);
            
            gui.slider(&mut sli_1);

            gui.check_box(&mut check_box);

            if but_1.pressed {
                println!("button 1 {}", sli_1.val);
                let (_, g, _, _) = <(u8, u8, u8, u8)>::from(gui.style.text_color);
                gui.style.text_color = Color::from(((sli_1.val * 255.0) as u8, g, 0, 255));
            }

            if but_2.pressed {
                println!("button 2");
                let (r, _, _, _) = <(u8, u8, u8, u8)>::from(gui.style.text_color);
                gui.style.text_color = Color::from((r, (sli_1.val * 255.0) as u8, 0, 255));
            }

            gui.text_box(&mut text_box);

            gui.update();
            win.poll_events();
            win.swap_buffers();
        }
    }

    #[test]
    fn scaled() {
        let mut win  = Window::new(600, 400, "").unwrap();
        win.make_current();
        init_gl(&mut win);
        unsafe {
            enable(Capability::Blending);
            blend_func(BlendMode::SrcAlpha, BlendMode::OneMinusSrcAlpha);
        }
        let mut gui = gui::GUI::new(&mut win);

        let mut but_1 = Button {
            x: 0.0,
            y: 0.0,
            width: 128.0,
            height: 64.0,
            text: String::from("press 1"),
            pressed: false,
        };

        let mut but_2 = Button {
            x: 0.0,
            y: 128.0,
            width: 128.0,
            height: 64.0,
            text: String::from("press 2"),
            pressed: false,
        };

        let mut text_box = TextBox {
            x: 140.0,
            y: 128.0,
            width: 128.0,
            height: 64.0,
            text: String::new(),
            selected: false,
        };
        
        let texture = Texture::from_file("res/textures/tile_sheet.png");

        gui.graphics.set_font(Font::new("res/fonts/arial.ttf", 64));

        gui.style.foreground_texture = TextureRegion::new(0, texture.height()-16, 16, 16, &texture);
        gui.style.background_texture = TextureRegion::new(16, texture.height()-16, 16, 16, &texture);

        gui.style.text_align = gui::TextAlign::Center;

        while !win.should_close() {
            gui.clear();

            gui.graphics.set_translation(-1.0, -1.0);
            gui.graphics.set_scale(2.0 / win.get_width() as f32, 2.0 / win.get_height() as f32);
            
            gui.button(&mut but_1);
            gui.button(&mut but_2);

            gui.text_box(&mut text_box);
            
            gui.update();
            win.poll_events();
            win.swap_buffers();
        }
    }
}