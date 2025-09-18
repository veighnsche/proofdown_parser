use std::{env, fs};

use anyhow::{bail, Result};
use proofdown_parser::{parse, parse_with_limits, ParserLimits};
use proofdown_validate::{validate, Limits};
use serde_json::json;

fn usage() -> &'static str {
    "Usage:\n  pml [--help|-h] [--version|-V]\n  pml parse <file> [--json] [--pretty] [--limits.depth=N] [--limits.nodes=N] [--limits.input-size=BYTES]\n  pml validate <file> [--json] [--pretty] [--limits.depth=N] [--limits.nodes=N] [--limits.input-size=BYTES]\n\nExit codes:\n  0 OK\n  1 Parse error\n  2 Validation error\n  3 IO/usage error\n"
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
    let lims = parse_limits_flags(&args);
    let input = match fs::read_to_string(&file) {
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
                eprintln!("IO error reading {}: {}", file, e);
            }
            std::process::exit(3);
        }
    };
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
            } else {
                // Minimal human-readable dump
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
                    "Parse error at {}:{} [{}]: {}",
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
    let lims = parse_limits_flags(&args);
    let input = match fs::read_to_string(&file) {
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
                eprintln!("IO error reading {}: {}", file, e);
            }
            std::process::exit(3);
        }
    };
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
                    "Parse error at {}:{} [{}]: {}",
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
            eprintln!("Validation error: {}", e);
        }
        std::process::exit(2);
    } else if json {
        let payload = if pretty {
            serde_json::to_string_pretty(&json!({"ok": true}))?
        } else {
            json!({"ok": true}).to_string()
        };
        println!("{}", payload);
    } else {
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
