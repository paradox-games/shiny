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
        -> Object2d {
        return Object2d {
            sprite: window().unwrap()
                    .document().unwrap()
                    .get_element_by_id(sprite_id).unwrap()
                    .dyn_into::<web_sys::SvgImageElement>().unwrap(),
            id: id,
            x: x,
            y: y,
        };
    }

    pub fn NullObj() -> Object2d {
        return Object2d {
            sprite: None::<SvgImageElement>.unwrap().
                        dyn_into::<web_sys::SvgImageElement>().unwrap(),
            id: String::new(),
            x: 0.0,
            y: 0.0,
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

    pub fn set_sprite(&mut self,
                      sprite_id: &str)
    {
        self.sprite = window().unwrap().document().unwrap().get_element_by_id(sprite_id).unwrap();
    }

    pub fn get_id(&self) -> String {
        return self.id;
    }
}