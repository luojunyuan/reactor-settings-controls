---
title: SettingsExpander
description: A collapsible Windows 11 settings group for windows-reactor.
---

`SettingsExpander` uses the same header, description, icon, and header-content
slots as a card and adds a keyed `ItemsRepeater` body:

```rust
use reactor_settings_controls::*;

let expander = SettingsExpander::new()
    .header("Network")
    .description("Connection settings")
    .is_expanded(true)
    .item("proxy", SettingsCard::new().header("Proxy"))
    .items_header(InfoBar::new().title("Connected").is_open(true))
    .items_footer(SettingsCard::new().header("Add a device"));
```

Use `items_source_with` or `item_template` for typed data and a Rust closure;
the closure's result is converted into a Reactor `View` and indexed rows receive
stable keys. `keyed_items` accepts application supplied `KeyedView` values when
the source has a persistent identity.

`on_is_expanded_changed` receives every state, while `on_expanded` and
`on_collapsed` mirror the original events. The complete counterparts of the
source samples are in:

- `examples/settings_expander.rs`
- `examples/settings_expander_items_source.rs`
- `examples/settings_expander_drag_handle.rs`
- `examples/settings_page.rs`
