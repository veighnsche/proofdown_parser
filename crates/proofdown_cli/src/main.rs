use std::{env, fs, io::Read};

use anyhow::{bail, Result};
use proofdown_parser::{parse, parse_with_limits, ParserLimits};
use proofdown_validate::{validate, Limits};
use serde_json::json;
use clap::{Arg, Command};
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_complete::generate;

fn usage() -> &'static str {
    "Usage:\n  pml [--help|-h] [--version|-V]\n  pml completions <bash|zsh|fish|powershell|elvish>\n  pml parse <file> [--json] [--pretty] [--quiet] [--verbose] [--limits.depth=N] [--limits.nodes=N] [--limits.input-size=BYTES]\n  pml validate <file> [--json] [--pretty] [--quiet] [--verbose] [--limits.depth=N] [--limits.nodes=N] [--limits.input-size=BYTES]\n\nNotes:\n  - Use <file> = '-' to read from stdin (UTF-8).\n  - --quiet suppresses human-mode success output; JSON output unaffected.\n  - --verbose prints diagnostic info to stderr (limits applied, sizes).\n\nExit codes:\n  0 OK\n  1 Parse error\n  2 Validation error\n  3 IO/usage error\n"
}

fn build_cli_for_completions() -> Command {
    Command::new("pml")
        .about("Proofdown CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("parse")
                .arg(Arg::new("file").required(true))
                .arg(Arg::new("json").long("json"))
                .arg(Arg::new("pretty").long("pretty"))
                .arg(Arg::new("quiet").long("quiet"))
                .arg(Arg::new("verbose").long("verbose"))
                .arg(Arg::new("limits.depth").long("limits.depth").num_args(1))
                .arg(Arg::new("limits.nodes").long("limits.nodes").num_args(1))
                .arg(Arg::new("limits.input-size").long("limits.input-size").num_args(1)),
        )
        .subcommand(
            Command::new("validate")
                .arg(Arg::new("file").required(true))
                .arg(Arg::new("json").long("json"))
                .arg(Arg::new("pretty").long("pretty"))
                .arg(Arg::new("quiet").long("quiet"))
                .arg(Arg::new("verbose").long("verbose"))
                .arg(Arg::new("limits.depth").long("limits.depth").num_args(1))
                .arg(Arg::new("limits.nodes").long("limits.nodes").num_args(1))
                .arg(Arg::new("limits.input-size").long("limits.input-size").num_args(1)),
        )
}

fn cmd_completions(mut args: Vec<String>) -> Result<()> {
    if args.is_empty() {
        eprintln!("Usage: pml completions <bash|zsh|fish|powershell|elvish>");
        bail!("completions: missing shell");
    }
    let shell = args.remove(0);
    let mut cmd = build_cli_for_completions();
    match shell.as_str() {
        "bash" => generate(Bash, &mut cmd, "pml", &mut std::io::stdout()),
        "zsh" => generate(Zsh, &mut cmd, "pml", &mut std::io::stdout()),
        "fish" => generate(Fish, &mut cmd, "pml", &mut std::io::stdout()),
        "powershell" => generate(PowerShell, &mut cmd, "pml", &mut std::io::stdout()),
        "elvish" => generate(Elvish, &mut cmd, "pml", &mut std::io::stdout()),
        _ => {
            eprintln!("Unknown shell: {}", shell);
            bail!("unknown shell")
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("{}", usage());
        bail!("no command provided");
    }
    // Top-level flags
    if args.len() == 1 {
        match args[0].as_str() {
            "--help" | "-h" => {
                println!("{}", usage());
                return Ok(());
            }
            "--version" | "-V" => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            _ => {}
        }
    }
    let cmd = args.remove(0);
    match cmd.as_str() {
        "completions" => cmd_completions(args),
        "parse" => cmd_parse(args),
        "validate" => cmd_validate(args),
        _ => {
            eprintln!("{}", usage());
            bail!("unknown command: {}", cmd)
        }
    }
}

fn cmd_parse(mut args: Vec<String>) -> Result<()> {
    if args.is_empty() {
        eprintln!("{}", usage());
        bail!("parse: missing <file>");
    }
    let file = args.remove(0);
    let json = args.iter().any(|a| a == "--json");
    let pretty = args.iter().any(|a| a == "--pretty");
    let quiet = args.iter().any(|a| a == "--quiet");
    let verbose = args.iter().any(|a| a == "--verbose");
    let lims = parse_limits_flags(&args);
    let input = if file == "-" {
        let mut buf = String::new();
        if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
            if json {
                let payload = if pretty {
                    serde_json::to_string_pretty(&json!({"ok": false, "err": {"code": "IO", "msg": e.to_string(), "path": "-" }}))?
                } else {
                    json!({"ok": false, "err": {"code": "IO", "msg": e.to_string(), "path": "-" }}).to_string()
                };
                eprintln!("{}", payload);
            } else {
                eprintln!("IO error reading - (stdin): {}\nHint: pass a file path or pipe UTF-8 text via '-'", e);
            }
            std::process::exit(3);
        }
        buf
    } else {
        match fs::read_to_string(&file) {
            Ok(s) => s,
            Err(e) => {
                if json {
                    let payload = if pretty {
                        serde_json::to_string_pretty(
                            &json!({"ok": false, "err": {"code": "IO", "msg": e.to_string(), "path": file }}),
                        )?
                    } else {
                        json!({"ok": false, "err": {"code": "IO", "msg": e.to_string(), "path": file }})
                            .to_string()
                    };
                    eprintln!("{}", payload);
                } else {
                    eprintln!("IO error reading {}: {}\nHint: check the file path and permissions.", file, e);
                }
                std::process::exit(3);
            }
        }
    };
    if verbose {
        if let Some(l) = lims {
            eprintln!("limits: depth={}, nodes={}, input-size={} bytes", l.max_depth, l.max_nodes, l.max_input_bytes);
        } else {
            eprintln!("limits: defaults in effect");
        }
    }
    let parse_res = if let Some(l) = lims {
        parse_with_limits(&input, l)
    } else {
        parse(&input)
    };
    match parse_res {
        Ok(doc) => {
            if json {
                if pretty {
                    println!("{}", serde_json::to_string_pretty(&doc)?);
                } else {
                    println!("{}", serde_json::to_string(&doc)?);
                }
            } else if !quiet {
                // Minimal human-readable dump (suppressed by --quiet)
                println!("Parsed Document: {} root blocks", doc.blocks.len());
            }
            Ok(())
        }
        Err(e) => {
            if json {
                let payload = if pretty {
                    serde_json::to_string_pretty(
                        &json!({"ok": false, "err": {"code": match e.kind { proofdown_ast::ErrorKind::Syntax => "Syntax", proofdown_ast::ErrorKind::LimitExceeded => "LimitExceeded" }, "msg": e.msg, "line": e.line, "col": e.col}}),
                    )?
                } else {
                    json!({"ok": false, "err": {"code": match e.kind { proofdown_ast::ErrorKind::Syntax => "Syntax", proofdown_ast::ErrorKind::LimitExceeded => "LimitExceeded" }, "msg": e.msg, "line": e.line, "col": e.col}}).to_string()
                };
                eprintln!("{}", payload);
            } else {
                eprintln!(
                    "Parse error at {}:{} [{}]: {}\nHint: verify component attributes and tag matching.",
                    e.line,
                    e.col,
                    match e.kind {
                        proofdown_ast::ErrorKind::Syntax => "Syntax",
                        proofdown_ast::ErrorKind::LimitExceeded => "LimitExceeded",
                    },
                    e.msg
                );
            }
            std::process::exit(1);
        }
    }
}

fn cmd_validate(mut args: Vec<String>) -> Result<()> {
    if args.is_empty() {
        eprintln!("{}", usage());
        bail!("validate: missing <file>");
    }
    let file = args.remove(0);
    let json = args.iter().any(|a| a == "--json");
    let pretty = args.iter().any(|a| a == "--pretty");
    let quiet = args.iter().any(|a| a == "--quiet");
    let verbose = args.iter().any(|a| a == "--verbose");
    let lims = parse_limits_flags(&args);
    let input = if file == "-" {
        let mut buf = String::new();
        if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
            if json {
                let payload = if pretty {
                    serde_json::to_string_pretty(&json!({"ok": false, "err": {"code": "IO", "msg": e.to_string(), "path": "-" }}))?
                } else {
                    json!({"ok": false, "err": {"code": "IO", "msg": e.to_string(), "path": "-" }}).to_string()
                };
                eprintln!("{}", payload);
            } else {
                eprintln!("IO error reading - (stdin): {}\nHint: pass a file path or pipe UTF-8 text via '-'", e);
            }
            std::process::exit(3);
        }
        buf
    } else {
        match fs::read_to_string(&file) {
            Ok(s) => s,
            Err(e) => {
                if json {
                    let payload = if pretty {
                        serde_json::to_string_pretty(
                            &json!({"ok": false, "err": {"code": "IO", "msg": e.to_string(), "path": file }}),
                        )?
                    } else {
                        json!({"ok": false, "err": {"code": "IO", "msg": e.to_string(), "path": file }})
                            .to_string()
                    };
                    eprintln!("{}", payload);
                } else {
                    eprintln!("IO error reading {}: {}\nHint: check the file path and permissions.", file, e);
                }
                std::process::exit(3);
            }
        }
    };
    if verbose {
        if let Some(l) = lims {
            eprintln!("limits: depth={}, nodes={}, input-size={} bytes", l.max_depth, l.max_nodes, l.max_input_bytes);
        } else {
            eprintln!("limits: defaults in effect");
        }
    }
    let doc = match parse(&input) {
        Ok(d) => d,
        Err(e) => {
            if json {
                let payload = if pretty {
                    serde_json::to_string_pretty(
                        &json!({"ok": false, "err": {"code": match e.kind { proofdown_ast::ErrorKind::Syntax => "Syntax", proofdown_ast::ErrorKind::LimitExceeded => "LimitExceeded" }, "msg": e.msg, "line": e.line, "col": e.col}}),
                    )?
                } else {
                    json!({"ok": false, "err": {"code": match e.kind { proofdown_ast::ErrorKind::Syntax => "Syntax", proofdown_ast::ErrorKind::LimitExceeded => "LimitExceeded" }, "msg": e.msg, "line": e.line, "col": e.col}}).to_string()
                };
                eprintln!("{}", payload);
            } else {
                eprintln!(
                    "Parse error at {}:{} [{}]: {}\nHint: correct the input before validation.",
                    e.line,
                    e.col,
                    match e.kind {
                        proofdown_ast::ErrorKind::Syntax => "Syntax",
                        proofdown_ast::ErrorKind::LimitExceeded => "LimitExceeded",
                    },
                    e.msg
                );
            }
            std::process::exit(1);
        }
    };
    let limits = lims
        .map(|l| Limits {
            max_depth: l.max_depth,
            max_nodes: l.max_nodes,
            max_input_bytes: l.max_input_bytes,
        })
        .unwrap_or_default();
    if let Err(e) = validate(&doc, Some(&limits)) {
        if json {
            // map to a generic JSON error payload
            let payload = if pretty {
                serde_json::to_string_pretty(
                    &json!({"ok": false, "err": {"code": e.code(), "msg": e.to_string()}}),
                )?
            } else {
                json!({"ok": false, "err": {"code": e.code(), "msg": e.to_string()}}).to_string()
            };
            eprintln!("{}", payload);
        } else {
            eprintln!("Validation error: {}\nHint: unknown components/attributes are rejected by the validator.", e);
        }
        std::process::exit(2);
    } else if json {
        let payload = if pretty {
            serde_json::to_string_pretty(&json!({"ok": true}))?
        } else {
            json!({"ok": true}).to_string()
        };
        println!("{}", payload);
    } else if !quiet {
        println!("OK");
    }
    Ok(())
}

fn parse_limits_flags(args: &[String]) -> Option<ParserLimits> {
    let mut out = ParserLimits::default();
    let mut seen = false;
    for a in args {
        if let Some(v) = a.strip_prefix("--limits.depth=") {
            if let Ok(n) = v.parse::<usize>() {
                out.max_depth = n;
                seen = true;
            }
        } else if let Some(v) = a.strip_prefix("--limits.nodes=") {
            if let Ok(n) = v.parse::<usize>() {
                out.max_nodes = n;
                seen = true;
            }
        } else if let Some(v) = a.strip_prefix("--limits.input-size=") {
            if let Ok(n) = v.parse::<usize>() {
                out.max_input_bytes = n;
                seen = true;
            }
        }
    }
    if seen {
        Some(out)
    } else {
        None
    }
}
