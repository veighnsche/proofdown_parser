# Contract Testing Artifacts

Contract tests ensure that services communicate according to an agreed contract (HTTP/JSON, gRPC, messages). Artifacts include the contract documents themselves and verification results.

## Common producers

- Pact (consumer-driven contracts) — Pact files (JSON), provider verification results
- Spring Cloud Contract — generated stubs, verification results
- OpenAPI/Swagger + Dredd/Schemathesis — schema validation results
- AsyncAPI — event/message contract definitions

## Artifact types

- Pact contracts: `*.json` (v2–v4 of Pact spec)
- Verification results: JUnit XML, JSON summaries, HTML reports
- OpenAPI/AsyncAPI specs: `.yaml`/`.json`
- Breaking-change reports: JSON or Markdown summaries from diff tools

## Pact JSON (excerpt)
```json
{
  "consumer": { "name": "web-ui" },
  "provider": { "name": "user-service" },
  "interactions": [
    {
      "description": "get user",
      "request": { "method": "GET", "path": "/users/123" },
      "response": { "status": 200, "headers": {"Content-Type":"application/json"}, "body": {"id":123} }
    }
  ],
  "metadata": { "pactSpecification": { "version": "4.0.0" } }
}
```

## Proofdown viewer mapping

- Pact JSON → `artifact.json` (collapsed) and `artifact.table` summarizing interactions.
- Provider verification (JUnit/JSON) → `artifact.json` + rollup table; include logs via `artifact.markdown` or link.
- OpenAPI/AsyncAPI → `artifact.link` to spec plus diff/breaking-change summary as `artifact.json`.
- Hosted Pact Broker links → `artifact.link` with key results summarized in-page.

## Implications for Proofdown Language Spec

- Keep grammar free of contract-specific syntax; treat Pact/OpenAPI/AsyncAPI documents as artifacts.
- Standardize interaction rollups via `artifact.table` (provider, path, method, status/result); implement behavior in renderer.
- Large specs and broker UIs remain linked artifacts (`artifact.link`); present compact JSON summaries in-page.
- Encourage digest-addressed contract files and verification outputs for immutability and traceability.
- Parser stays syntax-only; SSG performs normalization (e.g., pact schema versions) into stable JSON summaries.

## References

- Pact specification: https://github.com/pact-foundation/pact-specification
- Pact docs: https://docs.pact.io/
- Spring Cloud Contract: https://spring.io/projects/spring-cloud-contract
- Dredd: https://dredd.org/en/latest/
- Schemathesis: https://schemathesis.readthedocs.io/
- OpenAPI: https://www.openapis.org/
- AsyncAPI: https://www.asyncapi.com/
