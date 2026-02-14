# Rho OS UI - Product Integration Examples

This document shows how to apply the Rho OS UI template to the three existing products in the repository.

## Overview

The three products ready for UI integration:
1. **product.api-notary** - HTTP wrapper for B2B API receipt generation
2. **product.content-sign** - CLI tool for content signing
3. **product.ai-passport** - AI model registry with compliance

---

## 1. API Notary Dashboard

### Purpose
Monitor and verify API transactions with cryptographic receipts.

### UI Components Needed
- Transaction list with status indicators
- CID badges for each transaction
- Real-time verification status
- Detail view with JSON viewer

### Example Implementation

```tsx
// products/api-notary/Dashboard.tsx
import { 
  CidBadge, 
  StatusIndicator, 
  JsonViewer 
} from '@/components';

interface ApiTransaction {
  id: string;
  method: string;
  path: string;
  timestamp: string;
  status_code: number;
  cid: string;
  status: 'valid' | 'invalid' | 'processing';
}

export function ApiNotaryDashboard() {
  const [transactions, setTransactions] = useState<ApiTransaction[]>([]);
  const [selectedTx, setSelectedTx] = useState<ApiTransaction | null>(null);

  return (
    <div className="min-h-screen bg-slate-900 p-8">
      <div className="max-w-7xl mx-auto space-y-6">
        {/* Header */}
        <div className="glass-panel p-6">
          <h1 className="text-3xl font-bold text-slate-100">
            API Notary Dashboard
          </h1>
          <p className="text-slate-400 mt-2">
            Cryptographic receipts for all API transactions
          </p>
        </div>

        {/* Transaction List */}
        <div className="glass-panel p-6">
          <h2 className="text-xl font-semibold text-slate-200 mb-4">
            Recent Transactions
          </h2>
          <div className="space-y-3">
            {transactions.map(tx => (
              <div
                key={tx.id}
                className="p-4 bg-slate-800/50 rounded hover:bg-slate-800 cursor-pointer transition-colors"
                onClick={() => setSelectedTx(tx)}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-4">
                    <StatusIndicator status={tx.status} />
                    <span className="font-mono text-slate-300">
                      {tx.method}
                    </span>
                    <span className="text-slate-400">{tx.path}</span>
                  </div>
                  <CidBadge cid={tx.cid} algorithm="blake3" />
                </div>
                <div className="text-xs text-slate-500 mt-2 font-mono">
                  {new Date(tx.timestamp).toLocaleString()}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Detail Panel */}
        {selectedTx && (
          <div className="glass-panel p-6">
            <h2 className="text-xl font-semibold text-slate-200 mb-4">
              Transaction Details
            </h2>
            <div className="grid grid-cols-2 gap-4 mb-4">
              <div>
                <label className="text-sm text-slate-400">Method</label>
                <p className="font-mono text-slate-200">{selectedTx.method}</p>
              </div>
              <div>
                <label className="text-sm text-slate-400">Status Code</label>
                <p className="font-mono text-slate-200">{selectedTx.status_code}</p>
              </div>
            </div>
            <JsonViewer data={selectedTx} mode="human" />
          </div>
        )}
      </div>
    </div>
  );
}
```

### Integration Steps

1. Create `products/api-notary/ui/` directory
2. Copy component library from `ui/src/components/`
3. Create `Dashboard.tsx` with above code
4. Connect to Rust backend API endpoints
5. Build: `npm run build`

---

## 2. Content Sign Studio

### Purpose
Sign content and generate verifiable receipts with visual proof.

### UI Components Needed
- Secure drop zone for files
- Signature form (author, timestamp)
- Live preview of signed content
- Download signed receipt

### Example Implementation

```tsx
// products/content-sign/Studio.tsx
import { 
  SecureDropZone, 
  JsonViewer, 
  CidBadge, 
  StatusIndicator 
} from '@/components';
import { useRhoCore } from '@/hooks/useRhoCore';

interface SignedContent {
  content_cid: string;
  author: string;
  timestamp: string;
  signature: string;
  status: 'valid';
}

export function ContentSignStudio() {
  const [file, setFile] = useState<{ cid: string; filename: string } | null>(null);
  const [author, setAuthor] = useState('');
  const [signed, setSigned] = useState<SignedContent | null>(null);
  const { normalize } = useRhoCore();

  const handleSign = async () => {
    if (!file || !author) return;

    const signedContent: SignedContent = {
      content_cid: file.cid,
      author,
      timestamp: new Date().toISOString(),
      signature: 'ed25519_mock_signature',
      status: 'valid',
    };

    setSigned(signedContent);
  };

  return (
    <div className="min-h-screen bg-slate-900 p-8">
      <div className="max-w-5xl mx-auto space-y-6">
        {/* Header */}
        <div className="glass-panel p-6 text-center">
          <h1 className="text-4xl font-bold text-slate-100">
            Content Sign Studio
          </h1>
          <p className="text-slate-400 mt-2">
            Create cryptographic proof of content authenticity
          </p>
        </div>

        {/* Two Column Layout */}
        <div className="grid md:grid-cols-2 gap-6">
          {/* Left: Input */}
          <div className="space-y-6">
            <div className="glass-panel p-6">
              <h2 className="text-xl font-semibold text-slate-200 mb-4">
                1. Upload Content
              </h2>
              <SecureDropZone
                onFileProcessed={setFile}
                accept=".json,.txt,.pdf,image/*"
              />
              {file && (
                <div className="mt-4 p-3 bg-slate-800/50 rounded">
                  <p className="text-sm text-slate-400">File: {file.filename}</p>
                  <CidBadge cid={file.cid} algorithm="blake3" />
                </div>
              )}
            </div>

            <div className="glass-panel p-6">
              <h2 className="text-xl font-semibold text-slate-200 mb-4">
                2. Author Information
              </h2>
              <input
                type="text"
                placeholder="Your name or organization"
                className="w-full px-4 py-2 bg-slate-800 border border-slate-700 rounded text-slate-200 focus:border-rho-processing focus:outline-none"
                value={author}
                onChange={(e) => setAuthor(e.target.value)}
              />
              <button
                className="w-full mt-4 px-6 py-3 bg-rho-valid hover:bg-rho-valid/80 text-white font-semibold rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                onClick={handleSign}
                disabled={!file || !author}
              >
                Sign Content
              </button>
            </div>
          </div>

          {/* Right: Output */}
          <div className="glass-panel p-6">
            <h2 className="text-xl font-semibold text-slate-200 mb-4">
              Signed Receipt
            </h2>
            {signed ? (
              <div className="space-y-4">
                <StatusIndicator status="valid" label="Signed" size="lg" />
                <JsonViewer data={signed} mode="canonical" />
                <button
                  className="w-full px-6 py-3 bg-slate-700 hover:bg-slate-600 text-white font-semibold rounded transition-colors"
                  onClick={() => {
                    const blob = new Blob([JSON.stringify(signed, null, 2)]);
                    const url = URL.createObjectURL(blob);
                    const a = document.createElement('a');
                    a.href = url;
                    a.download = `signed-${Date.now()}.json`;
                    a.click();
                  }}
                >
                  Download Receipt
                </button>
              </div>
            ) : (
              <div className="text-center text-slate-500 py-12">
                Sign content to generate receipt
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
```

### Integration Steps

1. Create `products/content-sign/ui/` directory
2. Copy component library
3. Create `Studio.tsx` with above code
4. Connect to Rust CLI via WASM
5. Build: `npm run build`

---

## 3. AI Passport Registry

### Purpose
Register AI models with compliance metrics and bias scores.

### UI Components Needed
- Model registration form
- Bias metrics display with validation
- Compliance status indicators
- Model passport viewer

### Example Implementation

```tsx
// products/ai-passport/Registry.tsx
import { 
  StatusIndicator, 
  CidBadge, 
  JsonViewer 
} from '@/components';

interface BiasMetrics {
  demographic_parity: number;  // 0-10000 (0-100%)
  equal_opportunity: number;
  fairness_score: number;
  toxicity_score?: number;
}

interface AiPassport {
  model_name: string;
  model_cid: string;
  bias_metrics: BiasMetrics;
  compliance_status: 'valid' | 'invalid';
  timestamp: string;
}

function BiasMetricBar({ 
  label, 
  value, 
  threshold, 
  inverse = false 
}: { 
  label: string; 
  value: number; 
  threshold: number; 
  inverse?: boolean;
}) {
  const percentage = value / 100;
  const passes = inverse 
    ? value <= threshold 
    : value >= threshold;

  return (
    <div className="space-y-2">
      <div className="flex justify-between items-center">
        <span className="text-sm text-slate-400">{label}</span>
        <span className={`font-mono text-sm ${passes ? 'text-rho-valid' : 'text-rho-invalid'}`}>
          {percentage.toFixed(2)}%
        </span>
      </div>
      <div className="h-2 bg-slate-800 rounded-full overflow-hidden">
        <div
          className={`h-full transition-all ${passes ? 'bg-rho-valid' : 'bg-rho-invalid'}`}
          style={{ width: `${percentage}%` }}
        />
      </div>
      <p className="text-xs text-slate-500">
        Threshold: {(threshold / 100).toFixed(2)}%
      </p>
    </div>
  );
}

export function AiPassportRegistry() {
  const [passports, setPassports] = useState<AiPassport[]>([]);
  const [selected, setSelected] = useState<AiPassport | null>(null);

  return (
    <div className="min-h-screen bg-slate-900 p-8">
      <div className="max-w-7xl mx-auto space-y-6">
        {/* Header */}
        <div className="glass-panel p-6">
          <h1 className="text-3xl font-bold text-slate-100">
            AI Passport Registry
          </h1>
          <p className="text-slate-400 mt-2">
            EU AI Act Compliance Certification
          </p>
        </div>

        {/* Model List */}
        <div className="grid md:grid-cols-3 gap-4">
          {passports.map(passport => (
            <div
              key={passport.model_cid}
              className="glass-panel p-6 cursor-pointer hover:border-slate-600 transition-colors"
              onClick={() => setSelected(passport)}
            >
              <div className="flex items-start justify-between mb-4">
                <h3 className="font-semibold text-slate-200">
                  {passport.model_name}
                </h3>
                <StatusIndicator status={passport.compliance_status} size="sm" />
              </div>
              <CidBadge cid={passport.model_cid} algorithm="blake3" />
              <div className="mt-4 text-xs text-slate-500 font-mono">
                {new Date(passport.timestamp).toLocaleDateString()}
              </div>
            </div>
          ))}
        </div>

        {/* Detail View */}
        {selected && (
          <div className="glass-panel p-6">
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-2xl font-semibold text-slate-200">
                {selected.model_name}
              </h2>
              <StatusIndicator 
                status={selected.compliance_status} 
                label={selected.compliance_status === 'valid' ? 'Compliant' : 'Non-Compliant'}
                size="lg" 
              />
            </div>

            {/* Bias Metrics */}
            <div className="grid md:grid-cols-2 gap-6 mb-6">
              <div className="space-y-4">
                <h3 className="text-lg font-semibold text-slate-300">
                  Bias Metrics
                </h3>
                <BiasMetricBar
                  label="Demographic Parity"
                  value={selected.bias_metrics.demographic_parity}
                  threshold={2000}
                  inverse
                />
                <BiasMetricBar
                  label="Equal Opportunity"
                  value={selected.bias_metrics.equal_opportunity}
                  threshold={8000}
                />
                <BiasMetricBar
                  label="Fairness Score"
                  value={selected.bias_metrics.fairness_score}
                  threshold={7000}
                />
                {selected.bias_metrics.toxicity_score && (
                  <BiasMetricBar
                    label="Toxicity Score"
                    value={selected.bias_metrics.toxicity_score}
                    threshold={3000}
                    inverse
                  />
                )}
              </div>

              <div>
                <h3 className="text-lg font-semibold text-slate-300 mb-4">
                  Full Passport
                </h3>
                <JsonViewer data={selected} mode="human" />
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
```

### Integration Steps

1. Create `products/ai-passport/ui/` directory
2. Copy component library
3. Create `Registry.tsx` with above code
4. Connect to Rust backend API
5. Build: `npm run build`

---

## General Integration Pattern

For any new product, follow this pattern:

```tsx
import { WasmGate } from '@/components';
import { YourProductUI } from './YourProductUI';

function App() {
  return (
    <WasmGate>
      <div className="min-h-screen bg-slate-900">
        <YourProductUI />
      </div>
    </WasmGate>
  );
}

export default App;
```

### Checklist

- [ ] Copy `ui/src/components/` to your product
- [ ] Copy `ui/src/types/` for TypeScript definitions
- [ ] Copy `ui/src/hooks/useRhoCore.ts` for WASM
- [ ] Copy `ui/src/styles/index.css` for global styles
- [ ] Copy `tailwind.config.js` for Rho OS theme
- [ ] Install dependencies from `package.json`
- [ ] Create product-specific components
- [ ] Connect to Rust backend
- [ ] Test build and deployment

---

## Deployment

All three products can share the same UI infrastructure:

```bash
# Build all products
cd ui && npm run build

# Copy build to products
cp -r dist/ ../products/api-notary/public/
cp -r dist/ ../products/content-sign/public/
cp -r dist/ ../products/ai-passport/public/
```

Or serve as a unified dashboard with routing:

```tsx
import { BrowserRouter, Routes, Route } from 'react-router-dom';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HeroValidator />} />
        <Route path="/api-notary" element={<ApiNotaryDashboard />} />
        <Route path="/content-sign" element={<ContentSignStudio />} />
        <Route path="/ai-passport" element={<AiPassportRegistry />} />
      </Routes>
    </BrowserRouter>
  );
}
```

---

## Next Steps

1. Choose a product to integrate first (recommend starting with api-notary)
2. Set up the UI directory structure
3. Copy the component library
4. Implement product-specific UI
5. Connect to Rust backend
6. Test and iterate
7. Repeat for other products

The UI template is production-ready and can be applied to any Rho Circles product!
