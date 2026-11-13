# Contributing

Contributions to Dinghy are appreciated. This is a very casual project, but
please do read the guidelines before contributing. Feel free to ask anything
in issues or discussions.

## AI Contribution Policy

This project follows the [GTK AI Contribution Policy](https://github.com/GNOME/gtk/blob/main/CONTRIBUTING.md#ai-contribution-policy).

## Conventions and Best Practices

- Follow the style of nearby code.
- GTK conventions supercede Rust ones. 
- Keep the Rust focused on logic. Things like layout, styling and UI content
  should be defined in XML templates and CSS where possible.
- Register signals and bind GtkExpressions in template files. Prefer both over
  manually subscribing to signals with `connect_*`. 
- Use weak references in move closures (e.g. `glib::clone!`).
- Keep changes focused and modest. Please discuss larger architectural changes
  first so we can keep the project approachable and maintainable.

## Project Structure 

- The `views` directory is for specialized UI components. Most will fall under
  this category.
- The `widgets` directory is for UI components that are generalized building
  blocks.

## Commits

Commit messages should state in present tense the modification made by the 
commit.

## Submitting changes

Please manually test your changes before submitting a pull request. 

A single PR should address a single problem or improvement.

Feel free to create an issue before starting work.

