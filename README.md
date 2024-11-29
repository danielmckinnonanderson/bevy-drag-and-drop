# Bevy Drag & Drop Plugin
A small, simple library for adding mouse-based drag and drop functionality to a Bevy game.

## Quick Start
Add to `Cargo.toml`:
```toml
bevy_drag_and_drop = { git = "https://github.com/danielmckinnonanderson/bevy-drag-and-drop" }
```

In your app setup:
```rust
app.add_plugins(CursorDragAndDropPlugin);
```

Usage:
- Add `Draggable` to draggable entities. Specify the area which will register cursor left-clicks (for dragging) by passing a `Rectangle` to this component.
- `Dragging` marker component moves the entity with the cursor each frame while holding left-click on it. The plugin systems will handle adding / removing the `Dragging` component and translating it according to the cursor position.

## License
[Mozilla Public License Version 2.0](./LICENSE)

