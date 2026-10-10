# Source Inventory

The port was checked against every source file under
`Windows/components/SettingsControls`.

## SettingsCard

- `Header`
- `Description`
- `HeaderIcon`
- `ActionIcon`
- `ActionIconToolTip`
- `IsClickEnabled`
- `ContentAlignment` (`Right`, `Left`, `Vertical`)
- `IsActionIconVisible`
- `Content`
- `IsEnabled`
- common visual properties (`Width`, `Height`, min/max bounds, margin,
  alignment, opacity, background, border, corner radius, padding, tab stop)
- click activation for pointer release and Enter/Space keyboard input

## SettingsExpander

- `Header`
- `Description`
- `HeaderIcon`
- `Content`
- `ItemsHeader`
- `ItemsFooter`
- `IsExpanded`
- `Items`
- `ItemsSource`
- `ItemTemplate`
- `ItemContainerStyleSelector` equivalent through item builders and
  `items_source_with`
- `Expanded` and `Collapsed` callbacks
- common visual properties and native `ResourceOverrides`

## Samples

The six source sample pages are represented by these examples:

| Source page | Reactor example |
| --- | --- |
| `SettingsCardSample` | `examples/settings_card.rs` |
| `ClickableSettingsCardSample` | `examples/clickable_settings_card.rs` |
| `SettingsExpanderSample` | `examples/settings_expander.rs` |
| `SettingsExpanderItemsSourceSample` | `examples/settings_expander_items_source.rs` |
| `SettingsExpanderDragHandleSample` | `examples/settings_expander_drag_handle.rs` |
| `SettingsPageExample` | `examples/settings_page.rs` |

The source `Generic.xaml`, control templates, visual state names, and helper
converters are represented by deterministic primitive view construction and
the default dimensions in `VisualOptions::card_defaults`.
