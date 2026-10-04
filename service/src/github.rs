//! GitHub publishing without storing a token in Git configuration or arguments.

use std::path::Path;
use std::process::{Command, Output};

// Git appends the operation name to this shell snippet. Only `get` returns a
// credential; `store` and `erase` deliberately do nothing. Keep the token in the
// environment, never interpolate it into this snippet (which appears in argv).
const CREDENTIAL_HELPER: &str = r#"!f() {
    test "$1" = get || return 0
    protocol= host= path=
    while IFS= read -r line && test -n "$line"; do
        case "$line" in
            protocol=*) protocol=${line#protocol=} ;;
            host=*) host=${line#host=} ;;
            path=*) path=${line#path=} ;;
        esac
    done
    if test "$protocol" = https && test "$host" = github.com && test "$path" = "$MGEN_GITHUB_PATH"; then
        printf '%s\n' 'username=oauth2' "password=$MGEN_GITHUB_TOKEN"
    else
        printf '%s\n' 'quit=1'
    fi
}; f"#;

pub(crate) fn validate_token(token: &str) -> Result<(), String> {
    if token.is_empty() || token.chars().any(char::is_control) {
        return Err("Invalid GitHub token".to_string());
    }
    Ok(())
}

fn github_path(clone_url: &str) -> Result<&str, String> {
    let invalid = || "Invalid GitHub clone URL".to_string();
    let path = clone_url
        .strip_prefix("https://github.com/")
        .ok_or_else(invalid)?;
    let (owner, repo) = path.split_once('/').ok_or_else(invalid)?;
    let name = repo.strip_suffix(".git").ok_or_else(invalid)?;
    let valid_part = |part: &str| {
        !part.is_empty()
            && part != "."
            && part != ".."
            && part
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
    };
    if !valid_part(owner) || !valid_part(name) {
        return Err(invalid());
    }
    Ok(path)
}

fn authenticate_push(command: &mut Command, repo_path: &str, token: &str) {
    // Reset inherited helpers so Git cannot send `store` to a persistent helper.
    // These -c settings exist only for this command and its Git children.
    command.args([
        "-c",
        "credential.helper=",
        "-c",
        &format!("credential.helper={CREDENTIAL_HELPER}"),
        "-c",
        "credential.useHttpPath=true",
        "-c",
        "core.askPass=",
        "-c",
        "http.followRedirects=false",
        // URL-specific values beat the generic setting, even from config files.
        "-c",
        &format!("http.https://github.com/{repo_path}.followRedirects=false"),
    ]);
    // Do not let inherited diagnostics record credential environment values or
    // HTTP authentication. This does not sandbox Git, hooks, or same-user tools.
    for key in std::env::vars_os().map(|(key, _)| key) {
        if key.to_string_lossy().starts_with("GIT_TRACE") {
            command.env_remove(key);
        }
    }
    command
        // Trace2 initializes before -c options, so disable its targets via env.
        .env("GIT_TRACE2", "0")
        .env("GIT_TRACE2_EVENT", "0")
        .env("GIT_TRACE2_PERF", "0")
        .env_remove("GIT_CURL_VERBOSE")
        .env_remove("GIT_CONFIG_PARAMETERS")
        .env("GIT_CONFIG_COUNT", "0")
        .env("GIT_ASKPASS", "")
        .env("SSH_ASKPASS", "")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("MGEN_GITHUB_PATH", repo_path)
        .env("MGEN_GITHUB_TOKEN", token);
}

pub(crate) fn push_project(path: &Path, clone_url: &str, token: &str) -> Result<(), String> {
    push_project_with(path, clone_url, token, |command| command.output())
}

fn push_project_with(
    path: &Path,
    clone_url: &str,
    token: &str,
    mut run: impl FnMut(&mut Command) -> std::io::Result<Output>,
) -> Result<(), String> {
    validate_token(token)?;
    let repo_path = github_path(clone_url)?;
    let steps: &[&[&str]] = &[
        &["init"],
        &["add", "."],
        &["commit", "-m", "Initial commit from MGen"],
        &["branch", "-M", "main"],
        &["remote", "add", "origin", clone_url],
        &["push", "-u", "origin", "main"],
    ];
    for args in steps {
        let mut command = Command::new("git");
        command.current_dir(path);
        if args[0] == "push" {
            authenticate_push(&mut command, repo_path, token);
        }
        command.args(*args);
        let output = run(&mut command).map_err(|_| format!("Could not start git {}", args[0]))?;
        if !output.status.success() {
            // Child diagnostics can contain credentials (including encoded ones).
            // Return the failed step/status, never raw Git stdout or stderr.
            return Err(format!("git {} failed ({})", args[0], output.status));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use std::process::Stdio;
    use std::sync::atomic::{AtomicU64, Ordering};

    const FAKE_TOKEN: &str = "MGEN_FAKE_ONLY_$(never-execute);'quoted'";
    const CLONE_URL: &str = "https://github.com/fixture-owner/fixture-repo.git";
    const REPO_PATH: &str = "fixture-owner/fixture-repo.git";
    static NEXT: AtomicU64 = AtomicU64::new(0);

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "mgen-git-test-{}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            fs::create_dir(path.join("home")).unwrap();
            fs::write(path.join("project.txt"), "disposable test project\n").unwrap();
            Self(path)
        }

        fn isolate(&self, command: &mut Command) {
            // Preserve only explicit command settings, not machine credentials,
            // config, hooks, or proxy settings. No Git network transport is allowed.
            let env: Vec<_> = command
                .get_envs()
                .map(|(k, v)| (k.to_owned(), v.map(|v| v.to_owned())))
                .collect();
            command
                .env_clear()
                .env("PATH", "/usr/bin:/bin")
                .env("HOME", self.0.join("home"))
                .env("GIT_CONFIG_NOSYSTEM", "1")
                .env("GIT_CONFIG_GLOBAL", self.0.join("home/.gitconfig"))
                .env("GIT_ALLOW_PROTOCOL", "")
                .env("GIT_AUTHOR_NAME", "MGen fixture")
                .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
                .env("GIT_COMMITTER_NAME", "MGen fixture")
                .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid");
            for (key, value) in env {
                if let Some(value) = value {
                    command.env(key, value);
                }
            }
        }

        fn git(&self, args: &[&str]) -> Output {
            let mut command = Command::new("git");
            command.current_dir(&self.0).args(args);
            self.isolate(&mut command);
            command.output().unwrap()
        }

        fn credential(&self, operation: &str, input: &str) -> Output {
            assert!(["fill", "approve", "reject"].contains(&operation));
            let mut command = Command::new("git");
            command.current_dir(&self.0);
            authenticate_push(&mut command, REPO_PATH, FAKE_TOKEN);
            command.args(["credential", operation]);
            self.isolate(&mut command);
            command
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            let mut child = command.spawn().unwrap();
            child
                .stdin
                .take()
                .unwrap()
                .write_all(input.as_bytes())
                .unwrap();
            child.wait_with_output().unwrap()
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    fn fake_output(success: bool, stderr: &[u8]) -> Output {
        // `true`/`false` provide portable ExitStatus values without a network call.
        let status = Command::new(if success { "/bin/true" } else { "/bin/false" })
            .status()
            .unwrap();
        Output {
            status,
            stdout: Vec::new(),
            stderr: stderr.to_vec(),
        }
    }

    #[test]
    fn only_push_receives_token_and_no_argument_contains_it() {
        let mut calls = 0;
        push_project_with(Path::new("."), CLONE_URL, FAKE_TOKEN, |command| {
            let args: Vec<_> = command
                .get_args()
                .map(|arg| arg.to_string_lossy())
                .collect();
            assert!(args.iter().all(|arg| !arg.contains(FAKE_TOKEN)));
            let token = command
                .get_envs()
                .find(|(key, _)| *key == OsStr::new("MGEN_GITHUB_TOKEN"));
            if args.contains(&"push".into()) {
                assert_eq!(token.unwrap().1, Some(OsStr::new(FAKE_TOKEN)));
                assert!(args.contains(&"credential.helper=".into()));
                assert!(args.contains(&"http.followRedirects=false".into()));
                assert!(args.contains(&format!("http.{CLONE_URL}.followRedirects=false").into()));
            } else {
                assert!(token.is_none());
            }
            if args.first() == Some(&"remote".into()) {
                assert_eq!(args.last().unwrap(), CLONE_URL);
            }
            calls += 1;
            Ok(fake_output(true, b""))
        })
        .unwrap();
        assert_eq!(calls, 6);
    }

    #[test]
    fn redirects_remain_disabled_under_inherited_url_specific_config() {
        // `config --get-urlmatch` resolves the same HTTP URL-specific options
        // without issuing any HTTP request or invoking a credential helper.
        for scope in [
            None,
            Some("https://github.com/"),
            Some("https://github.com/fixture-owner/"),
            Some(CLONE_URL),
        ] {
            let fixture = Fixture::new();
            if let Some(scope) = scope {
                fs::write(
                    fixture.0.join("home/.gitconfig"),
                    format!("[http \"{scope}\"]\n\tfollowRedirects = true\n"),
                )
                .unwrap();
                // Prove the fixture overrides the old generic command option.
                let old = fixture.git(&[
                    "-c",
                    "http.followRedirects=false",
                    "config",
                    "--get-urlmatch",
                    "http.followRedirects",
                    CLONE_URL,
                ]);
                assert!(old.status.success());
                assert_eq!(String::from_utf8(old.stdout).unwrap().trim(), "true");
            }
            let mut command = Command::new("git");
            command.current_dir(&fixture.0);
            authenticate_push(&mut command, REPO_PATH, FAKE_TOKEN);
            command.args([
                "config",
                "--get-urlmatch",
                "http.followRedirects",
                CLONE_URL,
            ]);
            fixture.isolate(&mut command);
            let output = command.output().unwrap();
            assert!(output.status.success());
            assert_eq!(
                String::from_utf8(output.stdout).unwrap().trim(),
                "false",
                "inherited scope: {scope:?}"
            );
            assert!(output.stderr.is_empty());
        }
    }

    #[test]
    fn local_repository_remains_token_free_on_push_success_and_failure() {
        for success in [true, false] {
            let fixture = Fixture::new();
            let mut pushed = false;
            let result = push_project_with(&fixture.0, CLONE_URL, FAKE_TOKEN, |command| {
                let args: Vec<_> = command
                    .get_args()
                    .map(|arg| arg.to_string_lossy().into_owned())
                    .collect();
                if args.iter().any(|arg| arg == "push") {
                    pushed = true;
                    return Ok(fake_output(
                        success,
                        format!("fake diagnostic: {FAKE_TOKEN}").as_bytes(),
                    ));
                }
                assert!(["init", "add", "commit", "branch", "remote"].contains(&args[0].as_str()));
                fixture.isolate(command);
                command.output()
            });
            assert!(pushed);
            assert_eq!(result.is_ok(), success);
            if let Err(error) = result {
                assert!(!error.contains(FAKE_TOKEN));
                assert!(error.contains("git push failed"));
            }
            let config = fs::read_to_string(fixture.0.join(".git/config")).unwrap();
            assert!(config.contains(CLONE_URL));
            assert!(!config.contains(FAKE_TOKEN));
            assert!(!config.contains("credential"));
            assert!(!config.contains("oauth2"));
            assert!(!fixture.0.join("home/.git-credentials").exists());
        }
    }

    #[test]
    fn credential_helper_is_exactly_scoped_and_has_no_store_or_erase_effects() {
        let fixture = Fixture::new();
        assert!(fixture.git(&["init"]).status.success());
        // A configured helper would create this marker if reset did not work.
        let helper = "!f() { printf called >> helper-called; }; f";
        assert!(fixture
            .git(&["config", "credential.helper", helper])
            .status
            .success());
        assert!(fixture
            .git(&["config", "credential.https://github.com.helper", helper])
            .status
            .success());
        let matching = format!("protocol=https\nhost=github.com\npath={REPO_PATH}\n\n");
        let output = fixture.credential("fill", &matching);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let credential = String::from_utf8(output.stdout).unwrap();
        assert!(credential.contains("username=oauth2\n"));
        assert!(credential.contains(&format!("password={FAKE_TOKEN}\n")));
        for request in [
            format!("protocol=http\nhost=github.com\npath={REPO_PATH}\n\n"),
            format!("protocol=https\nhost=github.com.evil.invalid\npath={REPO_PATH}\n\n"),
            format!("protocol=https\nhost=github.com:443\npath={REPO_PATH}\n\n"),
            "protocol=https\nhost=github.com\npath=fixture-owner/other.git\n\n".to_string(),
            "protocol=https\nhost=github.com\n\n".to_string(),
        ] {
            let output = fixture.credential("fill", &request);
            assert!(!output.status.success());
            assert!(!String::from_utf8_lossy(&output.stdout).contains(FAKE_TOKEN));
            assert!(!String::from_utf8_lossy(&output.stderr).contains(FAKE_TOKEN));
        }
        for operation in ["approve", "reject"] {
            let output = fixture.credential(operation, &credential);
            assert!(output.status.success());
            assert!(output.stdout.is_empty());
        }
        assert!(!fixture.0.join("helper-called").exists());
        assert!(!fixture.0.join(".git/helper-called").exists());
        assert!(!fs::read_to_string(fixture.0.join(".git/config"))
            .unwrap()
            .contains(FAKE_TOKEN));
    }

    #[test]
    fn untrusted_urls_and_control_characters_fail_before_any_subprocess() {
        for url in [
            "",
            "http://github.com/a/b.git",
            "https://oauth2:fake@github.com/a/b.git",
            "https://github.com.evil.invalid/a/b.git",
            "https://github.com:443/a/b.git",
            "https://github.com/a/b.git?query",
            "https://github.com/a/b.git#fragment",
            "https://github.com/a/b/c.git",
            "https://github.com/../b.git",
            "https://github.com/a/%62.git",
            "file:///tmp/fixture.git",
        ] {
            assert!(
                push_project_with(Path::new("."), url, FAKE_TOKEN, |_| panic!("must not run"))
                    .is_err()
            );
        }
        for token in ["", "fake\npassword=second", "fake\r", "fake\0"] {
            assert!(
                push_project_with(Path::new("."), CLONE_URL, token, |_| panic!("must not run"))
                    .is_err()
            );
        }
    }

    #[test]
    fn every_failed_step_stops_and_never_returns_raw_diagnostics() {
        for fail_at in 0..6 {
            let mut calls = 0;
            let error = push_project_with(Path::new("."), CLONE_URL, FAKE_TOKEN, |_| {
                let success = calls != fail_at;
                calls += 1;
                Ok(fake_output(
                    success,
                    format!("<script>{FAKE_TOKEN}</script>").as_bytes(),
                ))
            })
            .unwrap_err();
            assert_eq!(calls, fail_at + 1);
            assert!(!error.contains(FAKE_TOKEN));
            assert!(!error.contains("<script>"));
        }
        let error = push_project_with(Path::new("."), CLONE_URL, FAKE_TOKEN, |_| {
            Err(std::io::Error::other(FAKE_TOKEN))
        })
        .unwrap_err();
        assert!(!error.contains(FAKE_TOKEN));
    }

    #[test]
    fn existing_origin_failure_does_not_push_or_disclose_its_diagnostics() {
        let fixture = Fixture::new();
        assert!(fixture.git(&["init"]).status.success());
        assert!(fixture
            .git(&["remote", "add", "origin", CLONE_URL])
            .status
            .success());
        let result = push_project_with(&fixture.0, CLONE_URL, FAKE_TOKEN, |command| {
            assert!(!command.get_args().any(|arg| arg == "push"));
            fixture.isolate(command);
            command.output()
        });
        assert!(result.unwrap_err().contains("git remote failed"));
        assert!(!fs::read_to_string(fixture.0.join(".git/config"))
            .unwrap()
            .contains(FAKE_TOKEN));
    }

    #[test]
    fn inherited_diagnostics_and_askpass_are_disabled() {
        if std::env::var_os("MGEN_DIAGNOSTIC_CHILD").is_none() {
            let output = Command::new(std::env::current_exe().unwrap())
                .args([
                    "inherited_diagnostics_and_askpass_are_disabled",
                    "--nocapture",
                ])
                .env_clear()
                .env("PATH", "/usr/bin:/bin")
                .env("MGEN_DIAGNOSTIC_CHILD", "1")
                .env("GIT_TRACE", "1")
                .env("GIT_TRACE_CURL", "1")
                .env("GIT_TRACE_REDACT", "0")
                .env("GIT_TRACE2_EVENT", "1")
                .env("GIT_TRACE2_ENV_VARS", "MGEN_GITHUB_TOKEN")
                .env("GIT_CURL_VERBOSE", "1")
                .env("GIT_ASKPASS", "must-not-run")
                .env("SSH_ASKPASS", "must-not-run")
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(!String::from_utf8_lossy(&output.stdout).contains(FAKE_TOKEN));
            assert!(
                output.stderr.is_empty(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        let fixture = Fixture::new();
        let trace = fixture.0.join("trace.log");
        fs::write(fixture.0.join("home/.gitconfig"), format!(
            "[credential]\n\thelper = !printf called > \\\"$HOME/helper-called\\\"\n\
             [credential \"https://github.com\"]\n\thelper = !printf called > \\\"$HOME/scoped-helper-called\\\"\n\
             [core]\n\taskPass = must-not-run\n\
             [trace2]\n\tnormalTarget = {0}\n\teventTarget = {0}\n\tperfTarget = {0}\n\tenvVars = MGEN_GITHUB_TOKEN\n",
            trace.display()
        )).unwrap();
        let matching = format!("protocol=https\nhost=github.com\npath={REPO_PATH}\n\n");
        let filled = fixture.credential("fill", &matching);
        assert!(filled.status.success());
        assert!(filled.stderr.is_empty());
        let credential = String::from_utf8(filled.stdout).unwrap();
        assert!(credential.contains(FAKE_TOKEN));
        for operation in ["approve", "reject"] {
            let output = fixture.credential(operation, &credential);
            assert!(output.status.success());
            assert!(output.stderr.is_empty());
        }
        let denied = fixture.credential(
            "fill",
            "protocol=https\nhost=other.invalid\npath=other/repo.git\n\n",
        );
        assert!(!denied.status.success());
        assert!(!String::from_utf8_lossy(&denied.stderr).contains("must-not-run"));
        assert!(!fixture.0.join("home/helper-called").exists());
        assert!(!fixture.0.join("home/scoped-helper-called").exists());
        assert!(!trace.exists());

        // Negative control: without per-push overrides these same fixture
        // settings really invoke both persistent helpers and create a trace.
        let mut command = Command::new("git");
        command
            .current_dir(&fixture.0)
            .args(["credential", "approve"]);
        fixture.isolate(&mut command);
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn().unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(credential.as_bytes())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        assert!(fixture.0.join("home/helper-called").exists());
        assert!(fixture.0.join("home/scoped-helper-called").exists());
        assert!(trace.exists());
    }
}
