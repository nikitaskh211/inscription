# Inscription

**Inscription** is a lightweight GUI text editor written in Rust, designed with Vim-inspired motions for efficient text editing.

## Overview

Inscription aims to combine the power of Vim-like keybindings with a modern graphical interface. Whether you're a developer, writer, or casual user, Inscription provides a seamless and customizable text editing experience.

---

## Features (Roadmap)

The development of Inscription is organized into three core phases:

- [ ] **Back-end for Text Editing**: Implement robust text manipulation capabilities, including Vim-style motions and commands.
- [ ] **Graphical User Interface**: Build an intuitive and responsive GUI for a smooth user experience.
- [ ] **Plugin Support**: Enable extensibility through a plugin system, allowing users to customize functionality.

---

## Project Structure

Below is the planned file-tree structure for the finished program:

```plaintext
/src
├── /plugins       # Plugin-related modules
├── /fs            # Filesystem abstraction
│   ├── mod.rs     # Module entry point
│   └── vfs.rs     # Virtual filesystem implementation
├── /gui           # Graphical user interface components
│   ├── mod.rs     # Module entry point
│   └── window.rs  # Window management logic
└── main.rs        # Application entry point
