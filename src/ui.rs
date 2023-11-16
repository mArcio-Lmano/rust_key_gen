use std::fmt::format;
use std::rc::Rc;

use crate::database::pass_save_derived_lenses::pass;
use crate::database::{self, PassSave};


use druid::text::{FontDescriptor, FontFamily, FontWeight};
use druid::widget::{Flex, Label, Button, ListIter, Scroll, List, Split, Container, SizedBox, ZStack};
use druid::{Widget, Data, Lens, Env, Color, WidgetExt, Vec2, UnitPoint};


#[derive(Clone, Lens)]
pub struct AppState {
    pub passsaves: Vec<database::PassSave>,
    pub counter: u32,
    pub site_info: String,
    pub passsave: PassSave,
}

impl AppState {
    pub fn new(passsaves: Vec<database::PassSave>) -> Self {
        AppState { 
            passsaves,
            counter: 0,
            site_info: "Teste".to_string(),
            passsave: database::PassSave::new()
        }
    }
}


impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.passsaves == other.passsaves
    }
}



// pub fn build_ui(pass_saves_fn: Vec<PassSave>) -> impl Widget<AppState> {
//     // Create buttons for each PassSave entry
//     let mut col_sites = Flex::column();

//     // Create a label to display the selected site's information
//     // let site_info_label = Label::new("Arroz");
 
//     let site_info_label: Label<AppState> = Label::dynamic(|state: &AppState, _| {
//         format!(
//             "Information: {}",
//             state.site_info
//         )
//     });

//     for pass_save in pass_saves_fn {
//         let site_button = Button::new(pass_save.site.clone())
//             .on_click(move | _ctx, data: &mut AppState, _env| {
//                 // Update the label with the selected site's information
//                 let info_text = format!("User: {}, Pass: {}", pass_save.user, pass_save.pass);
//                 data.site_info = info_text;
//                 _ctx.request_update();
//             });

//         col_sites.add_child(site_button);
//     }

//     // Create a split layout with scrolling for the buttons and a column with the label
//     let split = Split::columns(
//         Scroll::new(col_sites),
//         Container::new(
//             Flex::column()
//                 .with_child(site_info_label)
//         ),
//     );
//     split
// }

pub fn build_ui(pass_saves_fn: Vec<PassSave>) -> impl Widget<AppState> {
    ZStack::new(
        Button::from_label(Label::dynamic(|state: &AppState, _| {
            format!(
                "Very large button with text! Count up (currently {})",
                state.counter
            )
        }))
        .on_click(|_, state: &mut AppState, _| state.counter += 1),
    )
    .with_child(
        Button::new("Reset").on_click(|_, state: &mut AppState, _| state.counter = 0),
        Vec2::new(1.0, 1.0),
        Vec2::ZERO,
        UnitPoint::LEFT,
        Vec2::new(10.0, 0.0),
    )
}
// pub fn build_ui(pass_saves_fn: Vec<PassSave>) -> impl Widget<AppState> {
//     // Create buttons for each PassSave entry
//     let mut col_sites = Flex::column();

//     // Create a label to display the selected site's information
//     let site_info_label = Label::new(|data: &AppState, _env: &Env| {
//         if data.site_info.is_empty() {
//             "Hello anybody!?".to_string()
//         } else {
//             format!("Hello {}!", data.site_info)
//         }
//     }).with_text_size(32.0);


//     for pass_save in pass_saves_fn {
//         let site_button = Button::new(pass_save.site.clone())
//             .on_click(move | ctx, data: &mut AppState, _env| {
//                 // Update the label with the selected site's information
//                 let info_text = format!("User: {}, Pass: {}", pass_save.user, pass_save.pass);
//                 data.site_info = "".to_string();
//                 println!("{}", info_text);
//                 // Manually trigger an update
//                 ctx.request_update();
//             }).center();

//         col_sites.add_child(site_button);
//     }
//     // Create a split layout with scrolling for the buttons and a column with the label
//     let split = Split::columns(
//         Scroll::new(col_sites),
//         Container::new(Flex::column().with_child(site_info_label)),
//     );
//     split
// }


// pub fn main_window<T: 'static + Data, W: Widget<T> + 'static>() -> WindowDesc<T> {
//     WindowDesc::new(|| W::new().boxed())
// }