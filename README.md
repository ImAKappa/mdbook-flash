# mdbook-flash

An mdbook preprocessor for incorporating flashcards and fill-in-the-blanks into your documents

## Getting Started

### Installation

```bash
mdbook-flash install
```

```toml
[preprocessor.flash]
renderers = ["html"]
```

### Usage

Fill-in-the-blanks

```md

# My Fill Note

?`{Mitochondria} are the powerhouse of the cell!`

```

Flashcard

```md

# My Flash Note

??`{-emia} means presence in blood`

```

## TODO

- [ ] Mvp
- [ ] Testing
- [ ] Examples
- [ ] Github CI
- [ ] Documentation site?
