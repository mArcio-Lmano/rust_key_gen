use crate::database::pass_save_derived_lenses::pass;
use crate::database::{self, PassSave};

use druid::widget::prelude::*;
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
        self.passsaves == other.passsaves && self.counter == other.counter
    }
}


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