# FAQ

- Is Proofdown a general markup like Markdown?
  - No. It’s a minimal, purpose-built language for CI/testing evidence. It avoids inline syntax and focuses on deterministic blocks and components.

- Where are semantics enforced?
  - The validator (or SSG) enforces whitelist and attribute bounds. The parser is syntax-only.

- How are large JSON artifacts handled?
  - Use `artifact.json` with `collapsed=true` and a small `depth`. For focused views, use `json_pointer` to a subtree.

- Does Proofdown support scripting or embedded HTML?
  - No. Content must be safe and deterministic.
