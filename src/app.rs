use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("greet", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>

            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <p>
                {"Recommended EDI setup: "}
                <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                {" + "}
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                {" + "}
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
            </p>

            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>

            <p><b>{ &*greet_msg }</b></p>
        </main>
    }
}



// -------- The program itself (parts of it, the whole doesnt work, many tries for graphics libraries were kept, each having its own features)
// -------- ==> Tauri seems way more complete for this frontend goal





// -------- Piston (/ egui) attempt


// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// /**
// Finds simplified Mandarin words containing a given radical.
//  */

// // use sqlite;
// // use std::io::stdin;
// extern crate graphics;
// extern crate opengl_graphics;
// extern crate piston;

// // use piston::event_input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
// use glutin_window::GlutinWindow as Window;
// use opengl_graphics::{GlGraphics, OpenGL};
// use piston::event_loop::{EventSettings, Events};
// use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
// use piston::window::WindowSettings;


// pub struct PistonApp {
//     gl: GlGraphics,
// }

// impl PistonApp {
//     fn render(&mut self, args: &RenderArgs) {
//         use graphics::*;

//         // Draw frame

//         const BLACKTEAL: [f32; 4] = [44.0/255.0, 51.0/255.0, 51.0/255.0, 1.0];
//         const DARKTEAL: [f32; 4] = [46.0/255.0, 79.0/255.0, 79.0/255.0, 1.0];
//         const TEAL: [f32; 4] = [14.0/255.0, 131.0/255.0, 136.0/255.0, 1.0];
//         const LIGHTTEAL: [f32; 4] = [203.0/255.0, 228.0/255.0, 222.0/255.0, 1.0];
//         let (width, height) = (args.window_size[0], args.window_size[1]);
//         let scale_border: [f64; 2] = [width/300.0, height/300.0];

//         let borders = vec![
//             [0.6*width, height/2.0, 3.0+scale_border[0], height/2.0],
//             [0.3*width, height*0.6, 0.3*width, 3.0+scale_border[1]],
//             [0.8*width, height*0.8, 0.2*width, 3.0+scale_border[1]],
//         ];

//         self.gl.draw(args.viewport(), |c, gl| {
//             clear(TEAL, gl);
//             for rect in borders.iter(){
//                 rectangle(LIGHTTEAL, rectangle::centered(*rect), c.transform, gl);
//             }
//         });
//     }
    
//     fn update(&mut self, args: &UpdateArgs) {

//     }
// }

// fn main() {
//     // Setup
//     // https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started
//     // https://docs.rs/pistoncore-glutin_window/0.71.0/glutin_window/struct.GlutinWindow.html#impl-AdvancedWindow-for-GlutinWindow
    
//     let opengl = OpenGL::V3_2;
 
//      // Create a Glutin window.
//     let mut window: Window = WindowSettings::new("spinning-square", [800, 600])
//         .graphics_api(opengl)
//         .exit_on_esc(true)
//         .build()
//         .unwrap();

//     // Create a new game and run it.
//     let mut piston_app = PistonApp {
//         gl: GlGraphics::new(opengl),
//     };

//     let mut events = Events::new(EventSettings::new());
//     while let Some(e) = events.next(&mut window) {
//         if let Some(args) = e.render_args() {
//             piston_app.render(&args);
//         }

//         if let Some(args) = e.update_args() {
//             piston_app.update(&args);
//         }
//     }
// }


// -------- Winit front

// // let win_size = LogicalSize{
// //     width: 1000,
// //     height: 1000,
// // };
// // let win_min_size = LogicalSize{
// //     width: 400,
// //     height: 400,
// // };
// // let win_logical_pos = LogicalPosition{
// //     x: 460,
// //     y: 20,
// // };
// // let window: PistonWindow<Window> =
// // WindowSettings::new("title", [512; 2])
// //     .build().unwrap();
// // // Launch

// // let event_loop = EventLoop::new();

// // event_loop.run(move |event, _, control_flow| {
// //     *control_flow = ControlFlow::Wait;

// //     match event {
// //         Event::WindowEvent {
// //             event: WindowEvent::CloseRequested,
// //             window_id,
// //         } if window_id == window.id() => *control_flow = ControlFlow::Exit,
// //         _ => (),
// //     }
// // });



// //     let window = WindowBuilder::new()
// //         .with_min_inner_size(win_min_size)
// //         .with_inner_size(win_size)
// //         .with_maximized(true)
// //         .with_position(win_logical_pos)
// //         .with_fullscreen(None)
// //         .with_resizable(true)
// //         .with_visible(true)
// //         .with_transparent(false)
// //         .with_decorations(true)
// //         .with_enabled_buttons(WindowButtons::all())
// //         .with_window_level(WindowLevel::Normal)
// //         .with_content_protected(false)
// //         .with_active(true)
// //         .with_title("XinHua ZhaoDao")
// //         .with_window_icon(None)
// //         .build(&event_loop).unwrap();





// -------- Piston with Glutin backend

// // fn main() {
// //     // Change this to OpenGL::V2_1 if not working.
// //     let opengl = OpenGL::V3_2;
// // 
// //     // Create a Glutin window.
// //     let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
// //         .graphics_api(opengl)
// //         .exit_on_esc(true)
// //         .build()
// //         .unwrap();
// // 
// //     // Create a new game and run it.
// //     let mut app = App {
// //         gl: GlGraphics::new(opengl),
// //         rotation: 0.0,
// //     };
// // 
// //     let mut events = Events::new(EventSettings::new());
// //     while let Some(e) = events.next(&mut window) {
// //         if let Some(args) = e.render_args() {
// //             app.render(&args);
// //         }
// // 
// //         if let Some(args) = e.update_args() {
// //             app.update(&args);
// //         }
// //     }
// // }


// -------- <!!!>
// -------- DB access

// //fn main() -> Result<(), ()>{
// //	let conn = sqlite::open("./data/hanyu.db").unwrap();
// //
// //    let mut component = String::new();
// //    stdin().read_line(&mut component).unwrap();
// //    let component = component.trim();
// //
// //	let query = format!("{}{}{}{}{}{}{}",
// //	"select word from 
// //	(
// //			select word from zidian 
// //		INTERSECT 
// //			select ch as word from decomp 
// //				where (ch like '%", component, "%' OR p1 like '%", component, "%' OR p2 like '%", component, "%')
// //	) result 
// //	
// //	join decomp on result.word = decomp.ch 
// //
// //	order by s asc");
// //	let mut statement = conn.prepare(query).unwrap();
// //
// //	while let Ok(sqlite::State::Row) = statement.next() {
// //	    println!("- {}", statement.read::<String, _>("word").unwrap());
// //	}
// //
// //	Ok(())
// //}
