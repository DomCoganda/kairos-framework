# kairos-framework

The UI framework behind [SeraphOS](https://github.com/your-org/seraph-os) — a privacy-focused
Linux desktop built entirely in Rust.

This is not a general-purpose UI toolkit. It is purpose-built for SeraphOS applications,
designed so that every app in the ecosystem shares the same design language, reactive model,
and component primitives. The renderer can be swapped without touching application code.

---

## Crates

### `mantle`
Design tokens and theming. Defines the shared visual language — colours, borders, typography,
spacing, animation speeds, and palette structures. Every style decision in the framework
flows through mantle.

### `petra`
Renderer-agnostic widget primitives and composites. Defines what widgets *are* — their
structure, properties, and behaviour — with no dependency on any rendering library.
Petra is the stability boundary: it never changes to accommodate renderer limitations.

Includes: `Row`, `Column`, `Text`, `Button`, `Graph`, `Canvas`, `Slider`, `Toggle`,
`Checkbox`, `Dropdown`, `Spinner`, `ProgressBar`, `Scrubber`, `Avatar`, `Divider`, and more.

### `nuntius`
Reactive state and signals. Provides `Signal<T>` for fine-grained reactivity,
`SignalBuffer<T>` for rolling data windows (used by live graphs), and `Broker` for
cross-component communication. Signals are persistent — never recreated per frame.

### `kairos`
The iced-based renderer. This is the swap boundary: it translates petra's widget tree
into iced primitives and owns the application runtime. App code depends only on petra
and nuntius — kairos is an implementation detail.

### `kairos-macros`
Procedural macros for the framework. Provides `#[component]`, `#[primitive]`, and
`#[widget]` to reduce boilerplate when defining new components at any layer of the stack.

---

## Architecture
mantle        →  design tokens

petra         →  widgets (renderer-agnostic)

nuntius       →  signals and reactive state

kairos        →  iced renderer (the swap boundary)

kairos-macros →  proc macros

Application code imports from `kairos`, which re-exports everything from petra and nuntius.
When the renderer changes, only kairos changes — nothing above it.

---

## Status

Active development. Used internally by SeraphOS applications. Not yet published to crates.io.

---

## Part of SeraphOS

SeraphOS is a privacy-focused Linux distribution built entirely in Rust, targeting a clean
and familiar desktop experience with GrapheneOS-level hardening as a long-term goal.
