<div align="center">

# 🇵🇹 pizza-analysis-portuguese

**Portuguese text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--portuguese-blue)](https://github.com/pizza-rs/analysis-portuguese)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

European Portuguese language analysis with light stemming and stop words.
For Brazilian Portuguese, see `pizza-analysis-brazilian`.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `portuguese_light_stem` | Portuguese light stemmer |
| TokenFilter | `portuguese_stop` | Portuguese stop words (203 entries) |
| Analyzer | `portuguese` | Full pipeline: lowercase → light_stem → stop |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_portuguese::register_all(&mut factory);

let analyzer = factory.get_analyzer("portuguese").unwrap();
// "universidades" → "universidade"
```

## Installation

```toml
[dependencies]
pizza-analysis-portuguese = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["portuguese"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
