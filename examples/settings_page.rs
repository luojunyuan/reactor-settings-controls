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
        context.window_title("Settings Page Example");
        let stack = StackPanel::new().spacing(4.0).max_width(1000.0).children((
            TextBlock::new().text("Section 1").font_weight(FontWeight::BOLD).margin(Thickness::new(1.0, 30.0, 0.0, 6.0)),
            SettingsCard::new()
                .header("This is the Header")
                .description("This is a default card, with the Header, HeaderIcon, Description and Content set")
                .header_icon(FontIcon::new().glyph("\u{e716}"))
                .content(ToggleSwitch::new().is_on(true)),
            SettingsExpander::new()
                .header("SettingsExpander")
                .description("The SettingsExpander has the same properties as a SettingsCard")
                .header_icon(FontIcon::new().glyph("\u{e91b}"))
                .content(Button::new().style(ButtonStyle::Accent).content("Content"))
                .item(
                    "basic",
                    SettingsCard::new()
                        .header("A basic SettingsCard within an SettingsExpander")
                        .content(Button::new().content("Button")),
                )
                .item(
                    "clickable",
                    SettingsCard::new()
                        .header("This item can be clicked")
                        .description("SettingsCard within an Expander can be made clickable too!")
                        .is_click_enabled(true)
                        .on_click(context.message(())),
                )
                .item(
                    "left",
                    SettingsCard::new()
                        .content_alignment(ContentAlignment::Left)
                        .content(CheckBox::new().content("Here the ContentAlignment is set to Left. This is great for e.g. CheckBoxes or RadioButtons")),
                ),
            TextBlock::new().text("Section 2").font_weight(FontWeight::BOLD).margin(Thickness::new(1.0, 30.0, 0.0, 6.0)),
            SettingsCard::new()
                .header("Another SettingsCard")
                .description("Another card to show grouping of cards")
                .header_icon(FontIcon::new().glyph("\u{e799}"))
                .content(ComboBox::new().selected_index(0).items_source(["Option 1", "Option 2", "Option 3"])),
            SettingsCard::new()
                .header("Yet another SettingsCard")
                .description("Another card to show grouping of cards")
                .header_icon(FontIcon::new().glyph("\u{e768}"))
                .content(Button::new().content("Content")),
            TextBlock::new().text("About").font_weight(FontWeight::BOLD).margin(Thickness::new(1.0, 30.0, 0.0, 6.0)),
            SettingsExpander::new()
                .header("Community Toolkit Gallery")
                .description("© 2023. All rights reserved.")
                .header_icon(
                    BitmapIcon::new()
                        .uri_source("ms-appx:///Assets/AppTitleBar.scale-200.png")
                        .expect("valid packaged URI"),
                )
                .content(TextBlock::new().text("Version 8.0.0").is_text_selection_enabled(true))
                .item(
                    "links",
                    SettingsCard::new()
                        .content_alignment(ContentAlignment::Left)
                        .horizontal_content_alignment(HorizontalAlignment::Left)
                        .content(StackPanel::new().children((
                            HyperlinkButton::new().content("Link 1"),
                            HyperlinkButton::new().content("Link 2"),
                            HyperlinkButton::new().content("Link 3"),
                        ))),
                ),
            HyperlinkButton::new().content("Send feedback").margin(Thickness::new(0.0, 8.0, 0.0, 0.0)),
        ));
        ScrollViewer::new()
            .content(Grid::new().children((stack,)))
            .into()
    }
}

fn main() {
    App::run_component::<Sample>(()).expect("failed to start Settings Page sample");
}
