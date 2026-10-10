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
        context.window_title("ClickableSettingsCard");
        let clicked = context.message(());
        StackPanel::new().spacing(4.0).children((
            SettingsCard::new()
                .header("A clickable SettingsCard")
                .description("A SettingsCard can be made clickable and you can leverage the Command property or Click event.")
                .header_icon(FontIcon::new().glyph("\u{e799}"))
                .is_click_enabled(true)
                .on_click(clicked.clone())
                .content(TextBlock::new().text("This is content").foreground(ThemeBrush::PrimaryText)),
            SettingsCard::new()
                .header("Customizing the ActionIcon")
                .description("You can customize the ActionIcon and ActionIconToolTip.")
                .header_icon(FontIcon::new().glyph("\u{e774}"))
                .action_icon(FontIcon::new().glyph("\u{e8a7}"))
                .action_icon_tooltip("Open in new window")
                .is_click_enabled(true)
                .on_click(clicked.clone()),
            SettingsCard::new()
                .header("Hiding the ActionIcon")
                .header_icon(FontIcon::new().glyph("\u{e72e}"))
                .is_action_icon_visible(false)
                .is_click_enabled(true)
                .on_click(clicked),
        )).into()
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {}
}

fn main() {
    App::run_component::<Sample>(()).expect("failed to start clickable SettingsCard sample");
}
