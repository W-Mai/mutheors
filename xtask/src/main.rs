use std::process::Command;
use std::time::{Duration, Instant};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("");

    if cmd == "bump" {
        let level = args.get(1).map(|s| s.as_str()).unwrap_or("");
        return cmd_bump(level);
    }
    if cmd == "publish" {
        let dry_run = args.iter().any(|a| a == "--dry-run");
        return cmd_publish(dry_run);
    }
    if cmd == "release" {
        return cmd_release();
    }

    match cmd {
        "ci" => cmd_ci(),
        "build" => cmd_build(),
        "test" => cmd_test(),
        "lint" => cmd_lint(),
        _ => {
            eprintln!("usage: cargo xtask <ci|build|test|lint|bump <major|minor|patch>|publish [--dry-run]|release>");
            std::process::exit(1);
        }
    }
}

// ── CI ──────────────────────────────────────────────────────────────

fn cmd_ci() -> Result {
    for (name, step) in [
        ("build", cmd_build as fn() -> Result),
        ("test", cmd_test),
        ("lint", cmd_lint),
    ] {
        println!("\n=== xtask: {name} ===");
        step()?;
    }
    println!("\n✅ All CI checks passed.");
    Ok(())
}

fn cmd_build() -> Result {
    cargo(&["build", "--workspace"])?;
    cargo(&["build", "--workspace", "--features", "audio"])
}

fn cmd_test() -> Result {
    cargo(&["test", "--workspace"])?;
    cargo(&["test", "--workspace", "--features", "audio"])
}

fn cmd_lint() -> Result {
    cargo(&["fmt", "--all", "--check"])?;
    cargo(&["clippy", "--workspace", "--features", "audio"])
}

// ── Publish ─────────────────────────────────────────────────────────

fn cmd_publish(dry_run: bool) -> Result {
    ensure_clean_tree()?;
    let mut args = vec!["publish"];
    if dry_run {
        args.push("--dry-run");
    }
    cargo(&args)?;
    let verb = if dry_run { "dry-run" } else { "published" };
    println!("\n  ✅ {verb} mutheors successfully");
    Ok(())
}

// ── Bump ────────────────────────────────────────────────────────────

fn cmd_bump(level: &str) -> Result {
    if !matches!(level, "major" | "minor" | "patch") {
        return Err("usage: cargo xtask bump <major|minor|patch>".into());
    }
    let root = project_root();
    let current = read_version(&root)?;
    let next = bump_version(&current, level)?;
    println!("  → bumping {current} → {next}");

    let toml = format!("{root}/Cargo.toml");
    if rewrite_version(&toml, &next)? {
        println!("  → updated {toml}");
    }

    println!("  ✅ version bumped to {next}");
    println!("  → run: git add Cargo.toml && git commit -m \"🔖 bump version to {next}\"");
    Ok(())
}

// ── Release ─────────────────────────────────────────────────────────

fn cmd_release() -> Result {
    let root = project_root();
    ensure_clean_tree()?;

    let version = read_version(&root)?;
    let tag = format!("v{version}");
    println!("  → releasing {tag}");

    println!("\n  → git push origin main");
    run_cmd("git", &["push", "origin", "main"])?;

    println!("\n  → waiting for CI to pass...");
    wait_for_workflow("ci.yml", Duration::from_secs(20 * 60))?;
    println!("  ✅ CI passed");

    println!("\n  → tagging {tag}");
    tag_and_push(&tag)?;

    println!("\n  → publishing to crates.io...");
    cmd_publish(false)?;

    println!("\n  🎉 released {tag} successfully!");
    Ok(())
}

// ── Helpers ─────────────────────────────────────────────────────────

fn project_root() -> String {
    std::env::var("CARGO_MANIFEST_DIR")
        .map(|d| {
            std::path::Path::new(&d)
                .parent()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .unwrap_or_else(|_| ".".to_string())
}

fn cargo(args: &[&str]) -> Result {
    run_cmd("cargo", args)
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result {
    println!("  → {cmd} {}", args.join(" "));
    let status = Command::new(cmd)
        .args(args)
        .current_dir(project_root())
        .status()
        .map_err(|e| format!("failed to run {cmd}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{cmd} {} failed with {status}", args.join(" ")).into())
    }
}

fn ensure_clean_tree() -> Result {
    let out = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(project_root())
        .output()?;
    if !out.stdout.is_empty() {
        return Err("working tree is not clean — commit all changes first".into());
    }
    Ok(())
}

fn read_version(root: &str) -> Result<String> {
    let content = std::fs::read_to_string(format!("{root}/Cargo.toml"))?;
    content
        .lines()
        .find(|l| l.trim().starts_with("version =") && !l.contains("workspace"))
        .and_then(|l| l.split('"').nth(1))
        .map(|s| s.to_string())
        .ok_or_else(|| "could not find version in Cargo.toml".into())
}

fn rewrite_version(path: &str, next: &str) -> Result<bool> {
    let content = std::fs::read_to_string(path)?;
    let updated = content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("version =") && !trimmed.contains("workspace") {
                replace_first_semver(line, next)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    if updated == content {
        return Ok(false);
    }
    std::fs::write(path, updated)?;
    Ok(true)
}

fn replace_first_semver(line: &str, next: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(start) = rest.find('"') {
        result.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        if let Some(end) = after.find('"') {
            let inner = &after[..end];
            if is_semver(inner) {
                result.push_str(&format!("\"{next}\""));
                rest = &after[end + 1..];
                result.push_str(rest);
                return result;
            }
            result.push('"');
            result.push_str(inner);
            result.push('"');
            rest = &after[end + 1..];
        } else {
            result.push_str(&rest[start..]);
            return result;
        }
    }
    result.push_str(rest);
    result
}

fn is_semver(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

fn bump_version(version: &str, level: &str) -> Result<String> {
    let parts: Vec<u64> = version
        .split('.')
        .map(|p| {
            p.parse::<u64>()
                .map_err(|e| format!("invalid version: {e}"))
        })
        .collect::<std::result::Result<_, _>>()?;
    if parts.len() != 3 {
        return Err(format!("expected semver x.y.z, got {version}").into());
    }
    let (major, minor, patch) = (parts[0], parts[1], parts[2]);
    Ok(match level {
        "major" => format!("{}.0.0", major + 1),
        "minor" => format!("{major}.{}.0", minor + 1),
        "patch" => format!("{major}.{minor}.{}", patch + 1),
        _ => unreachable!(),
    })
}

fn gh(args: &[&str]) -> Result<String> {
    let out = Command::new("gh")
        .args(args)
        .output()
        .map_err(|e| format!("gh {}: {e}", args.join(" ")))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(format!(
            "gh {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        )
        .into())
    }
}

fn wait_for_workflow(workflow: &str, timeout: Duration) -> Result<()> {
    let start = Instant::now();
    println!("  → waiting for {workflow} ...");
    loop {
        std::thread::sleep(Duration::from_secs(15));
        let out = gh(&[
            "run",
            "list",
            "--workflow",
            workflow,
            "--limit",
            "1",
            "--json",
            "status,conclusion",
            "-q",
            ".[0] | [.status, .conclusion] | @tsv",
        ])?;
        let parts: Vec<&str> = out.split('\t').collect();
        let status = parts.first().copied().unwrap_or("");
        let conclusion = parts.get(1).copied().unwrap_or("");
        println!("    {workflow}: {status} / {conclusion}");
        if status == "completed" {
            return if conclusion == "success" {
                Ok(())
            } else {
                Err(format!("{workflow} completed with: {conclusion}").into())
            };
        }
        if start.elapsed() > timeout {
            return Err(format!("timeout waiting for {workflow}").into());
        }
    }
}

fn tag_and_push(tag: &str) -> Result {
    let root = project_root();
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(&root)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    let tag_commit = Command::new("git")
        .args(["rev-list", "-n1", tag])
        .current_dir(&root)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    if !tag_commit.is_empty() && tag_commit == head {
        println!("  → tag {tag} already points to HEAD, skipping");
        return Ok(());
    }

    run_cmd("git", &["tag", tag])?;
    run_cmd("git", &["push", "origin", tag])
}
