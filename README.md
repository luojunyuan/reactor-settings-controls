# reactor-settings-controls

`reactor-settings-controls` is the Reactor implementation of the Community
Toolkit `SettingsControls` package. It provides the same two controls as
declarative Rust builders:

- `SettingsCard` is a settings row with header, description, leading icon,
  trailing action icon, arbitrary content, alignment modes, and click input.
- `SettingsExpander` is a collapsible settings group with header content,
  `ItemsHeader`, `Items`, `ItemsFooter`, typed expansion callbacks, and keyed
  item reconciliation. `items_source_with_key` preserves application identity;
  `virtual_items_source` maps directly to Reactor's lazy `VirtualSource`.

The crate follows the `windows-reactor` model: every control is a pure builder
that converts into a `View`, and callbacks are typed Reactor callbacks. Strings
are accepted by the Settings Controls slots and are lowered to `TextBlock`
views through `IntoSettingContent`.

## Dependency

The dependency is a path dependency on the checked out `windows-rs` revision in
the parent workspace. `WINDOWS_RS_REVISION` records the exact Git revision used
by that checkout, keeping the control implementation on the same generated
declarations and native runtime as the local Reactor source.

```toml
[dependencies]
reactor-settings-controls = { path = "../reactor-settings-controls" }
windows-reactor = { path = "../windows-rs/crates/libs/reactor" }
```

## Example

```rust
use reactor_settings_controls::{ComboBox, SettingsCard, SettingsExpander, TextBlock, View};

let view: View = SettingsExpander::new()
    .header("Network")
    .description("Connection and proxy settings")
    .content(ComboBox::new().selected_index(0).items_source(["Automatic", "Manual"]))
    .item(
        "proxy",
        SettingsCard::new()
            .header("Proxy")
            .description("Use a system proxy")
            .content(TextBlock::new().text("Enabled")),
    )
    .into();
```

`examples/settings_card.rs`, `clickable_settings_card.rs`,
`settings_expander.rs`, `settings_expander_items_source.rs`,
`settings_expander_drag_handle.rs`, and `settings_page.rs` are direct Reactor
counterparts of the six XAML sample pages in the source SettingsControls
package. They preserve the sample labels, order, content slots, and item data.

## Source mapping

The original package is under `../Windows/components/SettingsControls`. Its
`SettingsCard.xaml` and `SettingsExpander.xaml` templates become the stable
primitive view trees built in `src/lib.rs`; the original dependency properties
map to builder methods and the original events map to typed Reactor callbacks.

The native `windows-reactor` dependency is intentionally not forked or patched:
the controls compose the generated primitives and therefore continue to follow
Reactor's reconciliation, keyed item, callback, and component rules.

## License

MIT OR Apache-2.0, matching `windows-rs` and the source Community Toolkit
component.
