# Copper

Copper is a PCB EDA Tool written in Rust.

![Screenshot](doc/img/screenshot.png)

# Motivation

Coming from an electrical engineering background, we have used quite a few PCB EDA tools.
Some of them are expensive and provide a lot of tools, that can work great or still be buggy.
Some of them are open source but don't really seem to make progress in terms of neither stability nor features.

We genuinely think that this is not a misery coming from incompetent developers, but from a really old code base that is just lackluster and rusty (ha ha).

Thus we decided to try and write our own tooling.

# Current state

Currently a basic [KiCad](http://kicad.org/) schema can be rendered. Text rendering is disabled for now as the development of [Pathfinder 2 by pcwalton](https://github.com/pcwalton/pathfinder) is awaited.

A basic UI with just some info, made with GTK3, is working.

Placing, moving and rotating components works. Drawing new wires works as well.

I am working on [a standalone hierarchical primitive renderer](https://github.com/Yatekii/svg) at the moment, to make rendering more convenient. Also I am trying to push [azul](https://github.com/maps4print/azul) to get a proper, native UI library.

So if you'd like to contribute, please help those two projects.

# Roadmap

The idea is to complete a schema renderer first with regards to the PCB layouter in the future when it comes to performance questions.
To start the project off, the idea is to use the KiCad formats, such that KiCad as a tool can be replaced piece by piece.

1. Schema renderer
    - Text rendering on schema
        - Adding & removing components
    - Electrical logic correctness
        - Tracking and validation of connections between components
            - With regards to netlist generation
    - Export the schema back to the KiCad format
    - Store to a modern format (XML, JSON, YAML, TOML, RON, et al.) to maintain better readability, maintainability and serializing
    - Generate a Netlist in the KiCad format
    - (optional) Export to a modern format

2. Layouter
    - Basic rendering
    - more to come

# Project Structure

The project consists of
    
- a library which contains all the operations for parsing, rendering and manipulation. The API to manipulate the designs programmatically should be a first class citizen from the get go to enable tests to the very last layer before the GUI and future possibilities of scripting and a console. A few remarks:
    - The library is thematically structured into
        - `drawing` (everything involved in the process of drawing the designs)
        - `loading` (everything involved in the process of loading the designs from various formats)
        - `parsing` (everything involved in the process of parsing the project design files)
        - `state` (contains all structures concerning the storing of the designs as well as their manipulation)
        - `utils` (helpers to make the coders life easier)
        - `geometry.rs` (for the moment contains various types; not sure if really need on the top level
    - For KiCad formats parsing, [nom](https://github.com/Geal/nom) is used. For a future modern format, [https://github.com/serde-rs/serde](serde) can be used.
    - No UI code should go in here.
- a binary, which renders the schemata. This binary should only contain GUI code + glue which interfaces the copper library to draw and manipulate the schemata.
    - For GUI [https://github.com/antoyo/relm](relm) is used, which is based on [https://github.com/gtk-rs/gtk](gtk-rs).
- future binaries will be added for different tasks.

## Schema Editor

`SchemaEditor` loads, holds and stores the schema
- `Schema` represents the state of the schema with all its components; does not contain any viewing information. Holds a vector of components and wires. It does not contain any manipulation methods itself.
    - `ComponentInstance` represents a single component in its logical meaning.
    - Each element (Components, Wires, Drawables, etc.) gets a UUID on creation. All entities are accessed via their UUID.
- `ViewState` holds the current view of the schema, cursor position, etc. It should also hold information about selected components, wires or wire preview, etc.
- `SchemaLoader(&schema)` loads the schema
- `SchemaViewer(&schema, &view_stae)` manipulates the current view of the schema; holds the current view state; does not manipulate the schema
- `SchemaDrawer(&schema, &view_state)` daws the schema; does not manipulate the schema. Contains a single Vector of Drawables.
    - Updates the Drawables only on changes.
    - Swaps removed Elemnts for the last one and changes indices of its vertices.
    - Data will only get pushed to the GPU when it changes. Not each frame as before
    - Each Drawable holds a UUID which matches the UUID of an element in the Schema.

- Actions are perfromed using the `EventMessage` enum, the `EventBus` struct and the `Listener` trait.
    - `Listener` is an enum which holds a variant for every single possible change of the Schema or its view. For example `EventMessage::AddComponent(ComponentInstance)` or `EventMessage::Zoom(f32)`.
    - Each `EventMessage` can be emitted using `EventBus::emit(EventMessage)` which internally pushes the `EventMessage` onto a stack. Simultaneously it calls `Listener::receive(EventMessage)` of each known `Listener`. When reverting an action, `Listener::revert(EventMessage)` will be executed in the same fashion and the `EventMessage` will be popped from the stack.
    - `Listener` is a trait implementing `Listenr::receive(EventMessage)` and `Listener::revert(EventMessage)` which apply and revert an action on a given actor respectively. It is meant that the `Listener` matches the enum variants it needs and discards the rest.
- No schema state is directly manipulated. Every action is started via the `MessageBus`. States that only concern a preview can be manipulated with methods directly (selected component marker display, wire preview, etc.).

# Building

## Building for Linux

Make sure `libgtk3` and `libepoxy` are installed on your system.
Make sure you have the current nightly rustc enabled. You can do so with

```
rustup override set nightly
rustup update
```

Inside the repository.
Finally you should be able to

```
cargo run --release --bin schema_editor test_data/kicad.lib test_data/kicad.sch
```

## Building for Windows

Set the active rustup toolchain using

```
rustup target add x86_64-pc-windows-gnu
rustup override set nightly-x86_64-pc-windows-gnu
rustup update
```

Install [MSYS2](http://www.msys2.org/).

In the “MSYS2 MSYS Shell” install libgtk3.

```
pacman -S mingw-w64-x86_64-gtk3
```

Then install the mingw toolchain

```
pacman -S mingw-w64-x86_64-toolchain
```

Start the “MSYS2 MinGW Shell” (not to be confused with “MSYS2 MSYS Shell”).
All future operations should be performed from this shell.

Add the rustup/cargo binaries to the `PATH`.
You can do this by adding the line

```
export PATH=$PATH:/c/Users/<your_name>/.cargo/bin:
```

to the `.bashrc`.
Restart the shell.

Now make sure the MYSYS2 environment is up to date (weirdly enough this is not the case after doing a fresh install). If you are prompted about conflicts between packages, just accept that pacman resolves the conflicts by uninstalling one.

```
pacman -Su
```

Restart the shell again (You might have to force close it, that's okay!)

```
pacman -Su
```

again. Now a ton of packages should get listed. Install them.

Now you should be able to

```
cargo run --release --bin schema_editor test_data/kicad.lib test_data/kicad.sch
```

# Contributing

If you would like to contribute code, ideas or criticism, please do not hesitate to do so.
We do not think that we already know how to perfectly write this application, so we are very open for big adjustments as well.

For now there is no "official process" of contruibuting code, so please just open issues and pull requests at will ;)

# Contact

Yatekii can be found on the mozilla and freenode IRC networks with the same nick, where he wanders in various rust and general programming related channels.
Sometimes tiwalun can be found there too if you get lucky.
