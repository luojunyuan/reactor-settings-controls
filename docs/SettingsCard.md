---
title: SettingsCard
description: A Windows 11 style settings row for windows-reactor.
---

`SettingsCard` accepts `header`, `description`, `header_icon`, and `content`
slots. Each slot accepts either a native Reactor view or text. The default
layout places metadata on the left and the setting on the right:

```rust
use reactor_settings_controls::*;

let card = SettingsCard::new()
    .header("This is the Header")
    .description("Choose an option")
    .header_icon(FontIcon::new().glyph("\u{e799}"))
    .content(ComboBox::new().selected_index(0).items_source(["One", "Two"]));
```

Set `is_click_enabled(true)` and add `on_click` when the row navigates to a
detail page or opens an external link. `action_icon`,
`action_icon_tooltip`, and `is_action_icon_visible` control the trailing action
glyph. `ContentAlignment::Left` is useful for checkboxes and radio buttons;
`ContentAlignment::Vertical` puts the metadata above the setting.

The complete counterparts of the original samples are in:

- `examples/settings_card.rs`
- `examples/clickable_settings_card.rs`
- `examples/settings_page.rs`
