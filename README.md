# Card Game Description Language (CGDL)

A Domain-Specific Language (DSL) for describing, validating, and executing card game logic, with built-in tooling for analysis, visualization, and editor integration.

---

## Overview

CGDL provides a structured way to define card games, capturing rules, stages, actions, and transitions in a formal, machine-readable format. The system includes:

- **Parser & AST:** Written in Rust using [Pest](https://pest.rs/) for grammar parsing. Semantic information is captured in a manually defined AST with spans for diagnostics.
- **IR (Intermediate Representation):** AST is transformed into a graph-based FSM (Finite State Machine) to model game flow.
- **Validation:** Multi-layered validation ensures syntactic correctness, semantic consistency, and structural soundness of the FSM.
- **Tooling:** VS Code extension with LSP support provides auto-completion, hover diagnostics, and one-click game execution.
- **Output Artifacts:** JSON IR for backend execution, `.dot` and `.png` files for visualizing game flows.

---

## Features

- **Extensible AST and Parser:** Easily add new language constructs.
- **Spanned AST for Diagnostics:** Every AST node tracks source spans for LSP feedback.
- **FSM IR:** Fully analyzable and visualizable intermediate representation.
- **Property-Based Testing:** Ensures grammar correctness and prevents ambiguities.
- **VS Code Integration:** Write, validate, and run games directly from the editor.

---

## Getting Started

### Prerequisites

- Rust 1.70+  
- Node.js 18+ (for building VS Code extension)  
- VS Code (latest stable release)

### Build Front-End

```bash
cd CardGameDSL
cargo build
cargo test