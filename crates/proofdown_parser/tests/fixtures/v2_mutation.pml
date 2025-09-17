## Mutation Testing

<grid cols=2 gap=16>
  <card title="Summary">
    <artifact.json id="mutation-summary.json" collapsed=true depth=2 caption="Mutation score" />
    <artifact.table id="mutation-summary.json" kind="mutation" caption="Operators" />
  </card>
  <card title="Diffs">
    <artifact.text id="survived-diff.txt" max_lines=200 caption="Survived mutants (excerpt)" />
  </card>
</grid>
