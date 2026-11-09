# Tokens and utilities (paste into src/app/globals.css)

```css
:root {
  --neu-bg: #e0e5ec;
  --neu-light: #ffffff;
  --neu-dark: #a3b1c6;
  --neu-text: #4a5568;
  --neu-muted: #9aa5b8;
  --neu-blue: #3b6cff;
  --neu-red: #e5484d;
  --neu-orange: #ff7a59;
  --neu-yellow: #ffd23f;
  --neu-green: #8fd400;
  --neu-teal: #3de0c5;
  --neu-radius: 24px;
}

.neu-raised {
  background: var(--neu-bg);
  border-radius: var(--neu-radius);
  border: 1px solid rgba(255, 255, 255, 0.6);
  box-shadow: 8px 8px 16px var(--neu-dark), -8px -8px 16px var(--neu-light);
}
.neu-raised-sm {
  background: var(--neu-bg);
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.6);
  box-shadow: 4px 4px 8px var(--neu-dark), -4px -4px 8px var(--neu-light);
}
.neu-inset {
  background: var(--neu-bg);
  border-radius: var(--neu-radius);
  box-shadow: inset 6px 6px 12px var(--neu-dark), inset -6px -6px 12px var(--neu-light);
}
.neu-dot { width: 12px; height: 12px; border-radius: 9999px; box-shadow: 0 0 12px currentColor; }
```
