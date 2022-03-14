use wasm_bindgen::{prelude::*, JsCast};
use web_sys::console;
use Game::LifeGame;

mod Game;

static mut data: Option<Box<LifeGame>> = None;

#[wasm_bindgen]
pub fn init(x: usize, y: usize) {
    unsafe {
        data = Some(Box::new(LifeGame::new(x, y)));
    }
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width((10 * x) as u32);
    canvas.set_height((10 * y) as u32);
}

pub fn game() -> &'static LifeGame {
    unsafe { data.as_ref().unwrap() }
}

pub fn mgame() -> &'static mut LifeGame {
    unsafe { data.as_mut().unwrap() }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub fn click(x: f64, y: f64) {
    let px = (x / 10.0) as usize;
    let py = (y / 10.0) as usize;
    mgame().click(px, py);
    // draw();
}

#[wasm_bindgen]
pub fn step() {
    mgame().step();
    // draw();
}
#[wasm_bindgen]
pub fn draw() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let s = game();

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    let x = s.cell[0].len();
    let y = s.cell.len();
    let w = 0.4;

    context.set_line_width(w);
    context.set_stroke_style(&JsValue::from_str("black"));

    for i in 1..x {
        context.begin_path();
        context.move_to((i * 10) as f64, w / 2.0);
        context.line_to((i * 10) as f64, (y * 10) as f64 + w / 2.0);
        context.close_path();
        context.stroke();
    }
    for j in 1..y {
        context.begin_path();
        context.move_to(w / 2.0, (j * 10) as f64);
        context.line_to((x * 10) as f64 + w / 2.0, (j * 10) as f64);
        context.close_path();
        context.stroke();
    }

    context.set_fill_style(&JsValue::from_str("green"));
    for (i, p) in s.cell.iter().enumerate() {
        for (j, q) in p.iter().enumerate() {
            if q.0 {
                context.fill_rect((i * 10) as f64 + 0.2, (j * 10) as f64 + 0.2, 9.6, 9.6);
            }
            // console::log_1(&JsValue::from_str(
            //     format!("({},{}): {}", i, j, q.1).as_str(),
            // ));
        }
    }
}
