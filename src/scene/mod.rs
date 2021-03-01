use std::vec::Vec;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::*;

/* local modules */
pub mod object;
use object::Object2d;


pub struct Scene2d {
    canvas: CanvasRenderingContext2d,
    objects: Vec<Object2d>,
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
            objects: vec![Object2d::NullObj()],
        };
    }

    pub fn render(&self) {
        for o in self.objects.iter() {
            o.render(self.canvas);
        }
    }

    pub fn add_object(&mut self,
                      obj: Object2d)
    {
        self.objects.push(obj);
    }

    pub fn get_object_by_id(&self,
                            id: String)
    -> Object2d {
        for o in self.objects {
            if o.get_id() == id {
                return o;
            }
        }

        return Object2d::NullObj();
    }
}