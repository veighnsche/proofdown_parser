## Accessibility & Visual diffs

<grid cols=2 gap=16>
  <card title="Accessibility (axe-core)">
    <artifact.json id="axe-summary.json" collapsed=true depth=2 caption="axe summary" />
    <artifact.table id="axe-summary.json" kind="a11y" sort_by="impact" caption="Violations by rule" />
  </card>
  <card title="Visual changes">
    <artifact.table id="visual-summary.json" kind="visual" caption="Changed stories" />
    <grid cols=3 gap=8>
      <artifact.image id="login-baseline.png" alt="Login baseline" max_height=220 caption="baseline" />
      <artifact.image id="login-actual.png" alt="Login actual" max_height=220 caption="actual" />
      <artifact.image id="login-diff.png" alt="Login diff" max_height=220 caption="diff" />
    </grid>
  </card>
</grid>
