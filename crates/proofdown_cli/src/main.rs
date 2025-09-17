use std::{env, fs};

use anyhow::{bail, Context, Result};
use proofdown_parser::parse;
use proofdown_validate::{validate, Limits};

fn usage() -> &'static str {
    "Usage:\n  pml parse <file> [--json] [--pretty]\n  pml validate <file>\n"
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("{}", usage());
        bail!("no command provided");
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
    let input = fs::read_to_string(&file).with_context(|| format!("reading {}", file))?;
    let doc = parse(&input).with_context(|| format!("parsing {}", file))?;
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

fn cmd_validate(mut args: Vec<String>) -> Result<()> {
    if args.is_empty() {
        eprintln!("{}", usage());
        bail!("validate: missing <file>");
    }
    let file = args.remove(0);
    let input = fs::read_to_string(&file).with_context(|| format!("reading {}", file))?;
    let doc = parse(&input).with_context(|| format!("parsing {}", file))?;
    validate(&doc, Some(&Limits::default())).context("validation failed")?;
    println!("OK");
    Ok(())
}
