# Wasabi_js: A JavaScript binding for Wasabi

This is a small package that compiles Wasabi to Wasm and make it usable from Web and node.js.

## Prerequisites

You need [wasm-pack](https://github.com/rustwasm/wasm-pack) installed to build the package.

Follow the instructions in the link above to install wasm-pack.

## Usage

First, install required dependencies:

```bash
npm install
```

To make npm pacakge:

```bash
npm run build
```

To run some tests:

```bash
npm run test
```

To run the demo:

```bash
npm run demo
```

To instrument wasm code:

```bash
npm run instrument -- {OPTIONS} <input.wasm>
```