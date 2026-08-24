# Project-wide:
- Lua API:
    - Make core config driven
    - Allow for custom keybinds and expose default ones in a Lua file (removable)
    - Plugin system

# Command/event system and error handling:
- Each step in the hierarchy should either handle their own events and pass them up
- Act upon events and errors
- Populate errors and events with useful context info

# Buffer/workspace functionality:
- Add tree validation before mutating
- Allow for dynamic buffer resizing (specially for cursor users)
- Introduce buffer operations (insert, delete, etc)
- Implement movement between buffer views

# Rendering pipeline:
- Figure out the bridge between core and renderer through app
