#![windows_subsystem = "windows"]

use reactor_settings_controls::*;

struct Sample;

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("SettingsExpander");
        SettingsExpander::new()
            .vertical_alignment(VerticalAlignment::Top)
            .header("SettingsExpander")
            .description("The SettingsExpander has the same properties as a Card, and you can set SettingsCard as part of the Items collection.")
            .header_icon(FontIcon::new().glyph("\u{e91b}"))
            .is_enabled(true)
            .is_expanded(false)
            .content(
                ComboBox::new()
                    .selected_index(0)
                    .items_source(["Option 1", "Option 2", "Option 3"]),
            )
            .items((
                SettingsCard::new()
                    .header("A basic SettingsCard within an SettingsExpander")
                    .content(Button::new().content("Button")),
                SettingsCard::new()
                    .header("This item can be clicked")
                    .description("SettingsCard within an Expander can be made clickable too!")
                    .is_click_enabled(true)
                    .on_click(context.message(())),
                SettingsCard::new()
                    .content_alignment(ContentAlignment::Left)
                    .content(CheckBox::new().content("Here the ContentAlignment is set to Left. This is great for e.g. CheckBoxes or RadioButtons.")),
                SettingsCard::new()
                    .horizontal_content_alignment(HorizontalAlignment::Left)
                    .content_alignment(ContentAlignment::Vertical)
                    .header("Vertically aligned")
                    .description("You can also align your content vertically. Make sure to set the HorizontalAlignment to Left when you do!")
                    .content(
                        GridView::new().selected_index(1).items((
                            GridViewItem::new().content(Border::new().width(64.0).height(64.0).background(Color::rgb(0, 120, 212)).corner_radius(4.0)),
                            GridViewItem::new().content(Border::new().width(64.0).height(64.0).background(Color::rgb(0, 94, 183)).corner_radius(4.0)),
                            GridViewItem::new().content(Border::new().width(64.0).height(64.0).background(Color::rgb(0, 61, 146)).corner_radius(4.0)),
                            GridViewItem::new().content(Border::new().width(64.0).height(64.0).background(Color::rgb(0, 25, 104)).corner_radius(4.0)),
                        )),
                    ),
                SettingsCard::new()
                    .header("Customization")
                    .description("You can override the Left indention of a SettingsCard by overriding the SettingsCardLeftIndention")
                    .left_indention(40.0),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<Sample>(()).expect("failed to start SettingsExpander sample");
}
