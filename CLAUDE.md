# CLAUDE.md

Operating guide for working in this repo. Project facts (what the tool does, how to install) live in `README.md`.

## What this repo is, in one line

A small Rust CLI (`how <command>`) that reports which package manager(s) installed a given executable. Each supported manager is a module under `src/package_managers/`.

## Core principles

- Read before writing. Confirm assumptions against the code, not from memory.
- Match scope to the request. A bug fix in one package manager module shouldn't ripple into the others.
- Prefer the smallest safe diff. Don't refactor on the side.
- This is a CLI that shells out to other tools (`which`, `apt`, `brew`, ...). Treat subprocess invocation and output parsing as the main correctness surface.

## Operational workflow

1. **Gather context.** For any change touching a package manager, read `src/package_manager.rs` (the trait/abstraction) and the specific module under `src/package_managers/`. For CLI/UX changes, read `src/main.rs`.
2. **Plan briefly** when adding a new package manager or changing the trait — it touches every module. Otherwise just implement.
3. **Implement** with the smallest diff. Follow the existing module shape; don't invent a new pattern for one manager.
4. **Verify**:
   - `cargo build` — must compile clean.
   - `cargo clippy --all-targets -- -D warnings` — lints must pass.
   - `cargo fmt --check` — formatting must match.
   - `cargo test` if tests exist for the area touched.
   - For behavioral changes, run `cargo run -- <command>` against a real command on this machine and confirm the output.
5. **Summarize** what changed in 1–2 sentences.

## Adding a new package manager

The repeatable shape, derived from existing modules:

1. Add `src/package_managers/<name>.rs` implementing the same interface as siblings (mirror `brew.rs` or `cargo.rs` — they're the cleanest references).
2. Wire it into `src/package_managers/mod.rs` (or wherever the existing modules are registered — check `package_manager.rs`).
3. Update `README.md`'s "Supported package managers" list.
4. Smoke-test with `cargo run -- <some-command-installed-by-that-manager>`.

If the new manager needs a different invocation model than the trait supports, raise it before extending the trait — a one-off change in the trait affects every module.

## Routing & delegation

- Use the **Explore** agent only if a question genuinely spans the codebase. This repo is small (~8 modules); usually direct Read/Grep is faster.
- Use the **Plan** agent before changes that touch the trait or every package manager module.
- Use the **`pr`** skill to open PRs.

## Quality bar

- Compile clean, no clippy warnings, formatted.
- Don't swallow errors from subprocess calls — surface them, since the whole tool is "did this command succeed and what did it print."
- Don't add dependencies for trivial functionality. The crate list is intentionally short (`clap`, `nom`, `tokio`, `futures`).
- No `unwrap()` / `expect()` on values that come from subprocess output or user input. Parse failures are expected; handle them.

## Coordination

- Run independent tool calls in parallel (e.g. reading several package manager modules at once).
- `cargo build` on a cold target dir can take a minute; run it in the background if you have other work to do.

## Self-improvement

If you notice a reusable pattern across modules (e.g. all package managers parse `which`-style output the same way), propose extracting it into `package_manager.rs` or a helper, rather than copy-pasting again. Surface the suggestion before implementing.
