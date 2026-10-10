#![windows_subsystem = "windows"]

use std::collections::HashMap;

use reactor_settings_controls::*;

#[derive(Clone, Copy)]
struct ExpandedCardInfo {
    name: &'static str,
    info: &'static str,
    link_description: &'static str,
    url: &'static str,
    is_expanded: bool,
}

#[derive(Clone)]
enum Message {
    Expanded(usize, bool),
    Reordered(Vec<String>),
}

struct Sample {
    items: Vec<ExpandedCardInfo>,
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            items: vec![
                ExpandedCardInfo {
                    name: "First Item",
                    info: "More about first item.",
                    link_description: "Click the link for more on first item.",
                    url: "https://microsoft.com/",
                    is_expanded: false,
                },
                ExpandedCardInfo {
                    name: "Second Item",
                    info: "More about second item.",
                    link_description: "Click the link for more on second item.",
                    url: "https://xbox.com/",
                    is_expanded: false,
                },
                ExpandedCardInfo {
                    name: "Third Item",
                    info: "More about third item.",
                    link_description: "Click the link for more on third item.",
                    url: "https://toolkitlabs.dev/",
                    is_expanded: false,
                },
            ],
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Expanded(index, value) => self.items[index].is_expanded = value,
            Message::Reordered(keys) => {
                let mut by_name: HashMap<&'static str, ExpandedCardInfo> =
                    self.items.iter().map(|item| (item.name, *item)).collect();
                self.items = keys
                    .into_iter()
                    .filter_map(|key| by_name.remove(key.as_str()))
                    .collect();
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("SettingsExpander DragHandle");
        let rows = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let message = context.callback(move |value| Message::Expanded(index, value));
                let row = Grid::new()
                    .columns([GridLength::Auto, GridLength::STAR])
                    .children((
                        Border::new()
                            .width(36.0)
                            .height(75.0)
                            .margin(Thickness::uniform(1.0))
                            .corner_radius(8.0)
                            .background(Color::transparent())
                            .content(TextBlock::new().text("\u{283f}").font_size(20.0)),
                        SettingsExpander::new()
                            .grid_column(1)
                            .header(item.name)
                            .description(item.info)
                            .is_expanded(item.is_expanded)
                            .on_is_expanded_changed(message.clone())
                            .content(
                                ToggleSwitch::new()
                                    .is_on(item.is_expanded)
                                    .on_content("On")
                                    .off_content("Off")
                                    .on_toggled(message),
                            )
                            .item(
                                "link",
                                SettingsCard::new().header(item.link_description).content(
                                    HyperlinkButton::new()
                                        .content(item.url)
                                        .navigate_uri(item.url)
                                        .expect("valid sample URI"),
                                ),
                            ),
                    ));
                DataItem::new(item.name, item.name).content(row)
            })
            .collect::<Vec<_>>();
        ListView::new()
            .selection_mode(ListViewSelectionMode::None)
            .can_drag_items(true)
            .can_reorder_items(true)
            .allow_drop(true)
            .items(rows)
            .on_reordered(context.callback(Message::Reordered))
            .into()
    }
}

fn main() {
    App::run_component::<Sample>(()).expect("failed to start drag handle sample");
}
