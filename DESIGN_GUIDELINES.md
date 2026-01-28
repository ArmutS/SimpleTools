# SimpleTools Design Guidelines

## Overview

This document defines the premium UI design system for SimpleTools. All pages should follow these guidelines for consistent, modern, and beautiful user experience.

## Color System (Catppuccin Dark)

### Base Colors

```css
--bg-app: #1e1e2e /* Main background */ --bg-input: #181825 /* Cards, inputs */
  --bg-elevated: #11111b /* Elevated surfaces */ --border-color: #313244
  /* Borders */ --text-main: #cdd6f4 /* Main text */ --text-muted: #a6adc8
  /* Secondary text */;
```

### Accent Colors by Module

**Text Tools** → Lavender/Purple

- Primary: `#cba6f7` (Lavender)
- Secondary: `#b4befe` (Blue-lavender)
- Gradient: `linear-gradient(135deg, #cba6f7 0%, #b4befe 100%)`

**PDF Tools** → Red/Pink

- Primary: `#f38ba8` (Red)
- Secondary: `#eba0ac` (Pink)
- Gradient: `linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%)`

**Converters** → Teal/Sky

- Primary: `#94e2d5` (Teal)
- Secondary: `#74c7ec` (Sky)
- Gradient: `linear-gradient(135deg, #94e2d5 0%, #74c7ec 100%)`

**File & System** → Blue/Sky

- Primary: `#89b4fa` (Blue)
- Secondary: `#74c7ec` (Sky)
- Gradient: `linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%)`

### Additional Colors

- Success: `#a6e3a1` (Green)
- Warning: `#f9e2af` (Yellow)
- Error: `#f38ba8` (Red)
- Info: `#89b4fa` (Blue)

## Layout Patterns

### Container

```css
.container {
  min-height: 100vh;
  padding: 3rem 2rem;
  background: linear-gradient(135deg, #1e1e2e 0%, #181825 100%);
}

.content {
  max-width: 900px; /* Adjust based on content */
  margin: 0 auto;
}
```

### Header Section

```html
<div class="header">
  <div class="icon-wrapper">
    <i class="nf-md-{icon} {module}-icon"></i>
  </div>
  <h1>{Tool Name}</h1>
  <p class="subtitle">{Tool Description}</p>
</div>
```

```css
.header {
  text-align: center;
  margin-bottom: 3rem;
}

.icon-wrapper {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 80px;
  height: 80px;
  background: {module gradient};
  border-radius: 20px;
  margin-bottom: 1.5rem;
  box-shadow: 0 8px 24px rgba({accent-color}, 0.3);
  animation: float 3s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translateY(0px); }
  50% { transform: translateY(-10px); }
}

h1 {
  background: {module gradient};
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  font-size: 2.5rem;
  margin-bottom: 0.5rem;
  font-weight: bold;
}

.subtitle {
  color: #a6adc8;
  font-size: 1.1rem;
}
```

## Card/Section Patterns

### Glassmorphic Card

```css
.input-section,
.result-section {
  background: rgba(24, 24, 37, 0.7);
  backdrop-filter: blur(10px);
  padding: 2.5rem;
  border-radius: 20px;
  margin-bottom: 2rem;
  border: 1px solid rgba({accent-color}, 0.1);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}
```

### Result Section Animation

```css
.result-section {
  animation: slideUp 0.4s ease-out;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
```

## Button Patterns

### Primary Button

```html
<button class="btn-primary">
  <i class="nf-md-{icon}"></i>
  <span>{Button Text}</span>
</button>
```

```css
.btn-primary {
  padding: 1rem 2rem;
  background: {module gradient};
  color: #1e1e2e;
  border: none;
  border-radius: 12px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba({accent-color}, 0.3);
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba({accent-color}, 0.4);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
```

### Secondary Button

```css
.btn-secondary {
  padding: 1rem 2rem;
  background: rgba(49, 50, 68, 0.5);
  color: #cdd6f4;
  border: 2px solid #313244;
  border-radius: 12px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  transition: all 0.3s ease;
}

.btn-secondary:hover {
  background: rgba(49, 50, 68, 0.8);
  border-color: {accent-color};
}
```

## Input Patterns

### Text Input

```css
input[type="text"],
input[type="number"],
textarea,
select {
  width: 100%;
  padding: 1rem 1.25rem;
  background: #11111b;
  border: 2px solid #313244;
  border-radius: 10px;
  color: #cdd6f4;
  font-family: "Consolas", "Monaco", monospace;
  font-size: 1rem;
  transition: all 0.3s ease;
}

input:focus,
textarea:focus,
select:focus {
  outline: none;
  border-color: {accent-color};
  box-shadow: 0 0 0 3px rgba({accent-color}, 0.1);
}
```

### Label

```css
label {
  color: {accent-color};
  font-size: 0.95rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

label i {
  font-size: 1.1rem;
}
```

## Animation Patterns

### Float Animation (Icons)

```css
@keyframes float {
  0%,
  100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-10px);
  }
}

.icon-wrapper {
  animation: float 3s ease-in-out infinite;
}
```

### Slide Up Animation (Content)

```css
@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.result-section {
  animation: slideUp 0.4s ease-out;
}
```

### Fade In Animation (List Items)

```css
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.list-item {
  animation: fadeIn 0.3s ease-out forwards;
  animation-delay: calc(var(--index) * 50ms);
}
```

### Staggered List Animation

```html
{#each items as item, i}
<div style="animation-delay: {i * 50}ms">{item}</div>
{/each}
```

## Module Landing Pages

### Grid Layout

```html
<div class="tools-box">
  {#each tools as tool, i}
  <button class="tool-card" style="animation-delay: {i * 0.05}s">
    <div class="key-badge">{tool.key}</div>
    <i class="{tool.icon}"></i>
    <span>{tool.name}</span>
  </button>
  {/each}
</div>
```

````css
### Main Landing Grid ```css .tools-box-main {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 0.75rem;
  width: 100%;
  flex: 1;
}
````

### Module Landing Grid

```css
.tools-box-module {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  grid-template-rows: repeat(3, 1fr);
  gap: 1rem;
  width: 100%;
  flex: 1;
}
```

.tool-card {
background: rgba(24, 24, 37, 0.7);
backdrop-filter: blur(10px);
border: 2px solid rgba({accent-color}, 0.2);
border-radius: 16px;
padding: 2rem 1.5rem;
cursor: pointer;
transition: all 0.3s ease;
position: relative;
animation: fadeIn 0.6s ease-out backwards;
}

.tool-card:hover {
transform: translateY(-4px);
box-shadow: 0 8px 24px rgba({accent-color}, 0.3);
border-color: rgba({accent-color}, 0.5);
}

.tool-card i {
font-size: 3rem;
color: {accent-color};
margin-bottom: 1rem;
display: block;
}

````

## Typography

### Headings

- **H1**: 2.5rem, gradient text, bold
- **H2**: 1.5rem, accent color, flex with icon
- **H3**: 1.2rem, text-main color

### Body Text

- **Main**: #cdd6f4
- **Muted**: #a6adc8
- **Monospace**: "Consolas", "Monaco", monospace

## Spacing System

- **Extra Small**: 0.5rem (8px)
- **Small**: 0.75rem (12px)
- **Medium**: 1rem (16px)
- **Large**: 1.5rem (24px)
- **Extra Large**: 2rem (32px)
- **XXL**: 2.5rem (40px)
- **XXXL**: 3rem (48px)

## Border Radius

- **Small**: 8px (badges, small buttons)
- **Medium**: 10-12px (inputs, buttons)
- **Large**: 16-20px (cards, sections)

## Shadows

### Primary Element Shadow

```css
box-shadow: 0 4px 12px rgba({accent-color}, 0.3);
````

### Hover Shadow

```css
box-shadow: 0 6px 20px rgba({accent-color}, 0.4);
```

### Card Shadow

```css
box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
```

## Icon Usage

- Use **Nerd Font** icons (nf-md-\*)
- Icon size in buttons: 1.2-1.3rem
- Icon size in headers: 3rem (floating icons)
- Icon size in labels: 1.1rem
- Always pair icons with text in buttons

## Accessibility

- All icon-only buttons must have `aria-label`
- Color contrast ratio ≥ 4.5:1
- Keyboard navigation support
- Focus states with visible outlines

## Implementation Checklist

For each new page, ensure:

- [ ] Gradient background container
- [ ] Floating icon in header with module color
- [ ] Gradient text heading
- [ ] Glassmorphic cards
- [ ] Proper animations (float, slideUp, fadeIn)
- [ ] Module-specific color theming
- [ ] Icon usage throughout
- [ ] Smooth transitions on all interactions
- [ ] Proper spacing and typography
- [ ] Accessibility attributes

## Example Full Page Structure

```svelte
<script lang="ts">
  // ... component logic
</script>

<main class="container">
  <div class="content">
    <!-- Header -->
    <div class="header">
      <div class="icon-wrapper">
        <i class="nf-md-{icon}"></i>
      </div>
      <h1>{Tool Name}</h1>
      <p class="subtitle">{Description}</p>
    </div>

    <!-- Input Section -->
    <div class="input-section">
      <!-- Form content -->
    </div>

    <!-- Results Section -->
    {#if results}
      <div class="result-section">
        <!-- Results content -->
      </div>
    {/if}
  </div>
</main>

<style>
  /* Container styles */
  /* Header styles */
  /* Card styles */
  /* Button styles */
  /* Animation keyframes */
</style>
```

---

**Version**: 1.0  
**Last Updated**: 2026-01-28  
**Module Colors**: Text (Purple), PDF (Red), Converters (Teal), File (Blue)
