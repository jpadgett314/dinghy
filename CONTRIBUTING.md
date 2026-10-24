# Contributing

Contributions to Dinghy are appreciated. This is a very casual project, but
please do read all guidelines before contributing. Feel free to ask anything
in issues or discussions.

## Coding conventions

- Follow the style of nearby code.
- Keep the Rust focused on logic. Things like layout, styling and UI content
  should be defined in XML templates and CSS where possible.
- Register callbacks and set properties in XML templates when it is possible
  and natural to do so. It can be nice to provide dependencies this way via
  properties. 
- GTK/libadwaita conventions supercede Rust ones. Hacks that subvert GTK 
  patterns do tend to propagate and make everything harder.
- Try to use weak references in move closures (e.g. `glib::clone!`).
- The `widgets` directory is for UI components that are generalized building
  blocks.
- The `views` directory is for specialized UI components. Most will fall under
  this category.
- Keep changes focused and modest. Please discuss larger architectural changes
  first so we can keep the project approachable and easy to maintain.

## Commits

Commit messages should state in present tense the modification made by the 
commit.

## Submitting changes

Please manually test your changes before submitting a pull request. 

One PR should address one problem or improvement, and ideally tags one issue. 

Feel free to assign yourself to an issue or create one before starting work. 

