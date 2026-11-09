---
name: neumorphism-gsap-shadcn
description: Build or restyle UI as Neumorphism (Soft UI) with GSAP animation on top of shadcn/ui, Next.js, TypeScript and Tailwind. Use whenever the user mentions neumorphism, soft UI, soft shadows, dual-shadow cards, embossed/inset controls, fintech/crypto dashboard widgets, or asks for GSAP animation on shadcn components, even if they do not name the style explicitly.
---

# Neumorphism + GSAP + shadcn

## Workflow (phased)
1. **Setup**: `npx shadcn@latest init`, then `npx shadcn@latest add button card input switch slider progress badge`, then `npm i gsap @gsap/react lucide-react`.
2. **Tokens + utilities**: add CSS variables and `.neu-*` classes from `references/tokens.md` to `src/app/globals.css`; map shadcn vars (`--background`, `--card`, `--border`, `--radius`) to them.
3. **Primitives**: wrap shadcn components in `src/components/neu/` (NeuCard, NeuButton, NeuInput, NeuSwitch, NeuSlider, NeuProgress, NeuIconButton, NeuLikeButton). Never replace them with raw HTML.
4. **Composites**: SparklineCard, BalanceCard, CreditCard, RingChart, Legend rows, PaginationDots.
5. **Animation**: GSAP via `useGSAP` only (see rules below).
6. **Demo page**: `src/app/page.tsx`, responsive 2-column grid.
7. **Verify**: run the checklist at the bottom.

Fetch the live shadcn and GSAP docs before generating install commands or API usage, to avoid stale APIs.

## Visual rules
- Background and component colour are identical (`--neu-bg: #e0e5ec`). Depth comes only from shadows.
- Raised: `8px 8px 16px #a3b1c6, -8px -8px 16px #ffffff` + `1px solid rgba(255,255,255,.6)`.
- Inset (inputs, switch track, ring hole, slider track): `inset 6px 6px 12px #a3b1c6, inset -6px -6px 12px #ffffff`.
- Radii 16-32px; circular or rounded-2xl icon buttons; min 44px tap target.
- Colour only for accents: orange to yellow to green to teal gradients, blue switch knob, red heart, glowing status dots (`box-shadow: 0 0 12px currentColor`).
- Text: `#4a5568`, muted `#9aa5b8`. Never pure black.
- Glass touch only on the credit card (translucent overlapping circles).

## Animation rules
- Register once in `src/lib/gsap.ts`; import `gsap` and `useGSAP` from there.
- Always `useGSAP(() => {...}, { scope: ref })`; never raw `useEffect` for tweens.
- Wrap in `gsap.matchMedia()` and honour `prefers-reduced-motion`.
- Load: stagger cards (`y: 24 -> 0`, `opacity: 0 -> 1`, `stagger: .06`, `power3.out`).
- Sparklines: `strokeDashoffset` draw-on, 1.2s, `power2.inOut`.
- Ring arcs: 0 to value, stagger .15, `power3.out`.
- Sliders/progress: fill 0 to value; tooltip follows thumb.
- Balance: count-up with `snap`, must end on the exact value.
- Button: pointerdown tweens to inset shadow + `scale .98` (.15s); release uses `elastic.out(1, .6)`.
- Switch knob: `back.out(1.7)` plus a single glow pulse.
- Heart: scale 1 to 1.3 to 1 with elastic, grey to red.
- Card hover: `y: -2`, shadow deepens, .25s.

## Code rules
- TypeScript strict; no `any`.
- `"use client"` only on components using state or GSAP.
- Forward refs and merge them when wrapping shadcn components.
- `aria-label` on icon-only buttons; visible blue `focus-visible` ring.
- No dependencies beyond gsap, @gsap/react, lucide-react.

## Verification checklist
- [ ] Surface and cards share `--neu-bg`
- [ ] Light shadow top-left, dark bottom-right everywhere
- [ ] Inputs/tracks inset; buttons raised then inset on press
- [ ] Sparklines and ring animate on load; count-up lands exact
- [ ] Reduced motion disables animation
- [ ] `npx tsc --noEmit` passes
- [ ] All tweens inside `useGSAP` with `scope`
- [ ] Focus ring visible; icon buttons labelled
