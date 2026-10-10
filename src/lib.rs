#![doc = include_str!("../README.md")]
#![allow(clippy::module_name_repetitions)]

use std::rc::Rc;

pub use windows_reactor::*;

/// Content accepted by the Settings Controls slots.
///
/// Reactor controls use `View` for content rather than an untyped object. This
/// wrapper adds the string conversion that XAML users get from a `ContentPresenter`
/// while retaining the normal typed `View` conversion for every native control.
#[derive(Clone, Debug, PartialEq)]
pub struct SettingContent(View);

impl SettingContent {
    /// Creates text content using a native `TextBlock`.
    pub fn text(value: impl AsRef<str>) -> Self {
        Self(TextBlock::new().text(value).into())
    }

    /// Wraps an existing Reactor view.
    pub fn view(value: impl Into<View>) -> Self {
        Self(value.into())
    }

    /// Returns the underlying Reactor view.
    pub fn into_view(self) -> View {
        self.0
    }
}

/// Converts either a Reactor view or text into a Settings Controls content slot.
pub trait IntoSettingContent {
    fn into_setting_content(self) -> SettingContent;
}

impl<T> IntoSettingContent for T
where
    T: Into<View>,
{
    fn into_setting_content(self) -> SettingContent {
        SettingContent::view(self)
    }
}

impl From<SettingContent> for View {
    fn from(value: SettingContent) -> Self {
        value.0
    }
}

/// Alignment modes from the WinUI SettingsCard control.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ContentAlignment {
    /// Header metadata on the left and the setting content on the right.
    #[default]
    Right,
    /// Content occupies the card body and header metadata is collapsed.
    Left,
    /// Header metadata is above the setting content.
    Vertical,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct VisualOptions {
    width: Option<f64>,
    height: Option<f64>,
    min_width: Option<f64>,
    max_width: Option<f64>,
    min_height: Option<f64>,
    max_height: Option<f64>,
    grid_row: Option<i32>,
    grid_column: Option<i32>,
    grid_row_span: Option<i32>,
    grid_column_span: Option<i32>,
    automation_name: Option<Rc<str>>,
    automation_id: Option<Rc<str>>,
    margin: Option<Thickness>,
    horizontal_alignment: Option<HorizontalAlignment>,
    vertical_alignment: Option<VerticalAlignment>,
    opacity: Option<f64>,
    background: Option<Brush>,
    border_brush: Option<Brush>,
    border_thickness: Option<Thickness>,
    corner_radius: Option<CornerRadius>,
    padding: Option<Thickness>,
    is_tab_stop: Option<bool>,
}

impl VisualOptions {
    fn card_defaults() -> Self {
        Self {
            min_width: Some(148.0),
            min_height: Some(68.0),
            horizontal_alignment: Some(HorizontalAlignment::Stretch),
            vertical_alignment: Some(VerticalAlignment::Center),
            background: Some(ThemeBrush::CardBackground.into()),
            border_brush: Some(ThemeBrush::CardStroke.into()),
            border_thickness: Some(Thickness::uniform(1.0)),
            corner_radius: Some(CornerRadius::uniform(8.0)),
            padding: Some(Thickness::uniform(16.0)),
            is_tab_stop: Some(false),
            ..Self::default()
        }
    }
}

macro_rules! visual_setters {
    () => {
        /// Sets the explicit width.
        pub fn width(mut self, value: f64) -> Self {
            self.visual.width = Some(value);
            self
        }

        /// Sets the explicit height.
        pub fn height(mut self, value: f64) -> Self {
            self.visual.height = Some(value);
            self
        }

        /// Sets the minimum width.
        pub fn min_width(mut self, value: f64) -> Self {
            self.visual.min_width = Some(value);
            self
        }

        /// Sets the maximum width.
        pub fn max_width(mut self, value: f64) -> Self {
            self.visual.max_width = Some(value);
            self
        }

        /// Sets the minimum height.
        pub fn min_height(mut self, value: f64) -> Self {
            self.visual.min_height = Some(value);
            self
        }

        /// Places the control in a grid row.
        pub fn grid_row(mut self, value: i32) -> Self {
            self.visual.grid_row = Some(value);
            self
        }

        /// Places the control in a grid column.
        pub fn grid_column(mut self, value: i32) -> Self {
            self.visual.grid_column = Some(value);
            self
        }

        /// Sets the number of grid rows occupied by the control.
        pub fn grid_row_span(mut self, value: i32) -> Self {
            self.visual.grid_row_span = Some(value);
            self
        }

        /// Sets the number of grid columns occupied by the control.
        pub fn grid_column_span(mut self, value: i32) -> Self {
            self.visual.grid_column_span = Some(value);
            self
        }

        /// Sets the maximum height.
        pub fn max_height(mut self, value: f64) -> Self {
            self.visual.max_height = Some(value);
            self
        }

        /// Sets the outer margin.
        pub fn margin(mut self, value: impl Into<Thickness>) -> Self {
            self.visual.margin = Some(value.into());
            self
        }

        /// Sets the UI Automation name.
        pub fn automation_name(mut self, value: impl Into<Rc<str>>) -> Self {
            self.visual.automation_name = Some(value.into());
            self
        }

        /// Sets the UI Automation id.
        pub fn automation_id(mut self, value: impl Into<Rc<str>>) -> Self {
            self.visual.automation_id = Some(value.into());
            self
        }

        /// Sets the horizontal alignment.
        pub fn horizontal_alignment(mut self, value: HorizontalAlignment) -> Self {
            self.visual.horizontal_alignment = Some(value);
            self
        }

        /// Sets the vertical alignment.
        pub fn vertical_alignment(mut self, value: VerticalAlignment) -> Self {
            self.visual.vertical_alignment = Some(value);
            self
        }

        /// Sets the opacity.
        pub fn opacity(mut self, value: f64) -> Self {
            self.visual.opacity = Some(value);
            self
        }

        /// Sets the card background brush.
        pub fn background(mut self, value: impl Into<Brush>) -> Self {
            self.visual.background = Some(value.into());
            self
        }

        /// Sets the card border brush.
        pub fn border_brush(mut self, value: impl Into<Brush>) -> Self {
            self.visual.border_brush = Some(value.into());
            self
        }

        /// Sets the card border thickness.
        pub fn border_thickness(mut self, value: impl Into<Thickness>) -> Self {
            self.visual.border_thickness = Some(value.into());
            self
        }

        /// Sets the card corner radius.
        pub fn corner_radius(mut self, value: impl Into<CornerRadius>) -> Self {
            self.visual.corner_radius = Some(value.into());
            self
        }

        /// Sets the card padding.
        pub fn padding(mut self, value: impl Into<Thickness>) -> Self {
            self.visual.padding = Some(value.into());
            self
        }

        /// Controls whether the card participates in keyboard tab navigation.
        pub fn is_tab_stop(mut self, value: bool) -> Self {
            self.visual.is_tab_stop = Some(value);
            self
        }
    };
}

fn apply_border_visual(mut value: Border, visual: &VisualOptions) -> Border {
    if let Some(v) = visual.width {
        value = value.width(v);
    }
    if let Some(v) = visual.height {
        value = value.height(v);
    }
    if let Some(v) = visual.min_width {
        value = value.min_width(v);
    }
    if let Some(v) = visual.max_width {
        value = value.max_width(v);
    }
    if let Some(v) = visual.min_height {
        value = value.min_height(v);
    }
    if let Some(v) = visual.max_height {
        value = value.max_height(v);
    }
    if let Some(v) = visual.grid_row {
        value = value.grid_row(v);
    }
    if let Some(v) = visual.grid_column {
        value = value.grid_column(v);
    }
    if let Some(v) = visual.grid_row_span {
        value = value.grid_row_span(v);
    }
    if let Some(v) = visual.grid_column_span {
        value = value.grid_column_span(v);
    }
    if let Some(v) = visual.margin {
        value = value.margin(v);
    }
    if let Some(v) = &visual.automation_name {
        value = value.automation_name(v);
    }
    if let Some(v) = &visual.automation_id {
        value = value.automation_id(v);
    }
    if let Some(v) = visual.horizontal_alignment {
        value = value.horizontal_alignment(v);
    }
    if let Some(v) = visual.vertical_alignment {
        value = value.vertical_alignment(v);
    }
    if let Some(v) = visual.opacity {
        value = value.opacity(v);
    }
    if let Some(v) = visual.background {
        value = value.background(v);
    }
    if let Some(v) = visual.border_brush {
        value = value.border_brush(v);
    }
    if let Some(v) = visual.border_thickness {
        value = value.border_thickness(v);
    }
    if let Some(v) = visual.corner_radius {
        value = value.corner_radius(v);
    }
    if let Some(v) = visual.padding {
        value = value.padding(v);
    }
    if let Some(v) = visual.is_tab_stop {
        value = value.is_tab_stop(v);
    }
    value
}

fn apply_expander_visual(value: Expander, visual: &VisualOptions) -> Expander {
    let mut value = value;
    if let Some(v) = visual.width {
        value = value.width(v);
    }
    if let Some(v) = visual.height {
        value = value.height(v);
    }
    if let Some(v) = visual.min_width {
        value = value.min_width(v);
    }
    if let Some(v) = visual.max_width {
        value = value.max_width(v);
    }
    if let Some(v) = visual.min_height {
        value = value.min_height(v);
    }
    if let Some(v) = visual.max_height {
        value = value.max_height(v);
    }
    if let Some(v) = visual.grid_row {
        value = value.grid_row(v);
    }
    if let Some(v) = visual.grid_column {
        value = value.grid_column(v);
    }
    if let Some(v) = visual.grid_row_span {
        value = value.grid_row_span(v);
    }
    if let Some(v) = visual.grid_column_span {
        value = value.grid_column_span(v);
    }
    if let Some(v) = visual.margin {
        value = value.margin(v);
    }
    if let Some(v) = &visual.automation_name {
        value = value.automation_name(v);
    }
    if let Some(v) = &visual.automation_id {
        value = value.automation_id(v);
    }
    if let Some(v) = visual.horizontal_alignment {
        value = value.horizontal_alignment(v);
    }
    if let Some(v) = visual.vertical_alignment {
        value = value.vertical_alignment(v);
    }
    if let Some(v) = visual.opacity {
        value = value.opacity(v);
    }
    value
}

fn text_panel(
    header: Option<SettingContent>,
    description: Option<SettingContent>,
    _foreground: Option<Brush>,
) -> Option<View> {
    let mut children = Vec::new();
    if let Some(header) = header {
        children.push(header.into_view());
    }
    if let Some(description) = description {
        children.push(description.into_view());
    }
    (!children.is_empty()).then(|| StackPanel::new().spacing(2.0).children(children).into())
}

fn apply_content_foreground(content: View, foreground: Option<Brush>) -> View {
    let _ = foreground;
    content
}

/// A Windows 11 style settings row.
///
/// The control is a declarative composite. It produces a stable Reactor view
/// made from `Border`, `Grid`, `StackPanel`, and the supplied content, so it can
/// be used anywhere a normal `View` is accepted.
#[derive(Clone, Debug, PartialEq)]
pub struct SettingsCard {
    header: Option<SettingContent>,
    description: Option<SettingContent>,
    header_icon: Option<SettingContent>,
    action_icon: Option<SettingContent>,
    action_icon_tooltip: Option<Rc<str>>,
    content: Option<SettingContent>,
    content_alignment: ContentAlignment,
    horizontal_content_alignment: HorizontalAlignment,
    vertical_content_alignment: VerticalAlignment,
    is_click_enabled: bool,
    is_action_icon_visible: bool,
    is_enabled: bool,
    foreground: Option<Brush>,
    wrap_threshold: f64,
    wrap_no_icon_threshold: f64,
    left_indention: f64,
    on_click: Option<Callback<()>>,
    visual: VisualOptions,
}

impl SettingsCard {
    /// Creates a card with the Community Toolkit defaults.
    pub fn new() -> Self {
        Self {
            header: None,
            description: None,
            header_icon: None,
            action_icon: Some(SettingContent::view(FontIcon::new().glyph("\u{e974}"))),
            action_icon_tooltip: None,
            content: None,
            content_alignment: ContentAlignment::Right,
            horizontal_content_alignment: HorizontalAlignment::Right,
            vertical_content_alignment: VerticalAlignment::Center,
            is_click_enabled: false,
            is_action_icon_visible: true,
            is_enabled: true,
            foreground: Some(ThemeBrush::PrimaryText.into()),
            wrap_threshold: 476.0,
            wrap_no_icon_threshold: 286.0,
            left_indention: 0.0,
            on_click: None,
            visual: VisualOptions::card_defaults(),
        }
    }

    visual_setters!();

    /// Sets the header content. Strings become `TextBlock`s.
    pub fn header(mut self, value: impl IntoSettingContent) -> Self {
        self.header = Some(value.into_setting_content());
        self
    }

    /// Clears or sets the header content.
    pub fn header_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.header = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Sets the description content. Strings become `TextBlock`s.
    pub fn description(mut self, value: impl IntoSettingContent) -> Self {
        self.description = Some(value.into_setting_content());
        self
    }

    /// Clears or sets the description content.
    pub fn description_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.description = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Sets the leading icon.
    pub fn header_icon(mut self, value: impl IntoSettingContent) -> Self {
        self.header_icon = Some(value.into_setting_content());
        self
    }

    /// Clears or sets the leading icon.
    pub fn header_icon_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.header_icon = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Sets the trailing action icon.
    pub fn action_icon(mut self, value: impl IntoSettingContent) -> Self {
        self.action_icon = Some(value.into_setting_content());
        self
    }

    /// Clears or sets the trailing action icon.
    pub fn action_icon_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.action_icon = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Sets the action icon tooltip.
    pub fn action_icon_tooltip(mut self, value: impl Into<Rc<str>>) -> Self {
        self.action_icon_tooltip = Some(value.into());
        self
    }

    /// Sets the setting content.
    pub fn content(mut self, value: impl IntoSettingContent) -> Self {
        self.content = Some(value.into_setting_content());
        self
    }

    /// Clears or sets the setting content.
    pub fn content_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.content = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Sets the Toolkit content alignment mode.
    pub fn content_alignment(mut self, value: ContentAlignment) -> Self {
        self.content_alignment = value;
        self
    }

    /// Sets the horizontal alignment used by the card body.
    pub fn horizontal_content_alignment(mut self, value: HorizontalAlignment) -> Self {
        self.horizontal_content_alignment = value;
        self
    }

    /// Sets the vertical alignment used by the card body.
    pub fn vertical_content_alignment(mut self, value: VerticalAlignment) -> Self {
        self.vertical_content_alignment = value;
        self
    }

    /// Enables pointer and keyboard activation for the card.
    pub fn is_click_enabled(mut self, value: bool) -> Self {
        self.is_click_enabled = value;
        self
    }

    /// Controls whether the action icon is shown when the card is clickable.
    pub fn is_action_icon_visible(mut self, value: bool) -> Self {
        self.is_action_icon_visible = value;
        self
    }

    /// Enables or disables the card.
    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = value;
        self
    }

    /// Sets the text and icon foreground brush.
    pub fn foreground(mut self, value: impl Into<Brush>) -> Self {
        self.foreground = Some(value.into());
        self
    }

    /// Overrides the width at which the card switches to a vertical layout.
    pub fn wrap_threshold(mut self, value: f64) -> Self {
        self.wrap_threshold = value;
        self
    }

    /// Overrides the width at which the icon is removed from a wrapped card.
    pub fn wrap_no_icon_threshold(mut self, value: f64) -> Self {
        self.wrap_no_icon_threshold = value;
        self
    }

    /// Sets the leading indentation used by the card grid.
    pub fn left_indention(mut self, value: f64) -> Self {
        self.left_indention = value;
        self
    }

    /// Receives activation from pointer or keyboard input.
    pub fn on_click(mut self, callback: impl IntoUnitCallback) -> Self {
        self.on_click = Some(callback.into_unit_callback());
        self
    }

    fn body(self) -> View {
        let SettingsCard {
            header,
            description,
            mut header_icon,
            action_icon,
            action_icon_tooltip,
            content,
            content_alignment,
            horizontal_content_alignment,
            vertical_content_alignment,
            is_click_enabled,
            is_action_icon_visible,
            is_enabled,
            foreground,
            wrap_threshold,
            wrap_no_icon_threshold,
            left_indention,
            on_click,
            visual,
        } = self;

        let mut icon_is_visible = header_icon.is_some();
        let mut alignment = content_alignment;
        if let Some(width) = visual.width {
            if width <= wrap_threshold {
                alignment = ContentAlignment::Vertical;
            }
            if width <= wrap_no_icon_threshold {
                icon_is_visible = false;
            }
        }
        if !icon_is_visible {
            header_icon = None;
        }

        let panel = text_panel(header, description, foreground);
        let content = content.map(SettingContent::into_view);
        let mut body: View = match alignment {
            ContentAlignment::Left => {
                let mut children = Vec::new();
                if let Some(content) = content {
                    children.push(apply_content_foreground(content, visual.background));
                } else if let Some(panel) = panel {
                    children.push(panel);
                }
                StackPanel::new()
                    .orientation(Orientation::Vertical)
                    .children(children)
                    .into()
            }
            ContentAlignment::Vertical => {
                let mut children = Vec::new();
                if let Some(icon) = header_icon {
                    children.push(icon.into_view());
                }
                if let Some(panel) = panel {
                    children.push(panel);
                }
                if let Some(content) = content {
                    children.push(apply_content_foreground(content, visual.background));
                }
                StackPanel::new()
                    .spacing(8.0)
                    .orientation(Orientation::Vertical)
                    .children(children)
                    .into()
            }
            ContentAlignment::Right => {
                let mut children = Vec::new();
                let mut grid = Grid::new()
                    .columns([
                        GridLength::Pixel(left_indention),
                        GridLength::Auto,
                        GridLength::STAR,
                        GridLength::Auto,
                        GridLength::Auto,
                    ])
                    .horizontal_alignment(horizontal_content_alignment)
                    .vertical_alignment(vertical_content_alignment);
                if let Some(icon) = header_icon {
                    children.push(
                        Border::new()
                            .grid_column(1)
                            .max_width(20.0)
                            .max_height(20.0)
                            .content(icon.into_view())
                            .into(),
                    );
                }
                if let Some(panel) = panel {
                    children.push(Border::new().grid_column(2).content(panel).into());
                }
                if let Some(content) = content {
                    children.push(
                        Border::new()
                            .grid_column(3)
                            .content(apply_content_foreground(content, visual.background))
                            .into(),
                    );
                }
                if is_click_enabled && is_action_icon_visible {
                    if let Some(action_icon) = action_icon {
                        let action = Border::new()
                            .grid_column(4)
                            .max_width(13.0)
                            .content(action_icon.into_view());
                        let action: View = if let Some(tooltip) = action_icon_tooltip {
                            action.tooltip(tooltip)
                        } else {
                            action.into()
                        };
                        children.push(action);
                    }
                }
                grid = grid.children(children);
                grid.into()
            }
        };

        if !is_enabled {
            body = Border::new().opacity(0.55).content(body).into();
        }

        let mut root = Border::new().content(body);
        root = apply_border_visual(root, &visual);
        if is_click_enabled && is_enabled {
            root = root
                .is_tab_stop(true)
                .capture_pointer_on_press(true)
                .focus_on_pointer_release(true);
            if let Some(callback) = on_click {
                let pointer_callback = callback.clone();
                root = root.on_pointer_released(move |_| pointer_callback.call(()));
                root = root.on_key_up(RoutedCallback::new(move |event: KeyEventInfo| {
                    if event.key == VirtualKey::ENTER || event.key == VirtualKey::SPACE {
                        callback.call(());
                    }
                    true
                }));
            }
        }
        root.into()
    }
}

impl Default for SettingsCard {
    fn default() -> Self {
        Self::new()
    }
}

impl From<SettingsCard> for View {
    fn from(value: SettingsCard) -> Self {
        value.body()
    }
}

/// A collapsible group of settings rows.
#[derive(Clone, Debug, PartialEq)]
pub struct SettingsExpander {
    header: Option<SettingContent>,
    description: Option<SettingContent>,
    header_icon: Option<SettingContent>,
    content: Option<SettingContent>,
    items_header: Option<SettingContent>,
    items_footer: Option<SettingContent>,
    items: Vec<KeyedView>,
    is_expanded: bool,
    is_enabled: bool,
    horizontal_content_alignment: HorizontalAlignment,
    foreground: Option<Brush>,
    resource_overrides: Option<ResourceOverrides>,
    on_is_expanded_changed: Option<Callback<bool>>,
    on_expanded: Option<Callback<()>>,
    on_collapsed: Option<Callback<()>>,
    visual: VisualOptions,
}

impl SettingsExpander {
    /// Creates a collapsed expander with the Toolkit card defaults.
    pub fn new() -> Self {
        Self {
            header: None,
            description: None,
            header_icon: None,
            content: None,
            items_header: None,
            items_footer: None,
            items: Vec::new(),
            is_expanded: false,
            is_enabled: true,
            horizontal_content_alignment: HorizontalAlignment::Stretch,
            foreground: Some(ThemeBrush::PrimaryText.into()),
            resource_overrides: None,
            on_is_expanded_changed: None,
            on_expanded: None,
            on_collapsed: None,
            visual: VisualOptions::card_defaults(),
        }
    }

    visual_setters!();

    /// Sets the header content.
    pub fn header(mut self, value: impl IntoSettingContent) -> Self {
        self.header = Some(value.into_setting_content());
        self
    }

    /// Clears or sets the header content.
    pub fn header_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.header = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Sets the description content.
    pub fn description(mut self, value: impl IntoSettingContent) -> Self {
        self.description = Some(value.into_setting_content());
        self
    }

    /// Clears or sets the description content.
    pub fn description_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.description = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Sets the leading icon.
    pub fn header_icon(mut self, value: impl IntoSettingContent) -> Self {
        self.header_icon = Some(value.into_setting_content());
        self
    }

    /// Clears or sets the leading icon.
    pub fn header_icon_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.header_icon = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Sets the content shown in the expander header.
    pub fn content(mut self, value: impl IntoSettingContent) -> Self {
        self.content = Some(value.into_setting_content());
        self
    }

    /// Clears or sets the content shown in the expander header.
    pub fn content_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.content = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Sets custom content before the item repeater.
    pub fn items_header(mut self, value: impl IntoSettingContent) -> Self {
        self.items_header = Some(value.into_setting_content());
        self
    }

    /// Clears or sets custom content before the item repeater.
    pub fn items_header_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.items_header = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Sets custom content after the item repeater.
    pub fn items_footer(mut self, value: impl IntoSettingContent) -> Self {
        self.items_footer = Some(value.into_setting_content());
        self
    }

    /// Clears or sets custom content after the item repeater.
    pub fn items_footer_optional<T: IntoSettingContent>(mut self, value: Option<T>) -> Self {
        self.items_footer = value.map(IntoSettingContent::into_setting_content);
        self
    }

    /// Adds eagerly declared items using positional keys.
    pub fn items(mut self, values: impl IntoViews) -> Self {
        self.items = values
            .into_visuals()
            .into_iter()
            .enumerate()
            .map(|(index, value)| keyed(index, value))
            .collect();
        self
    }

    /// Adds items with stable application supplied keys.
    pub fn keyed_items(mut self, values: impl IntoIterator<Item = KeyedView>) -> Self {
        self.items = values.into_iter().collect();
        self
    }

    /// Adds one keyed item.
    pub fn item(mut self, key: impl Into<Key>, value: impl Into<View>) -> Self {
        self.items.push(keyed(key, value));
        self
    }

    /// Uses an already materialized source as the item source.
    pub fn items_source(self, values: impl IntoViews) -> Self {
        self.items(values)
    }

    /// Adds an iterator of views, including `Vec<SettingsCard>` and other
    /// collections whose item type converts to `View`.
    pub fn items_iter<I>(mut self, values: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<View>,
    {
        self.items = values
            .into_iter()
            .enumerate()
            .map(|(index, value)| keyed(index, value))
            .collect();
        self
    }

    /// Iterator form of `items_source`.
    pub fn items_source_iter<I>(self, values: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<View>,
    {
        self.items_iter(values)
    }

    /// Materializes a source with a typed item template.
    pub fn items_source_with<I, T, F, V>(mut self, values: I, template: F) -> Self
    where
        I: IntoIterator<Item = T>,
        F: Fn(T) -> V,
        V: Into<View>,
    {
        self.items = values
            .into_iter()
            .enumerate()
            .map(|(index, value)| keyed(index, template(value)))
            .collect();
        self
    }

    /// Alias matching the XAML `ItemTemplate` concept.
    pub fn item_template<I, T, F, V>(self, values: I, template: F) -> Self
    where
        I: IntoIterator<Item = T>,
        F: Fn(T) -> V,
        V: Into<View>,
    {
        self.items_source_with(values, template)
    }

    /// Sets the expanded state.
    pub fn is_expanded(mut self, value: bool) -> Self {
        self.is_expanded = value;
        self
    }

    /// Enables or disables the expander.
    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = value;
        self
    }

    /// Sets the header content alignment.
    pub fn horizontal_content_alignment(mut self, value: HorizontalAlignment) -> Self {
        self.horizontal_content_alignment = value;
        self
    }

    /// Sets the header text and icon foreground brush.
    pub fn foreground(mut self, value: impl Into<Brush>) -> Self {
        self.foreground = Some(value.into());
        self
    }

    /// Overrides native Expander resources supported by Reactor.
    pub fn resource_overrides(mut self, value: ResourceOverrides) -> Self {
        self.resource_overrides = Some(value);
        self
    }

    /// Receives every expanded state change.
    pub fn on_is_expanded_changed(mut self, callback: impl IntoPayloadCallback<bool>) -> Self {
        self.on_is_expanded_changed = Some(callback.into_payload_callback());
        self
    }

    /// Receives the event only when the expander opens.
    pub fn on_expanded(mut self, callback: impl IntoUnitCallback) -> Self {
        self.on_expanded = Some(callback.into_unit_callback());
        self
    }

    /// Receives the event only when the expander closes.
    pub fn on_collapsed(mut self, callback: impl IntoUnitCallback) -> Self {
        self.on_collapsed = Some(callback.into_unit_callback());
        self
    }

    fn body(self) -> View {
        let SettingsExpander {
            header,
            description,
            header_icon,
            content,
            items_header,
            items_footer,
            items,
            is_expanded,
            is_enabled,
            horizontal_content_alignment,
            foreground,
            resource_overrides,
            on_is_expanded_changed,
            on_expanded,
            on_collapsed,
            visual,
        } = self;

        let header_padding = visual
            .padding
            .unwrap_or_else(|| Thickness::new(16.0, 16.0, 4.0, 16.0));
        let header = SettingsCard::new()
            .header_optional(header)
            .description_optional(description)
            .header_icon_optional(header_icon)
            .content_optional(content)
            .is_enabled(is_enabled)
            .foreground(foreground.unwrap_or_else(|| ThemeBrush::PrimaryText.into()))
            .padding(header_padding)
            .background(Color::transparent())
            .border_thickness(Thickness::uniform(0.0));

        let mut children = Vec::with_capacity(
            2 + usize::from(items_header.is_some() as u8)
                + usize::from(items_footer.is_some() as u8),
        );
        if let Some(value) = items_header {
            children.push(Border::new().grid_row(0).content(value.into_view()).into());
        }
        children.push(ItemsRepeater::new().items(items).grid_row(1).into());
        if let Some(value) = items_footer {
            children.push(Border::new().grid_row(2).content(value.into_view()).into());
        }
        let item_content = Grid::new()
            .rows([GridLength::Auto, GridLength::STAR, GridLength::Auto])
            .children(children);

        let mut expander = Expander::new()
            .is_expanded(is_expanded)
            .horizontal_content_alignment(horizontal_content_alignment)
            .header(header)
            .content(item_content);

        if let Some(resources) = resource_overrides {
            expander = expander.resource_overrides(resources);
        }

        if on_is_expanded_changed.is_some() || on_expanded.is_some() || on_collapsed.is_some() {
            expander = expander.on_is_expanded_changed(move |value| {
                if let Some(callback) = &on_is_expanded_changed {
                    callback.call(value);
                }
                if value {
                    if let Some(callback) = &on_expanded {
                        callback.call(());
                    }
                } else if let Some(callback) = &on_collapsed {
                    callback.call(());
                }
            });
        }

        let expander = apply_expander_visual(expander, &visual);
        let mut root_visual = visual;
        root_visual.padding = None;
        let mut root = Border::new().content(expander);
        root = apply_border_visual(root, &root_visual);
        if !is_enabled {
            root = root.opacity(0.55);
        }
        root.into()
    }
}

impl Default for SettingsExpander {
    fn default() -> Self {
        Self::new()
    }
}

impl From<SettingsExpander> for View {
    fn from(value: SettingsExpander) -> Self {
        value.body()
    }
}

/// Convenient imports for an application using the controls.
pub mod prelude {
    pub use crate::{
        ContentAlignment, IntoSettingContent, SettingContent, SettingsCard, SettingsExpander,
    };
    pub use windows_reactor::*;
}
