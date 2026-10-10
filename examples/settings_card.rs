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
        context.window_title("SettingsCard");
        StackPanel::new().spacing(4.0).children((
            SettingsCard::new()
                .header("This is the Header")
                .description("This is a default card, with the Header, HeaderIcon, Description and Content set.")
                .header_icon(FontIcon::new().glyph("\u{e799}"))
                .is_enabled(true)
                .content(
                    ComboBox::new()
                        .selected_index(0)
                        .items_source(["Option 1", "Option 2", "Option 3"]),
                ),
            SettingsCard::new()
                .header("Icon options")
                .description("You can use a FontIcon, SymbolIcon or BitmapIcon to set the cards HeaderIcon.")
                .header_icon(
                    BitmapIcon::new()
                        .uri_source("ms-appx:///Assets/AppTitleBar.scale-200.png")
                        .expect("valid packaged URI"),
                )
                .content(ToggleSwitch::new()),
            SettingsCard::new()
                .header("A card with custom objects as its Description")
                .description(HyperlinkButton::new().content("Learn more about Phone Link"))
                .content(Button::new().style(ButtonStyle::Accent).content("Open Phone Link")),
            SettingsCard::new()
                .header("Adaptive layouts")
                .description("When resizing a SettingsCard, the Content will wrap vertically. You can override this breakpoint by setting the SettingsCardWrapThreshold resource. For edge cases, you can also hide the icon by setting SettingsCardWrapNoIconThreshold.")
                .header_icon(FontIcon::new().glyph("\u{e745}"))
                .wrap_threshold(800.0)
                .wrap_no_icon_threshold(600.0)
                .content(Button::new().style(ButtonStyle::Accent).content("This control will wrap vertically!")),
            SettingsCard::new().header("This is a card with a Header only"),
        )).into()
    }
}

fn main() {
    App::run_component::<Sample>(()).expect("failed to start SettingsCard sample");
}
