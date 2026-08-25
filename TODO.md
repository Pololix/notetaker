# Project-wide:
- Lua API:
    - Make core config driven
    - Allow for custom keybinds and expose default ones in a Lua file (removable)
    - Plugin system
- Populate dev docs and later add docs for contributing

# Command/event system and error handling:
- Each step in the hierarchy should either handle their own events and pass them up
- Populate errors and events with useful context info from which act upon

# Editor functionality: 
- Consider using an array instead of a hash map for workspaces

# Buffer/workspace functionality:
- Add tree validation before mutating
- Allow for dynamic buffer resizing (specially for cursor users)
- Introduce buffer operations (insert, delete, etc)
- Implement movement between buffer views

# Rendering pipeline:
- Figure out the bridge between core and renderer through app
- Avoid redrawing every frame (redraw only when state changes)

# Text rendering: 
- User configs: 
    - See wrap modes (cosmic_text::Wrap)
    - Make scroll either fractional or unitarian
