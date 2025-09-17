# Publishing Checklist

This page tracks tasks to publish and maintain the mdBook.

- [x] Create `book/book.toml` and `book/src` with chapter structure.
- [x] Write chapters for grammar, components, artifacts, limits, validation, authoring, contract, schemas, testing.
- [ ] CI job: build mdBook on PRs and main.
- [ ] Optional: Deploy to GitHub Pages from main.

Local build (requires mdbook):

```bash
cargo install mdbook
mdbook build book
open book/book/index.html
```

CI recommendations:
- Add a job that runs `cargo install mdbook` and `mdbook build book` and uploads the `book/book/` as an artifact.
- Protect against external network fetches; fonts should be local.
