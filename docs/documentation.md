.. filepath: /Users/evan/SXsv/docs/documentation.rst

Conventions
===========

Rust
----

We follow standard Rust conventions for code style and organization. Adhering to these conventions ensures readability and maintainability across the codebase. While strict formatting is not enforced, consistency within files and modules is important.

- Use idiomatic Rust naming conventions (``snake_case`` for variables/functions, ``CamelCase`` for types).
- Prefer explicit types and error handling.
- Organize code into logical modules and files.
- Document public functions and types with doc comments (``///``).
- Use ``rustfmt`` for automatic code formatting.

Alignment and whitespace are not critical, as long as a consistent pattern is maintained throughout the project.

Ratatui
-------

All TUI (Text User Interface) application logic related to Ratatui should be organized into clear, modular methods. The main function should serve as a procedural entry point, delegating tasks to well-named helper functions.

**Example structure:**

From SXsv::

    fn main() -> Result<()> {
        sxsv_time();         // Record installation time
        sxsv_setup();        // Create essential SXsv directories

        color_eyre::install()?;
        let terminal = ratatui::init(); // Initialize Ratatui terminal

        // Parse and handle CLI arguments, ensuring terminal restoration
        let result = arguments_sxsv(terminal);

        ratatui::restore();  // Restore terminal state

        result // Propagate any errors
    }

**Application flow:**

Tree structure::

    main()
    ├─ ratatui::init()
    ├─ arguments_sxsv()
    │  ├─ // Match and validate CLI arguments
    │  ├─ parse_args_run() // Dispatch to the correct window/method
    │  │  ├─ run_somewindow()
    │  │  │  ├─ // Rendering loop and key input handling

- Rendering methods are separated into dedicated Rust files (e.g., ``info_menus.rs``, ``editor_csv.rs``).
- Use the naming convention ``run_WINDOW_NAME()`` for rendering functions.
- Keep rendering logic concise. Use only official Ratatui widgets, wrapped in blocks or layouts as needed.
- Separate other components into different files for clarity and ease of maintenance.

Project Structure & Documentation
---------------------------------

- Outside of ``src/``, ``target/``, and ``docs/``, all configuration files, licenses, and README files are kept at the project root.
- Each significant code addition should be accompanied by documentation in a separate file within ``docs/``.
- Maintain a changelog. Every commit should be reflected in the changelog for transparency and tracking.

Additional Guidelines
---------------------

- Write clear, descriptive commit messages.
- Use version control branches for features and bug fixes.
- Review code for readability and adherence to these conventions before merging.