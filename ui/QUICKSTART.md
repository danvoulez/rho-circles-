# Rho OS UI Template System - Quick Start Guide

## 🚀 Getting Started in 5 Minutes

### Prerequisites
- Node.js 18+ and npm
- Modern browser (Chrome, Firefox, Safari, Edge)

### Installation

```bash
cd ui
npm install
```

### Development Mode

```bash
npm run dev
```

Opens automatically at `http://localhost:3000`

### What You'll See

1. **The Verifier Interface** - A clean, professional UI with:
   - Dark slate background (The Ledger)
   - Translucent glass panels with backdrop blur
   - Large drop zone for file verification
   - Educational "How It Works" section

2. **Try It Out:**
   - Create a test JSON file:
   ```bash
   echo '{"test": "data", "number": 123}' > test.json
   ```
   - Drag and drop it into the drop zone
   - Watch it calculate the Blake3 hash
   - See the verification result overlay

### Build for Production

```bash
npm run build
```

Output will be in `ui/dist/` directory.

### Preview Production Build

```bash
npm run preview
```

## 🎨 Design System Demo

### Component Examples

#### 1. Status Indicator
```tsx
import { StatusIndicator } from '@/components';

// Valid state (green, pulsing)
<StatusIndicator status="valid" label="Verified" />

// Invalid state (red, pulsing)
<StatusIndicator status="invalid" label="Failed" />

// Processing state (amber, pulsing)
<StatusIndicator status="processing" label="Verifying" />

// Immutable state (slate, pulsing)
<StatusIndicator status="immutable" label="Historical" />
```

#### 2. CID Badge
```tsx
import { CidBadge } from '@/components';

// Displays truncated CID, shows full on hover, copies on click
<CidBadge 
  cid="bafy2bzacedkqw7noat3dthat7aqcul4jxj6x5fqvkzqheruntvacrxxxxxx" 
  algorithm="blake3" 
/>
```

#### 3. JSON Viewer
```tsx
import { JsonViewer } from '@/components';

const data = { name: "Test", value: 123, valid: true };

// Human-readable mode (pretty-printed with colors)
<JsonViewer data={data} mode="human" />

// Canonical mode (key-sorted, no whitespace)
<JsonViewer data={data} mode="canonical" />
```

#### 4. Secure Drop Zone
```tsx
import { SecureDropZone } from '@/components';

<SecureDropZone 
  onFileProcessed={(result) => {
    console.log('CID:', result.cid);
    console.log('Filename:', result.filename);
    console.log('Size:', result.size, 'bytes');
  }}
  accept=".json,.pdf,image/*"
  maxSize={10 * 1024 * 1024}  // 10MB
/>
```

## 🎯 Testing the UI

### Test 1: File Drop and Hash Calculation

1. Start the dev server: `npm run dev`
2. Create a test file:
   ```bash
   echo '{"message": "Hello Rho OS"}' > test-receipt.json
   ```
3. Drag `test-receipt.json` into the drop zone
4. Watch the process:
   - "Reading file..."
   - "Calculating Blake3 Hash..."
   - "Complete"
5. See the verification overlay appear

### Test 2: Invalid File Rejection

1. Create a large file:
   ```bash
   dd if=/dev/zero of=large.bin bs=1M count=20
   ```
2. Try to drop it (should show "File too large" error)

### Test 3: Keyboard Interaction

1. Click the drop zone (should open file picker)
2. Use ESC to close verification overlay
3. Test responsive design (resize browser window)

## 🔧 Customization

### Change Theme Colors

Edit `ui/tailwind.config.js`:

```javascript
colors: {
  'rho-valid': '#10b981',      // Change green
  'rho-invalid': '#f43f5e',    // Change red
  'rho-processing': '#fbbf24', // Change amber
  'rho-immutable': '#94a3b8',  // Change slate
}
```

### Change Fonts

Edit `ui/src/styles/index.css`:

```css
@import url('https://fonts.googleapis.com/css2?family=YourFont&display=swap');

body {
  font-family: 'YourFont', sans-serif;
}
```

### Modify Glass Effect

Edit `ui/src/styles/index.css`:

```css
.glass-panel {
  backdrop-filter: blur(20px);  /* Increase blur */
  background: rgba(30, 41, 59, 0.9);  /* More opaque */
}
```

## 📦 Adding to Existing Products

### For API Notary

```bash
# In api-notary directory
mkdir -p ui
cp -r ../ui/src/components ./ui/
cp -r ../ui/src/hooks ./ui/
cp -r ../ui/src/types ./ui/
cp ../ui/tailwind.config.js ./
```

Then create your product-specific components using the library.

### For Content Sign

Same pattern - copy the component library and build your custom UI.

### For AI Passport

Same pattern - reuse components like StatusIndicator and CidBadge for compliance display.

## 🐛 Troubleshooting

### Issue: "Module not found"
**Solution:** Run `npm install` again

### Issue: Port 3000 already in use
**Solution:** Kill the process or change port in `vite.config.ts`:
```typescript
server: {
  port: 3001,
}
```

### Issue: Tailwind classes not working
**Solution:** Make sure `tailwind.config.js` content paths are correct

### Issue: WASM not loading
**Solution:** Check browser console for errors. The mock implementation should work without actual WASM.

## 📚 Next Steps

1. ✅ You've seen the public verifier interface
2. 🔜 Next: Build the client interface (form renderer)
3. 🔜 Then: Build the admin interface (ledger viewer)
4. 🔜 Finally: Connect to actual Rust WASM backend

## 🎓 Learning Resources

- **UI README:** `ui/README.md` - Complete design system
- **Product Examples:** `PRODUCT_UI_EXAMPLES.md` - Integration guides
- **Components:** `ui/src/components/` - Source code with comments

## 💡 Tips

1. **Use Glass Panels:** Wrap content in `<div className="glass-panel">`
2. **Use Monospace for CIDs:** `<code className="font-mono">{cid}</code>`
3. **Use Status Colors:** `text-rho-valid`, `text-rho-invalid`, etc.
4. **Use Framer Motion:** For smooth animations
5. **Keep It Modular:** Each component should be self-contained

## ✨ The Philosophy

> "Boring in reliability, Magic in speed"

The UI should make complex cryptographic operations feel simple and instantaneous.

---

**Need Help?** Check the comprehensive documentation in `ui/README.md` and `PRODUCT_UI_EXAMPLES.md`
