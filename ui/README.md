# Rho OS UI Template System

**Version:** 1.0.0  
**Status:** Production Ready  
**Architecture:** Glass & Ledger Philosophy

---

## 📘 Overview

The Rho OS UI Template is a modular, reusable interface system designed for cryptographic verification applications. It implements the "Glass & Ledger" design philosophy where translucent, interactive panels (Glass) float over immutable data streams (Ledger).

## 🎨 Design Philosophy

### Glass & Ledger Concept

- **Glass (Interactive Layer):** Translucent panels using `backdrop-filter: blur(12px)` representing the "Now" - current interactions
- **Ledger (Base Layer):** Dark, dense background representing the "Historical" - immutable records

### Typography Semantics

- **Human UI:** Inter (sans-serif) - For labels, buttons, descriptions
- **Machine UI:** JetBrains Mono (monospace) - **Required** for:
  - CIDs (Content Identifiers)
  - Hashes (Blake3, Ed25519)
  - JSON payloads
  - Console logs
  - Financial/critical numbers

### Color Coding (The Traffic Light of Truth)

- **Emerald-500 (#10b981):** Cryptographically Valid - Signature verified, policy approved
- **Rose-500 (#f43f5e):** Invalid - Hash mismatch, signature failed, policy rejected
- **Amber-400 (#fbbf24):** Processing - Normalizing, awaiting confirmation, unverified state
- **Slate-400 (#94a3b8):** Immutable - Historical data, cold storage

---

## 🏗️ Architecture

### Tech Stack

- **Runtime:** React 18+ with Vite
- **Language:** TypeScript (Strict Mode)
- **State:** TanStack Query + Zustand
- **Styling:** Tailwind CSS + Framer Motion
- **Core Logic:** rho-circles-wasm (Rust compiled)

### Directory Structure

```
ui/
├── src/
│   ├── components/        # Reusable UI components
│   │   ├── CidBadge.tsx          # CID display with copy
│   │   ├── StatusIndicator.tsx   # Crypto status dots
│   │   ├── JsonViewer.tsx        # Syntax-highlighted JSON
│   │   ├── WasmGate.tsx          # WASM loading wrapper
│   │   ├── SecureDropZone.tsx    # File upload with hashing
│   │   ├── HeroValidator.tsx     # Public verifier interface
│   │   └── ProofOverlay.tsx      # Verification result display
│   ├── hooks/            # React hooks
│   │   └── useRhoCore.ts        # WASM integration
│   ├── types/            # TypeScript definitions
│   │   └── index.ts
│   ├── styles/           # Global styles
│   │   └── index.css
│   ├── App.tsx           # Main application
│   └── main.tsx          # Entry point
├── index.html
├── package.json
├── tsconfig.json
├── vite.config.ts
└── tailwind.config.js
```

---

## 🧩 Component Library

### Atomic Components

#### 1. CidBadge
**Purpose:** Display Content Identifiers with copy-to-clipboard

```tsx
import { CidBadge } from '@/components';

<CidBadge 
  cid="bafy2bzacedkqw7..." 
  algorithm="blake3" 
  truncate={true} 
/>
```

**Features:**
- Hover shows full CID
- Click copies to clipboard
- Monospace font styling
- Algorithm label

#### 2. StatusIndicator
**Purpose:** Visual indicator for cryptographic verification status

```tsx
import { StatusIndicator } from '@/components';

<StatusIndicator 
  status="valid" 
  label="Verified" 
  size="md" 
/>
```

**Status Types:** `valid`, `invalid`, `processing`, `immutable`

#### 3. JsonViewer
**Purpose:** Display JSON with syntax highlighting

```tsx
import { JsonViewer } from '@/components';

<JsonViewer 
  data={myObject} 
  mode="human"  // or "canonical"
  maxHeight="400px" 
/>
```

**Modes:**
- `human`: Pretty-printed with colors
- `canonical`: Minimal, key-sorted (for verification)

#### 4. WasmGate
**Purpose:** Block rendering until WASM module is loaded

```tsx
import { WasmGate } from '@/components';

<WasmGate>
  <YourApp />
</WasmGate>
```

Shows loading spinner with message: "Initializing Cryptographic Core..."

#### 5. SecureDropZone
**Purpose:** File upload with client-side hash calculation

```tsx
import { SecureDropZone } from '@/components';

<SecureDropZone 
  onFileProcessed={(result) => {
    console.log(result.cid, result.filename, result.size);
  }}
  accept=".json,.pdf,image/*"
  maxSize={10 * 1024 * 1024}
/>
```

**Key Feature:** File **never** uploaded automatically - only CID calculated locally

---

## 🌍 The Three Interfaces

### 1. Public Interface - The Verifier

**Objective:** Allow anyone to verify authenticity without login

**Components:**
- `HeroValidator`: Main verification screen with drop zone
- `ProofOverlay`: Full-screen verification result

**Flow:**
1. User drops file (receipt, PDF, image)
2. System calculates Blake3 hash
3. Queries ledger for verification
4. Shows verification result overlay

**URL:** `/` (home page)

### 2. Client Interface - The Issuer

**Objective:** Environment where "Intention" becomes "Proof"

**Components (Future):**
- Cockpit layout with sidebar and header
- UniversalFormRenderer (dynamic forms from JSON Schema)
- LiveCanonPreview (real-time canonical JSON)
- ReceiptWallet (receipt management)

**URL:** `/client`

### 3. Admin Interface - The Auditor

**Objective:** Total visibility, forensic auditing, rule management

**Components (Future):**
- GlobalLedgerFeed (virtualized data grid)
- DeepInspector (detailed inspection panel)
- PolicyStudio (visual policy editor)

**URL:** `/admin`

---

## 🚀 Quick Start

### Installation

```bash
cd ui
npm install
```

### Development

```bash
npm run dev
```

Opens browser at `http://localhost:3000`

### Build

```bash
npm run build
```

Output in `ui/dist/`

### Preview Production Build

```bash
npm run preview
```

---

## 🔌 Integration Guide

### Applying Template to New Products

The UI template is designed to be modular and reusable. Here's how to apply it to new products:

#### Step 1: Install Dependencies

```bash
cd your-product/
npm install react react-dom framer-motion @tanstack/react-query zustand
npm install -D @types/react @types/react-dom vite @vitejs/plugin-react typescript tailwindcss
```

#### Step 2: Copy Core Files

Copy these files from `ui/`:
- `tailwind.config.js` (Rho OS theme)
- `tsconfig.json` (TypeScript config)
- `vite.config.ts` (Vite config)
- `src/types/index.ts` (Core types)
- `src/hooks/useRhoCore.ts` (WASM integration)
- `src/components/` (All atomic components)
- `src/styles/index.css` (Global styles)

#### Step 3: Customize for Your Product

**For API Notary:**
```tsx
import { CidBadge, StatusIndicator } from '@/components';

function ApiNotaryDashboard() {
  return (
    <div className="glass-panel">
      <h1 className="font-sans">API Transactions</h1>
      {/* Your product-specific UI */}
    </div>
  );
}
```

**For Content Sign:**
```tsx
import { SecureDropZone, JsonViewer } from '@/components';

function ContentSignApp() {
  return (
    <div className="glass-panel">
      <SecureDropZone onFileProcessed={handleSign} />
      <JsonViewer data={signedContent} mode="canonical" />
    </div>
  );
}
```

**For AI Passport:**
```tsx
import { StatusIndicator, CidBadge } from '@/components';

function AiPassportRegistry() {
  return (
    <div className="glass-panel">
      <StatusIndicator status="valid" label="Compliant" />
      <CidBadge cid={modelCid} algorithm="blake3" />
    </div>
  );
}
```

---

## 📐 Design Guidelines

### Use Glass Panels

```tsx
<div className="glass-panel p-6">
  {/* Your content */}
</div>
```

### Use Machine Text for CIDs

```tsx
<code className="font-mono text-slate-200">
  {cid}
</code>
```

### Use Color Coding

```tsx
// Valid state
<span className="text-rho-valid">✓ Verified</span>

// Invalid state
<span className="text-rho-invalid">✗ Failed</span>

// Processing
<span className="text-rho-processing">⟳ Processing</span>
```

### Animation

```tsx
import { motion } from 'framer-motion';

<motion.div
  initial={{ opacity: 0, y: 20 }}
  animate={{ opacity: 1, y: 0 }}
  transition={{ duration: 0.3 }}
>
  {/* Content */}
</motion.div>
```

---

## 🎯 Implementation Phases

### ✅ Phase 1: Foundation (Complete)
- [x] Project setup (Vite + React + TypeScript)
- [x] Tailwind configuration with Rho OS theme
- [x] Core type definitions
- [x] WASM integration hook

### ✅ Phase 2: Atomic Components (Complete)
- [x] CidBadge
- [x] StatusIndicator
- [x] JsonViewer
- [x] WasmGate
- [x] SecureDropZone

### ✅ Phase 3: Public Verifier (Complete)
- [x] HeroValidator
- [x] ProofOverlay
- [x] Public verification page

### ⏳ Phase 4: Client Interface (Next)
- [ ] Cockpit layout
- [ ] UniversalFormRenderer
- [ ] LiveCanonPreview
- [ ] ReceiptWallet

### ⏳ Phase 5: Admin Interface (Future)
- [ ] GlobalLedgerFeed
- [ ] DeepInspector
- [ ] PolicyStudio

---

## 🔒 Zero Custody Architecture

The UI follows the **Zero Custody** principle:

1. **Input:** User enters data or drops file
2. **Normalize (Client):** Browser runs `rho.normalize(input)` via WASM
3. **Hash (Client):** Browser calculates CID using Blake3
4. **Sign (Client):** Private key (LocalStorage/Extension) signs CID
5. **Submit:** Browser sends `{ cid, signature, public_key }` to node
6. **Store (Optional):** Blob sent to CAS only if not private

**The server never sees raw data** unless explicitly public.

---

## 📚 Examples

### Example 1: Simple Verifier

```tsx
import { HeroValidator, WasmGate } from '@/components';

function App() {
  return (
    <WasmGate>
      <HeroValidator />
    </WasmGate>
  );
}
```

### Example 2: Custom Product Dashboard

```tsx
import { CidBadge, StatusIndicator, JsonViewer } from '@/components';

function ProductDashboard({ data }) {
  return (
    <div className="min-h-screen bg-slate-900 p-8">
      <div className="glass-panel p-6 space-y-4">
        <h1 className="text-2xl font-bold text-slate-100">
          My Product
        </h1>
        
        <StatusIndicator status={data.status} />
        
        <CidBadge cid={data.cid} algorithm="blake3" />
        
        <JsonViewer data={data.content} mode="human" />
      </div>
    </div>
  );
}
```

---

## 🤝 Contributing

When adding new components:

1. Follow the Glass & Ledger philosophy
2. Use semantic typography (Inter for humans, JetBrains Mono for machines)
3. Apply color coding for cryptographic states
4. Ensure components are reusable and product-agnostic
5. Add TypeScript types
6. Document usage with examples

---

## 📝 License

Part of the Rho Circles project. See root LICENSE file.

---

## 🎓 Philosophy

> "The Rho OS must be **'Boring'** in reliability, but **'Magic'** in verification speed. Users shouldn't feel they're using Blockchain or heavy Cryptography; they should feel like they're using the Excel of the Future."

The UI makes cryptographic verification feel instantaneous, transparent, and trustworthy - without exposing the complexity underneath.
