# Contributing

Contributions to Dinghy are appreciated. This is a very casual project, but
please do read all guidelines before contributing. Feel free to ask anything
in issues or discussions.

## Coding conventions

- Follow the style of nearby code.
- Keep the Rust focused on logic. Things like layout, styling and UI content
  should be defined in XML templates and CSS where possible.
- The `widgets` directory is for UI components that are generalized building
  blocks.
- The `views` directory is for specialized UI components. Most will fall under
  this category.
- Views should be stateless visualizations of underlying data (a model). 
  Don't store containers of views, store containers of models. Views reactively 
  update to reflect model. Outside of the view layer (or even in it sometimes), 
  modify views via their models.
- GTK/libadwaita conventions supercede Rust ones. Hacks that subvert GTK 
  patterns do tend to propagate and make everything harder.
- Try to use weak references in move closures (e.g. `glib::clone!`).
- It can be nice to provide dependencies via template properties and signals.
- Keep changes focused and modest. Please discuss larger architectural changes
  first so we can keep the project approachable and easy to maintain.

## Commits

Commit messages should state in present tense the modification made by the 
commit.

## Submitting changes

Please manually test your changes before submitting a pull request. 

One PR should address one problem or improvement, and ideally tags one issue. 

Feel free to assign yourself to an issue or create one before starting work. 
