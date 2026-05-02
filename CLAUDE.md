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
   - For changes that affect Linux package managers (apt/dnf/pacman) or distro-specific resolution, run the relevant e2e image — see below.
5. **Summarize** what changed in 1–2 sentences.

## End-to-end tests

Real package-manager behavior is exercised in containers under `tests/e2e/`. CI runs all three on every PR via `.github/workflows/e2e.yml`.

- **Layout**: one Dockerfile per distro (`debian.Dockerfile`, `arch.Dockerfile`, `fedora.Dockerfile`) installs a known package via each supported manager, then runs `tests/e2e/cases/<distro>.sh` as the container's CMD. Each case installs a package via one manager and asserts `how <cmd>` reports that manager (via the `assert_how` helper).
- **Run locally**: `./tests/e2e/run.sh <debian|arch|fedora>` builds the image and runs the assertions. Requires Docker. First build is slow (cold layer cache); reruns are fast.
- **When to run**: any change to `command_resolver.rs`, the `PackageManager` trait, or a Linux-specific manager module (apt, dnf, pacman, snap, and the cross-platform ones to a lesser extent). macOS-only managers (brew on Darwin, MacPorts) aren't covered — smoke-test those manually.
- **When adding a new manager**: add an install step to whichever distro Dockerfile(s) ship it, plus an `assert_how` line to the matching `cases/<distro>.sh`. Pick a package that *only* that manager would install, to avoid cross-attribution noise — except where shadowing is the point of the test.
- **Debugging a failure**: `docker run --rm -it --entrypoint bash how-e2e-<distro>` drops you into the built image with `how` on PATH; rerun the failing `how <cmd>` by hand.

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
