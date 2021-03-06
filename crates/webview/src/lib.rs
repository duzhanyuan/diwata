//#![deny(warnings)]
#![deny(clippy::all)]
use app::{App, Msg};
use diwata_intel::{
    field::ColumnDetail,
    widget::{Alignment, ControlWidget, Widget},
    window::{GroupedWindow, WindowName},
    ColumnName, Field, IndirectTab, SqlType, Tab, TableName, Window,
};
use sauron::{Dispatch, Program};
use std::rc::Rc;
use wasm_bindgen::{self, prelude::*, JsCast, JsValue};
use web_sys::Response;

mod app;
mod assets;
mod data;
mod rest_api;
mod widgets;

pub fn setup_program(initial_state: &str) -> Rc<Program<App, Msg>> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    sauron::log!("initial state: {}", initial_state);
    let root_node = sauron::document()
        .get_element_by_id("web-app")
        .expect("Unable to get hold of root-node");
    let app = make_app();
    Program::new_replace_mount(app, &root_node)
}

pub fn make_app() -> App {
    let windows: Vec<Window> = vec![
        sample_window("Window1"),
        sample_window("Window2"),
        sample_window("Window3"),
    ];
    let window_list: Vec<GroupedWindow> = make_sample_window_list();
    let (window_width, window_height) = get_window_size();
    let mut app = App::new(window_list, windows, window_width, window_height);
    app.set_window_data(0, crate::data::make_sample_window_data());
    app
}

fn make_sample_window_list() -> Vec<GroupedWindow> {
    vec![make_group_window("Group1"), make_group_window("Group2")]
}

fn make_group_window(group: &str) -> GroupedWindow {
    GroupedWindow {
        group: group.to_string(),
        window_names: make_window_names(),
    }
}

fn make_window_names() -> Vec<WindowName> {
    (0..40)
        .map(|n| make_window_name(&format!("Window{}", n)))
        .collect()
}

fn make_window_name(name: &str) -> WindowName {
    WindowName {
        name: name.to_string(),
        table_name: TableName::from(name),
        is_view: false,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_window_size() -> (i32, i32) {
    (800, 800)
}

#[cfg(target_arch = "wasm32")]
fn get_window_size() -> (i32, i32) {
    let window = sauron::window();
    let window_width = window
        .inner_width()
        .expect("unable to get window width")
        .as_f64()
        .expect("cant convert to f64");
    let window_height = window
        .inner_height()
        .expect("unable to get height")
        .as_f64()
        .expect("cant convert to f64");
    (window_width as i32, window_height as i32)
}

fn sample_window(name: &str) -> Window {
    Window {
        name: name.to_string(),
        description: None,
        group: None,
        main_tab: sample_tab(&format!("Main tab of {}", name)),
        has_one_tabs: vec![sample_tab("Has one 1"), sample_tab("Has one 2")],
        one_one_tabs: vec![sample_tab("One one 1"), sample_tab("One one 2")],
        has_many_tabs: vec![sample_tab("Has many 1"), sample_tab("Has many 2")],
        indirect_tabs: vec![
            IndirectTab::new(TableName::from("bazaar.table1"), sample_tab("Indirect 1")),
            IndirectTab::new(TableName::from("bazaar.table2"), sample_tab("Indirect 2")),
        ],
        is_view: false,
    }
}

fn sample_tab(name: &str) -> Tab {
    Tab {
        name: name.to_string(),
        description: None,
        table_name: TableName::from("bazaar.product"),
        fields: (0..10)
            .map(|n| sample_field(&format!("Field {}", n)))
            .collect(),
        is_view: false,
        display: None,
    }
}

fn sample_field(name: &str) -> Field {
    Field {
        name: name.to_string(),
        description: None,
        info: None,
        is_primary: false,
        column_detail: sample_column_detail(name),
        control_widget: sample_control_widget(name),
    }
}

fn sample_column_detail(name: &str) -> ColumnDetail {
    ColumnDetail::Simple(ColumnName::from(name), SqlType::Text)
}

fn sample_control_widget(_name: &str) -> ControlWidget {
    ControlWidget {
        widget: Widget::Textbox,
        dropdown: None,
        width: 100,
        max_len: Some(100),
        height: 20,
        alignment: Alignment::Left,
    }
}
