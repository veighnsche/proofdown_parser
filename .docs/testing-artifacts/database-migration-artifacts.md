# Database Migration Testing Artifacts

Database migration testing verifies that schema and data changes apply correctly and safely across environments. Artifacts include migration files, generated SQL, dry-run plans, verification test results, and schema diffs.

## Common producers & ecosystems

- SQL-first migration tools:
  - Flyway — versioned SQL and Java migrations, info/validate outputs
  - Liquibase — changelogs (XML/YAML/JSON/SQL), `updateSQL`, `status`, `diff`, `diffChangeLog`
  - Sqitch — change plans, verify scripts
- Language-specific ORMs & migration frameworks:
  - Rails ActiveRecord — migration Ruby files, schema.rb/structure.sql, test results (JUnit/TAP)
  - Django — migration Python files, `--plan`, `--check`; `pytest`/unittest results
  - Prisma Migrate — migration SQL, `prisma migrate diff`
  - Knex/Sequelize — migration scripts and logs
  - EF Core — migration C# files, generated SQL scripts
- Schema management & testing:
  - pgTAP — TAP output for PostgreSQL unit tests
  - Skeema — MySQL schema diff artifacts
  - Liquibase/Skeema/OpenDBDiff — schema diff reports

## Artifact types

- Migration definitions: SQL/Ruby/Python/JSON/YAML changelogs
- Generated SQL scripts: dry-run/`updateSQL` outputs for review
- Execution plans: `--plan` previews (Django), `info`/`validate` (Flyway), `status` (Liquibase)
- Verification results: JUnit XML, TAP (pgTAP), JSON summaries from test harnesses
- Schema diffs: human-readable or JSON diffs (tables, columns, constraints)
- Data backfill reports: row counts, validation queries, timing metrics

## Examples

Flyway info (simplified JSON-esque):

```json
{ "migrations": [ { "version": "1.2.0", "description": "add_users_email", "state": "Success" } ] }
```

Liquibase diffChangeLog (excerpt):

```yaml
- changeSet:
    id: 001-add-index
    author: ci
    changes:
      - createIndex:
          tableName: users
          columns:
            - column: { name: email }
```

pgTAP TAP output (excerpt):

```
1..3
ok 1 - table users has column id
ok 2 - column email exists
not ok 3 - column email has unique index
```

## Proofdown viewer mapping

- Migration plans and generated SQL → `artifact.markdown` (pre-rendered) for short scripts; larger scripts via `artifact.link`.
- Tool status/validate outputs → normalized JSON for `artifact.json` and rollup `artifact.table` (pending/applied/failed).
- Schema diffs → `artifact.json` with per-object changes; provide original diff text via `artifact.markdown` or `artifact.link`.
- Test results (JUnit/TAP) → convert to JSON summary for `artifact.json`; present failures prominently.

## Implications for Proofdown Language Spec

- Keep grammar minimal; treat migration files, plans, and diffs as artifacts. Do not add SQL/YAML parsing to the language.
- Standardize `artifact.table` rollups (pending/applied/failed migrations, objects added/changed/dropped) as renderer behavior.
- Present generated SQL only as pre-rendered text (`artifact.markdown`) for short snippets; link full scripts via `artifact.link` to keep pages deterministic.
- Support TAP/JUnit summaries via `artifact.json`; avoid embedding large logs inline—link them instead.
- Prefer digest-addressed artifacts so reviewers can verify the exact plan and diff that CI produced.

## References

- Flyway: <https://documentation.red-gate.com/fd>
- Liquibase: <https://docs.liquibase.com/>
- pgTAP: <https://pgtap.org/>
- Prisma Migrate: <https://www.prisma.io/docs/concepts/components/prisma-migrate>
- EF Core migrations: <https://learn.microsoft.com/ef/core/managing-schemas/migrations/>
- Skeema: <https://www.skeema.io/>
