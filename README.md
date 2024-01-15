# Robespierre

Robespierre is a tool built in RUST and packaged via NPM that helps you find extraneous and implicit dependencies by 
comparing your manifest and actual imports in your code using AST parsing.

## Installation

```bash
# Via npm
npm install robespierre --save-dev

# Via yarn
yarn add -D robespierre

# Via pnpm
pnpm add -D robespierre
```

## Usage

```bash
# List available commands
robespierre --help

# Run robespierre on current dir
robespierre

# Run robespierre and write a json summary
robespierre --report
```

## Build (Rust)

You can build the Robespierre Rust executable yourself, therefore you will need a working Rust and Cargo setup.

In order to compile for your native platform, just run:

```bash
cargo +nightly build --release
```

## Packaging (Npm)

You can then wrap the binary (by using bash):

```bash
./package.sh
```