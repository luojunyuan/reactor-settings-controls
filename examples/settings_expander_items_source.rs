#![windows_subsystem = "windows"]

use reactor_settings_controls::*;

#[derive(Clone)]
struct MyDataModel {
    name: &'static str,
    info: &'static str,
    item_type: &'static str,
    link_description: &'static str,
    url: &'static str,
}

fn data() -> [MyDataModel; 3] {
    [
        MyDataModel {
            name: "First Item",
            info: "More about first item.",
            item_type: "Item type: Button",
            link_description: "Click here for more on first item.",
            url: "https://microsoft.com/",
        },
        MyDataModel {
            name: "Second Item",
            info: "More about second item.",
            item_type: "Item type: Link button",
            link_description: "Click here for more on second item.",
            url: "https://xbox.com/",
        },
        MyDataModel {
            name: "Third Item",
            info: "More about third item.",
            item_type: "Item type: No button",
            link_description: "Click here for more on third item.",
            url: "https://toolkitlabs.dev/",
        },
    ]
}

struct Sample;

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("SettingsExpander ItemsSource");
        let first = SettingsExpander::new()
            .header("Settings Expander with ItemsSource")
            .description("The SettingsExpander can use ItemsSource to define its Items.")
            .header_icon(FontIcon::new().glyph("\u{ea37}"))
            .items_source_with(data(), |item| {
                SettingsCard::new()
                    .header(item.name)
                    .description(item.info)
                    .content(
                        HyperlinkButton::new()
                            .content(item.link_description)
                            .navigate_uri(item.url)
                            .expect("valid sample URI"),
                    )
            })
            .items_header(
                StackPanel::new().children((
                    InfoBar::new()
                        .title("This is the ItemsHeader")
                        .severity(InfoBarSeverity::Success)
                        .is_open(true),
                    HyperlinkButton::new().content("It can host custom content"),
                )),
            )
            .items_footer(
                SettingsCard::new()
                    .header("This is the ItemsFooter")
                    .content(Button::new().content("Add a device")),
            );

        let second = SettingsExpander::new()
            .header("Settings Expander with a custom ItemTemplate")
            .description("SettingsExpander can use a DataTemplate, DataTemplateSelector, or IElementFactory for its ItemTemplate.")
            .header_icon(FontIcon::new().glyph("\u{e8fd}"))
            .item_template(data(), move |item| {
                let card = SettingsCard::new()
                    .header(item.name)
                    .description(item.item_type);
                if item.item_type.ends_with("Button") && !item.item_type.ends_with("Link button") {
                    card.content(Button::new().content(item.link_description).on_click(context.message(())))
                } else if item.item_type.ends_with("Link button") {
                    card.content(
                        HyperlinkButton::new()
                            .content(item.link_description)
                            .navigate_uri(item.url)
                            .expect("valid sample URI"),
                    )
                } else {
                    card
                }
            });

        StackPanel::new()
            .spacing(4.0)
            .children((first, second))
            .into()
    }
}

fn main() {
    App::run_component::<Sample>(()).expect("failed to start ItemsSource sample");
}
