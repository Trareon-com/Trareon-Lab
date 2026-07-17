# Examination design tokens

**Scope:** Slint examination shell status and interaction tokens.  
**Rule:** Status is never color-only — every state pairs color with icon glyph + text label (+ optional pattern).

## Status tokens

| Token | Meaning | Color (light) | Icon | Text label | Pattern |
|---|---|---|---|---|---|
| `status.ok` | Verified / accepted | `#1B7F4A` | check | Verified | solid |
| `status.warn` | Limited / partial | `#A15C00` | triangle | Limited | diagonal hatch |
| `status.error` | Failed / integrity fail | `#B42318` | cross | Failed | dense dots |
| `status.info` | Informational / open | `#175CD3` | info | Open | soft fill |
| `status.neutral` | Unknown / not run | `#667085` | dash | Not run | outline |

## Interaction tokens

| Token | Value | Notes |
|---|---|---|
| `focus.ring` | 2px `#175CD3` | Keyboard focus always visible |
| `selection.bg` | `#E0EAFF` | List / table selection |
| `danger.action` | text + icon; never red fill alone | Destructive actions require confirm |

## Non-goals

- Dark-theme palette freeze (may follow; contrast ratios must still pass).
- Using emoji as the sole status indicator.
