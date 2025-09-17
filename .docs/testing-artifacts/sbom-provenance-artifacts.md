# Supply Chain: SBOM & Provenance Artifacts

Supply chain artifacts describe software composition and how it was built. They are foundational to verifying what CI produced and whether dependencies are vulnerable.

## Artifact types

- SBOMs: SPDX 2.x/3.x (JSON), CycloneDX 1.x (JSON/XML)
- Provenance attestations: SLSA Provenance (in-toto statement with `slsa.dev/provenance` predicate)
- Signatures/attestations: Sigstore Cosign bundles

## Common producers

- Syft (Anchore) — SBOMs in SPDX/CycloneDX
- Trivy — SBOMs and vulnerability reports (JSON, SARIF)
- CycloneDX CLI, SPDX tooling
- Build systems (GitHub Actions, Cloud Build) emitting SLSA provenance via generator actions

## Examples

CycloneDX JSON (excerpt):
```json
{ "bomFormat": "CycloneDX", "specVersion": "1.6", "components": [ { "name": "lodash", "version": "4.17.21" } ] }
```

SLSA Provenance (excerpt):
```json
{
  "_type": "https://in-toto.io/Statement/v1",
  "predicateType": "https://slsa.dev/provenance/v1.1",
  "subject": [{"name": "artifact.tar.gz", "digest": {"sha256": "..."}}]
}
```

## Proofdown viewer mapping

- SBOM JSON → `artifact.json` (collapsed) with `artifact.table` summarizing components by ecosystem/severity (if enriched).
- Provenance (in-toto/SLSA) → `artifact.json` and highlight the subject digests; link signature bundle via `artifact.link`.
- Vulnerability scan outputs (SARIF/JSON) → see Security page; include rollups.

## Implications for Proofdown Language Spec

- Treat SBOMs and provenance as JSON-first artifacts; avoid embedding HTML visualizers—link them via `artifact.link`.
- Encourage standardized rollups in `artifact.table` (e.g., component counts by ecosystem, license, severity) as renderer behavior, not grammar.
- Emphasize digest-addressed references across the language; the signed Index should enumerate all SBOM/provenance artifacts.
- Keep grammar minimal and parser syntax-only; normalization and enrichment (e.g., CVE joins) occur in SSG/validator.
- Large SBOMs should default to collapsed JSON with optional `depth` to keep pages usable and deterministic.

## References

- SPDX specs: https://spdx.dev/use/specifications/
- CycloneDX specs: https://cyclonedx.org/specification/overview/
- CycloneDX JSON reference: https://cyclonedx.org/docs/1.6/json/
- SLSA provenance spec: https://slsa.dev/spec/v1.1/provenance
- Trivy reporting: https://trivy.dev/latest/docs/configuration/reporting/
- Syft: https://github.com/anchore/syft
