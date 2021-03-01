use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::*;

/* local modules */
pub mod object;
use object::Object2d;


pub struct Scene2d {
    canvas: CanvasRenderingContext2d,
    objects: &[Object2d],
}

impl Scene2d {
    pub fn init(canvas_id: &str) -> Scene2d {
        let document = window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
    
        return Scene2d {
            canvas: canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap(),
            objects: [Object2d::NullObj()],
        }
    }

    pub fn render(&self) {
        for o in self.objects {
            o.render(self.canvas);
        }
    }
}