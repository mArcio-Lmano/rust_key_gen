use crate::database::{self, PassSave, Database};

use druid::text::{FontDescriptor, FontFamily, FontWeight, TextAlignment};
use druid::widget::{Flex, Label, Button, ListIter, Scroll, List, Split, Container, SizedBox, ZStack, TextBox, CrossAxisAlignment, Align, Padding, MainAxisAlignment};
use druid::{Widget, Data, Lens, Env, Color, WidgetExt, Vec2, UnitPoint};

const TEXT_BOX_WIDTH: f64 = 500.0;

#[derive(Clone)]
pub struct AppState {
    // pub database: Database,
    // pub pass_saves: &Vec<PassSave>,
    pub site_label: String,
    pub user_label: String,
    pub password_label: String,
    pub name: String
    // pub passsave: database::PassSave,
}

impl AppState {
    pub fn new(database: &Database) -> Self {
        AppState { 
            // pass_saves: database.passsaves,
            site_label: "".to_string(),
            user_label: "".to_string(),
            password_label: "".to_string(),
            // passsave: database::PassSave::new(),
            name: "teste".to_string(),
        }
    }
}

// impl Clone for AppState {
//     fn clone(&self) -> Self {
//         // Manually clone the non-cloneable field 'database'
//         let cloned_database = self.database;

//         // Clone the rest of the fields
//         let cloned_site_label = self.site_label.clone();
//         let cloned_user_label = self.user_label.clone();
//         let cloned_password_label = self.password_label.clone();
//         let cloned_name = self.name.clone();

//         // Construct the cloned AppState
//         AppState {
//             database: cloned_database,
//             site_label: cloned_site_label,
//             user_label: cloned_user_label,
//             password_label: cloned_password_label,
//             name: cloned_name,
//         }
//     }
// }

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        // self.database == other.database && 
        self.site_label == other.site_label &&
        self.user_label == other.user_label &&
        self.password_label == other.password_label
    }
}


pub fn build_ui(pass_saves_fn: Vec<PassSave>) -> impl Widget<AppState> {
    // Create buttons for each PassSave entry
    let mut col_sites = Flex::column();
    
    let site_info_box = TextBox::new()
        .with_placeholder("Site's Name")
        .with_text_size(40.0)
        .with_text_alignment(TextAlignment::Center)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AppState::site_label);

    let user_info_box = TextBox::new()
        .with_placeholder("User's Name")
        .with_text_size(20.0)
        .with_text_alignment(TextAlignment::Center)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AppState::user_label);

    let password_info_box = TextBox::new()
        .with_placeholder("User's Password")
        .with_text_size(20.0)
        .with_text_alignment(TextAlignment::Center)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AppState::password_label);

    for pass_save in pass_saves_fn {
        let site_button = Button::new(pass_save.site.clone())
            .on_click(move | _ctx, data: &mut AppState, _env| {
                // Update the label with the selected site's information
                let site_text = format!("{}", pass_save.site);
                let user_text = format!("{}", pass_save.user);
                let password_text = format!("{}", pass_save.pass);

                data.site_label = site_text;
                data.user_label = user_text;
                data.password_label = password_text;
                _ctx.request_update();
            });

<<<<<<< HEAD
            col_sites = col_sites.with_child(Padding::new(5.0, site_button));
    }
    let plus_btn = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env|{

            let new_entery = PassSave::new();
            println!("+")

        })    
        .fix_width(1000.0)
        .fix_height(1000.0);
    let minus_btn = Button::new("-")
        .fix_width(1000.0)
        .fix_height(1000.0);

    // Create Flex container and add buttons
    let mut control_btns = Flex::row()
        .with_child(plus_btn)
        .with_child(minus_btn).center();
    // control_btns.add_child(plus_btn);
    // control_btns.add_child(minus_btn);
    // control_btns = control_btns.add_child(minus_btn);
=======
            col_sites = col_sites.with_child(Padding::new(5.0, site_button.center().align_horizontal(UnitPoint::CENTER)));
    }
>>>>>>> 3d4f0d223de0227d153c8e015b2460bcd92f833f

    let info_labels = Container::new(Flex::column()
        .with_child(site_info_box)
        .with_child(user_info_box)
        .with_child(password_info_box)
    ).center();

    let sites_buttons = col_sites.center();
    // Create a split layout with scrolling for the buttons and a column with the label
<<<<<<< HEAD
    let split_database = Split::columns(
=======
    let split = Split::columns(
>>>>>>> 3d4f0d223de0227d153c8e015b2460bcd92f833f
        Scroll::new(sites_buttons),
        info_labels
    ).draggable(true)
        .split_point(0.2);
    
    let split = Split::rows(split_database, control_btns).split_point(0.95);
    split
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