# MGen service operation

MGen is a Rust web service for generating iOS and Android project scaffolds. The Rust application lives in `service/`, platform templates in `templates/`, and wrapper scripts in `scripts/`. Run the service from its own directory so relative template/script/output paths resolve as expected.

## Start and operate locally

Install Rust/Cargo before running the service. The original Rust setup command is:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This downloads and runs the Rust installer; it is an operator setup step, not part of this repository's offline verification. Review the installation before running it on the intended host. Once Rust/Cargo is available, start from the repository root:

```bash
cd service
cargo run
# Open http://127.0.0.1:3000
```

The web interface starts asynchronous generation, polls generation/build status every second through HTMX, displays generation/build logs and offers generated-project ZIP downloads. Tasks are UUID-keyed in shared concurrent state; that identity/state tracking does not certify a platform build. Platform prerequisites and wrapper commands are maintained in the [Android template guide](../templates/android/README.md#mgen-wrapper) and [iOS template guide](../templates/ios/README.md#mgen-wrapper).

The default output directory is `../generated` when launched from `service/`; the interface can choose another writable directory. Check that setting and filesystem permissions if a generated path is missing. Startup scans existing output folders and imports their status; imported folders are not evidence that their current contents were built or verified. Generation logs are held with task state. Successful generation attempts to save `.mgen_log.txt` in the selected output path (or first available platform path), and startup can reload that file. File-write errors are ignored; failed tasks are not guaranteed a saved log. In-memory task logs remain available while the task state exists. This is not a guarantee of timestamped logs under a root `logs/` directory.

If port 3000 is already occupied, stop the conflicting local service or select another loopback address from `service/`:

```bash
SCAFFOLD_SERVER_ADDR=127.0.0.1:8080 cargo run
```

Keep the service on loopback for a trusted local operator. Its route setup has no authentication layer; changing the bind address alone does not provide safe remote or multi-user access.

## GitHub publication and credential boundary

The web form submits to `POST /github/push`. For a completed generation task with an output path, this operation requests creation of a **private** repository in the authenticated user's GitHub account, then initializes/commits the generated project and pushes `main`. Confirm the chosen project and destination before using the form. Creation can succeed before a later Git step fails; automatic remote rollback is not provided. Existing origin configuration is not overwritten, and an already-clean working tree can still fail at the commit step.

The operator supplies a GitHub token authorized for the intended private-repository creation and push. Resolve authorization through GitHub's account controls; do not put tokens into URLs, command examples, project files or logs. Authentication or permission failures require operator review, rather than repeatedly submitting a real credential to a test. This guide contains no real token or runnable publication example.

The credential-lifetime repair changes newly generated Git operations as follows:

- Origin uses a validated clean `https://github.com/owner/repository.git` URL. The submitted token is not interpolated into Git arguments or that stored origin.
- A command-scoped helper returns the token only for the matching HTTPS GitHub repository. Earlier helpers are reset for this push; the helper ignores store/erase. This does not edit the user's persistent Git configuration.
- The token is supplied only to the push child through `MGEN_GITHUB_TOKEN`. Git tracing and inherited command-config environment overrides are disabled for that operation. Generic and exact-repository redirect settings are set to false.
- API/transport details and Git stdout/stderr are excluded from public error HTML; returned Git messages and success URLs are escaped.

This reduces new argv/origin persistence, not every exposure. The token remains in the incoming request, Rust memory, the push child's environment and the helper-to-Git pipe; memory is not zeroized. Same-user/privileged inspection, debuggers, dumps, hooks, hostile Git/shell/PATH, URL rewrites, transports, proxy/TLS settings and other arbitrary configuration are not sandboxed. Use only a trusted local machine and generated project. Existing generated repositories and historically persisted credentials are not scanned or cleaned by this change; any review or remediation of those requires a separate operator decision.

## Verification scope

Safe local checks for this repair, from `service/`:

```bash
cargo test --locked --offline -- --test-threads=2
cargo build --locked --offline
cargo clippy --all-targets --locked --offline -- -D warnings
```

Offline mode requires dependencies already present in Cargo's cache. The eight local tests use disposable Git/config fixtures and fake token data; they do not call the service routes or real GitHub. Independent probes also exercised mocked push success/failure and credential/config boundaries. These checks do not establish actual API authentication or push, iOS/Android builds, or successful scaffold/download execution. Full `cargo fmt --check` currently has an unchanged baseline formatting finding in `src/utils.rs`; do not report that gate as passing.
