## Database migrations

<grid cols=2 gap=16>
  <card title="Status">
    <artifact.table id="migrations-summary.json" kind="db_migrations" caption="Applied/pending/failed" />
  </card>
  <card title="SQL snippets">
    <artifact.text id="migration-001.sql" max_lines=50 caption="001-add-index.sql" />
    <artifact.link id="migrations.zip" title="All migration scripts" />
  </card>
</grid>
