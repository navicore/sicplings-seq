# SICP-lings in Seq

**Structure and Interpretation of Computer Programs** - reimagined in a stack-based language.

> **New to Seq?** Learn the language first with [seqlings](https://github.com/navicore/seqlings) - interactive exercises that teach stack-based programming from the ground up. Come back here once you're comfortable with words, stack effects, and quotations.

This is an experimental project to work through SICP exercises using [Seq](https://github.com/navicore/patch-seq), a concatenative stack-based language. The goal is twofold:

1. **Learn SICP's deep ideas** about computation, abstraction, and interpretation
2. **Exercise Seq** and discover gaps in the language

## Why Seq for SICP?

SICP's core ideas transcend any particular language. Scheme was chosen for its minimal syntax and first-class procedures. Seq offers something similar:

- **Quotations** are first-class (like lambda)
- **Composition** is natural (no parentheses to manage)
- **Stack effects** make data flow explicit
- **Minimalism** - very few special forms

Some things will be harder (no cons cells built-in), some easier (iteration is more natural than in Scheme). The friction is the point - it forces deep understanding.

## Structure

```
exercises/
├── 01-elements/       # 1.1: Elements of Programming
│   ├── 01-square.seq
│   ├── 02-sum-of-squares.seq
│   └── ...
├── 02-procedures/     # 1.2: Procedures and Processes
│   ├── 01-factorial-recursive.seq
│   └── ...
├── 03-higher-order/   # 1.3: Higher-Order Procedures
│   └── ...
```

## Getting Started

```bash
# Build the tool
cargo build --release

# Run in watch mode
cargo run

# Or after installing
sicplings watch
```

## The SICP Connection

Each exercise maps to concepts from SICP:

| SICP Section | Topic | Seq Exploration |
|--------------|-------|-----------------|
| 1.1 | Elements of Programming | Expressions, word definitions, composition |
| 1.1.7 | Square Root (Newton) | Iteration, fixed points |
| 1.2.1 | Linear Recursion | Stack-based recursion patterns |
| 1.2.2 | Tree Recursion | Fibonacci, inefficiency |
| 1.3 | Higher-Order Procedures | Quotations as first-class values |
| 1.3.3 | General Methods | Fixed point, average damping |

## Philosophy

> "The computer revolution is a revolution in the way we think."
> — SICP, Preface

This isn't about translating Scheme to Seq line-by-line. It's about understanding the *ideas* deeply enough to express them in a completely different paradigm.

When the stack gets complicated, remember: **factor into smaller words**.
