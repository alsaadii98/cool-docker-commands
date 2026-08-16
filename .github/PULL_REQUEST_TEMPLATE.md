## What this changes

<!-- One or two sentences. Link the issue if there is one. -->

## Output before / after

<!-- Paste terminal output for any change that alters what dok prints. -->

## Checklist

- [ ] `cargo fmt --all`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo test`
- [ ] Colours come from palette roles (`p().green`), not hardcoded RGB
- [ ] Structural glyphs come from `g()`, so the `ascii` theme stays ASCII
- [ ] README / ROADMAP / CHANGELOG updated if user-facing
