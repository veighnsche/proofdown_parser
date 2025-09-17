# Security Policy

Proofdown is designed for deterministic, safe rendering of CI/testing evidence. This repository follows principles that minimize attack surface.

Core security principles

- No scripting or raw HTML in parser output or downstream renderers.
- All artifacts referenced by `id` must be content-addressed and integrity-checked by callers before rendering.
- Fail closed on digest mismatches or missing artifacts.
- Bound resource usage: depth, node count, and input bytes (configurable limits).
- Escape text and treat HTML/DOM dumps as data (never execute).

Reporting

- Please open a private issue or contact the maintainers if you believe you have found a security issue.
- Provide a minimal reproduction and impacted versions if possible.

Hardening checklist

- Parser: no IO, no panics on malformed input, bounded scans.
- Validator: strict whitelist and attribute bounds; reject unknown components/attributes.
- Renderer (out of scope here): escape all data; keep JSON collapsed by default; depth limits for viewers.
