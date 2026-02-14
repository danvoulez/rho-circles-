// HeroValidator Component - Chapter 3.1
// The main public verification interface

import { useState } from 'react';
import { SecureDropZone } from '@/components/SecureDropZone';
import { ProofOverlay } from '@/components/ProofOverlay';
import type { VerificationResult } from '@/types';

export function HeroValidator() {
  const [verificationResult, setVerificationResult] = useState<VerificationResult | null>(null);
  const [isVerifying, setIsVerifying] = useState(false);

  const handleFileProcessed = async (result: { cid: string; filename: string; size: number }) => {
    setIsVerifying(true);

    // Simulate verification process
    // In production, this would query the ledger
    await new Promise(resolve => setTimeout(resolve, 1500));

    // Mock verification result
    const mockResult: VerificationResult = {
      valid: Math.random() > 0.3,
      status: Math.random() > 0.3 ? 'valid' : 'invalid',
      issuer: 'Universidade XYZ',
      timestamp: '2026-02-14T09:30:00Z',
      chip: 'Academic Degree V1',
      metadata: {
        cid: result.cid,
        filename: result.filename,
        size: result.size,
      },
    };

    if (!mockResult.valid) {
      mockResult.errors = ['Hash Mismatch', 'Content has been modified'];
    }

    setVerificationResult(mockResult);
    setIsVerifying(false);
  };

  return (
    <div className="min-h-screen bg-slate-900 flex items-center justify-center p-8">
      <div className="max-w-4xl w-full space-y-8">
        {/* Header */}
        <div className="text-center space-y-4">
          <h1 className="text-5xl font-bold text-slate-100">
            Rho OS Verifier
          </h1>
          <p className="text-xl text-slate-400">
            Verify the authenticity of any cryptographic proof
          </p>
          <p className="text-sm text-slate-500 font-mono">
            Zero-Knowledge · Content-Addressable · Immutable
          </p>
        </div>

        {/* The Vortex - Drop Zone */}
        <div className="glass-panel p-8">
          <SecureDropZone onFileProcessed={handleFileProcessed} />
        </div>

        {/* Processing Indicator */}
        {isVerifying && (
          <div className="glass-panel p-6 text-center">
            <div className="space-y-3">
              <div className="inline-block w-8 h-8 border-4 border-slate-700 border-t-rho-processing rounded-full animate-spin" />
              <div className="space-y-1">
                <p className="font-mono text-rho-processing">Calculating Blake3 Hash...</p>
                <p className="font-mono text-slate-400 text-sm">Normalizing Structure...</p>
                <p className="font-mono text-slate-400 text-sm">Consulting Ledger...</p>
              </div>
            </div>
          </div>
        )}

        {/* How It Works */}
        <div className="glass-panel p-6">
          <h3 className="text-lg font-semibold text-slate-200 mb-4">How It Works</h3>
          <div className="grid md:grid-cols-3 gap-4">
            <div className="space-y-2">
              <div className="text-3xl">🔐</div>
              <h4 className="font-semibold text-slate-300">1. Hash</h4>
              <p className="text-sm text-slate-400">
                Calculate cryptographic fingerprint using Blake3
              </p>
            </div>
            <div className="space-y-2">
              <div className="text-3xl">📚</div>
              <h4 className="font-semibold text-slate-300">2. Verify</h4>
              <p className="text-sm text-slate-400">
                Check signature against immutable ledger
              </p>
            </div>
            <div className="space-y-2">
              <div className="text-3xl">✅</div>
              <h4 className="font-semibold text-slate-300">3. Confirm</h4>
              <p className="text-sm text-slate-400">
                Get instant cryptographic proof of authenticity
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Proof Overlay */}
      {verificationResult && (
        <ProofOverlay
          result={verificationResult}
          onClose={() => setVerificationResult(null)}
        />
      )}
    </div>
  );
}
