use crate::{assets, widgets};
use diwata_intel::window::GroupedWindow;
use sauron::{
    html::{attributes::*, events::*, *},
    Cmd, Component, Node,
};

#[derive(Debug, Clone)]
pub enum Msg {
    ChangeSearch(String),
    ReceiveWindowList(Vec<GroupedWindow>),
}

pub struct WindowListView {
    window_list: Vec<GroupedWindow>,
    allocated_width: i32,
    allocated_height: i32,
}

impl WindowListView {
    pub fn new(window_list: Vec<GroupedWindow>) -> Self {
        WindowListView {
            window_list,
            allocated_width: 0,
            allocated_height: 0,
        }
    }

    fn calculate_window_list_height(&self) -> i32 {
        self.allocated_height - 20
    }

    fn calculate_window_list_width(&self) -> i32 {
        self.allocated_width
    }

    pub fn set_allocated_size(&mut self, (width, height): (i32, i32)) {
        self.allocated_width = width;
        self.allocated_height = height;
    }
}

impl Component<Msg> for WindowListView {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ChangeSearch(search) => {
                sauron::log!("searching for {}", search);
            }
            Msg::ReceiveWindowList(window_list) => {
                self.window_list = window_list;
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        div(
            [],
            [
                section(
                    [class("window_list_search")],
                    [widgets::search_widget(oninput(|input| {
                        Msg::ChangeSearch(input.value)
                    }))],
                ),
                section(
                    [
                        class("window_list"),
                        styles([
                            ("height", px(self.calculate_window_list_height())),
                            ("width", px(self.calculate_window_list_width())),
                        ]),
                    ],
                    self.window_list
                        .iter()
                        .map(|group| {
                            ul(
                                [],
                                [
                                    li([class("window_list_group_name")], [text(&group.group)]),
                                    ul(
                                        [],
                                        group
                                            .window_names
                                            .iter()
                                            .map(|win_name| {
                                                li(
                                                    [],
                                                    [a(
                                                        [
                                                            href(format!(
                                                                "#{}",
                                                                win_name.table_name.complete_name()
                                                            )),
                                                            class("window_list_link"),
                                                        ],
                                                        [
                                                            span(
                                                                [class("table_icon")],
                                                                [assets::svg_table_icon()],
                                                            ),
                                                            text(&win_name.name),
                                                        ],
                                                    )],
                                                )
                                            })
                                            .collect::<Vec<Node<Msg>>>(),
                                    ),
                                ],
                            )
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
            ],
        )
    }
}
