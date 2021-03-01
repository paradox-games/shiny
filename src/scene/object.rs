use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::*;


pub struct Object2d {
    sprite: SvgImageElement,   //images in SVG
    id: String,
    x: f64,
    y: f64,
}

impl Object2d {
    pub fn init(sprite_id: &str,
                id: String,
                x: f64,
                y: f64)
    {
        return Object2d{
            sprite: window().unwrap().document().unwrap().get_element_by_id(sprite_id).unwrap(),
            id: id,
            x: x,
            y: y,
        };
    }

    pub fn render(&self,
                  ctx: CanvasRenderingContext2d)
    {
        ctx.draw_image_with_svg_image_element(
            self.sprite,
            self.x,
            self.y,
        );
    }

    pub fn go_to(&mut self,
                 x: f64,
                 y: f64)
    {
        self.x = x;
        self.y = y;
    }
}