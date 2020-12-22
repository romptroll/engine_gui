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

use engine_core::{window::{Key}};

pub trait Bounds {
    fn bounds(&self) -> (f32, f32, f32, f32);
}

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub pressed: bool,
}

impl Button {
    
}

impl Bounds for Button {
    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

pub struct CheckBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub pressed: bool,
}

impl Bounds for CheckBox {
    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

pub struct Slider {
    pub x: f32,
    pub y: f32,
    pub width: f32, 
    pub height: f32, 
    pub val: f32, 
    pub selected: bool,
}

impl Bounds for Slider {
    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

pub struct TextBox {
    pub x: f32,
    pub y: f32, 
    pub width: f32, 
    pub height: f32, 
    pub text: String, 
    pub selected: bool, 
    pub keys: Vec<Key> //TODO change keys to be an action key pair. Reason: it opens more possibilities to do actions depending on if a key is presse, released or hold down
}

impl Bounds for TextBox {
    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}