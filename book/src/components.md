# Components

Structural containers compose content into scan-friendly layouts:

- `grid(cols, gap?)`: Rows/columns layout; validator enforces `cols` 1..6 and `gap` 0..64.
- `section(title)`: Logical grouping with heading-like intent.
- `card(title)`: A bordered content box; contains arbitrary blocks.

Components are recognized at parse time as `Component { name, attrs, children, self_closing }`. Children may contain full Markdown blocks/inlines. Raw HTML blocks/inline are dropped by the parser; use components instead.
