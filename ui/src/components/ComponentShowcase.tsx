// Component Showcase - Development Tool
// Displays all Rho OS components for visual testing

import { useState } from 'react';
import {
  CidBadge,
  StatusIndicator,
  JsonViewer,
  SecureDropZone,
} from '@/components';

export function ComponentShowcase() {
  const [selectedTab, setSelectedTab] = useState<string>('badges');

  const mockData = {
    name: 'Test Receipt',
    timestamp: '2026-02-14T09:30:00Z',
    values: [1, 2, 3, 4, 5],
    metadata: {
      issuer: 'Test Organization',
      valid: true,
    },
  };

  return (
    <div className="min-h-screen bg-slate-900 p-8">
      <div className="max-w-7xl mx-auto space-y-6">
        {/* Header */}
        <div className="glass-panel p-6">
          <h1 className="text-3xl font-bold text-slate-100">
            Rho OS Component Showcase
          </h1>
          <p className="text-slate-400 mt-2">
            Visual testing environment for all UI components
          </p>
        </div>

        {/* Tabs */}
        <div className="glass-panel p-4">
          <div className="flex gap-2">
            {['badges', 'status', 'json', 'dropzone'].map((tab) => (
              <button
                key={tab}
                onClick={() => setSelectedTab(tab)}
                className={`px-4 py-2 rounded transition-colors ${
                  selectedTab === tab
                    ? 'bg-rho-processing text-white'
                    : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
                }`}
              >
                {tab.charAt(0).toUpperCase() + tab.slice(1)}
              </button>
            ))}
          </div>
        </div>

        {/* CID Badges */}
        {selectedTab === 'badges' && (
          <div className="glass-panel p-6 space-y-6">
            <div>
              <h2 className="text-xl font-semibold text-slate-200 mb-4">
                CID Badge Component
              </h2>
              <p className="text-slate-400 mb-4">
                Displays Content Identifiers with copy-to-clipboard. Hover shows full CID, click copies.
              </p>
            </div>

            <div className="space-y-4">
              <div>
                <h3 className="text-sm text-slate-400 mb-2">Blake3 Algorithm</h3>
                <CidBadge
                  cid="bafy2bzacedkqw7noat3dthat7aqcul4jxj6x5fqvkzqheruntvacrxxxxxx"
                  algorithm="blake3"
                />
              </div>

              <div>
                <h3 className="text-sm text-slate-400 mb-2">Ed25519 Signature</h3>
                <CidBadge
                  cid="ed25519_abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                  algorithm="ed25519"
                />
              </div>

              <div>
                <h3 className="text-sm text-slate-400 mb-2">ML-DSA3 Post-Quantum</h3>
                <CidBadge
                  cid="mldsa3_xyz789abc456def123ghi789jkl012mno345pqr678stu901vwx234"
                  algorithm="mldsa3"
                />
              </div>

              <div>
                <h3 className="text-sm text-slate-400 mb-2">Full CID (No Truncation)</h3>
                <CidBadge
                  cid="short_cid_12345"
                  algorithm="blake3"
                  truncate={false}
                />
              </div>
            </div>
          </div>
        )}

        {/* Status Indicators */}
        {selectedTab === 'status' && (
          <div className="glass-panel p-6 space-y-6">
            <div>
              <h2 className="text-xl font-semibold text-slate-200 mb-4">
                Status Indicator Component
              </h2>
              <p className="text-slate-400 mb-4">
                Visual indicators for cryptographic verification status. Pulsing animation communicates state.
              </p>
            </div>

            <div className="space-y-6">
              {/* Valid */}
              <div className="p-4 bg-slate-800/50 rounded">
                <h3 className="text-sm text-slate-400 mb-3">Valid (Emerald-500)</h3>
                <div className="flex gap-8">
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Small</p>
                    <StatusIndicator status="valid" size="sm" label="Verified" />
                  </div>
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Medium</p>
                    <StatusIndicator status="valid" size="md" label="Valid Signature" />
                  </div>
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Large</p>
                    <StatusIndicator status="valid" size="lg" label="Cryptographically Valid" />
                  </div>
                </div>
              </div>

              {/* Invalid */}
              <div className="p-4 bg-slate-800/50 rounded">
                <h3 className="text-sm text-slate-400 mb-3">Invalid (Rose-500)</h3>
                <div className="flex gap-8">
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Small</p>
                    <StatusIndicator status="invalid" size="sm" label="Failed" />
                  </div>
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Medium</p>
                    <StatusIndicator status="invalid" size="md" label="Hash Mismatch" />
                  </div>
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Large</p>
                    <StatusIndicator status="invalid" size="lg" label="Verification Failed" />
                  </div>
                </div>
              </div>

              {/* Processing */}
              <div className="p-4 bg-slate-800/50 rounded">
                <h3 className="text-sm text-slate-400 mb-3">Processing (Amber-400)</h3>
                <div className="flex gap-8">
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Small</p>
                    <StatusIndicator status="processing" size="sm" label="Pending" />
                  </div>
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Medium</p>
                    <StatusIndicator status="processing" size="md" label="Normalizing" />
                  </div>
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Large</p>
                    <StatusIndicator status="processing" size="lg" label="Awaiting Confirmation" />
                  </div>
                </div>
              </div>

              {/* Immutable */}
              <div className="p-4 bg-slate-800/50 rounded">
                <h3 className="text-sm text-slate-400 mb-3">Immutable (Slate-400)</h3>
                <div className="flex gap-8">
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Small</p>
                    <StatusIndicator status="immutable" size="sm" label="Archived" />
                  </div>
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Medium</p>
                    <StatusIndicator status="immutable" size="md" label="Historical" />
                  </div>
                  <div>
                    <p className="text-xs text-slate-500 mb-2">Large</p>
                    <StatusIndicator status="immutable" size="lg" label="Cold Storage" />
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* JSON Viewer */}
        {selectedTab === 'json' && (
          <div className="glass-panel p-6 space-y-6">
            <div>
              <h2 className="text-xl font-semibold text-slate-200 mb-4">
                JSON Viewer Component
              </h2>
              <p className="text-slate-400 mb-4">
                Displays JSON with syntax highlighting in human or canonical mode.
              </p>
            </div>

            <div className="grid md:grid-cols-2 gap-6">
              <div>
                <h3 className="text-sm text-slate-400 mb-2">Human Mode (Pretty)</h3>
                <JsonViewer data={mockData} mode="human" />
              </div>

              <div>
                <h3 className="text-sm text-slate-400 mb-2">Canonical Mode (Verification)</h3>
                <JsonViewer data={mockData} mode="canonical" />
              </div>
            </div>

            <div className="p-4 bg-slate-800/50 rounded">
              <p className="text-sm text-slate-300">
                <strong>Human Mode:</strong> Pretty-printed with colors and indentation. Good for readability.
              </p>
              <p className="text-sm text-slate-300 mt-2">
                <strong>Canonical Mode:</strong> Keys sorted, no whitespace. This is what gets hashed for verification.
              </p>
            </div>
          </div>
        )}

        {/* Drop Zone */}
        {selectedTab === 'dropzone' && (
          <div className="glass-panel p-6 space-y-6">
            <div>
              <h2 className="text-xl font-semibold text-slate-200 mb-4">
                Secure Drop Zone Component
              </h2>
              <p className="text-slate-400 mb-4">
                Client-side file upload with Blake3 hash calculation. File never leaves the browser.
              </p>
            </div>

            <SecureDropZone
              onFileProcessed={(result) => {
                console.log('File processed:', result);
                alert(`File: ${result.filename}\nCID: ${result.cid}\nSize: ${result.size} bytes`);
              }}
              accept=".json,.pdf,.txt,image/*"
            />

            <div className="p-4 bg-slate-800/50 rounded space-y-2">
              <p className="text-sm text-slate-300">
                <strong>Zero Custody:</strong> File is read client-side only. Only the CID is transmitted.
              </p>
              <p className="text-sm text-slate-300">
                <strong>Try it:</strong> Drag any file (JSON, PDF, image) into the drop zone.
              </p>
              <p className="text-sm text-slate-300">
                <strong>Security:</strong> Max 10MB by default. Hash calculated with Blake3.
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
