// ProofOverlay Component - Chapter 3.2
// Displays verification result as a full-screen overlay

import { motion } from 'framer-motion';
import { StatusIndicator } from '@/components/StatusIndicator';
import { CidBadge } from '@/components/CidBadge';
import type { VerificationResult } from '@/types';

interface ProofOverlayProps {
  result: VerificationResult;
  onClose: () => void;
}

export function ProofOverlay({ result, onClose }: ProofOverlayProps) {
  const isValid = result.valid;

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 bg-slate-900/95 backdrop-blur-lg z-50 flex items-center justify-center p-8"
      onClick={onClose}
    >
      <motion.div
        initial={{ scale: 0.9, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        exit={{ scale: 0.9, opacity: 0 }}
        className="glass-panel p-12 max-w-3xl w-full"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="text-center space-y-6 mb-8">
          {isValid ? (
            <>
              <div className="text-8xl">✓</div>
              <h2 className="text-4xl font-bold text-rho-valid">
                Verified Artifact
              </h2>
            </>
          ) : (
            <>
              <div className="text-8xl">✗</div>
              <h2 className="text-4xl font-bold text-rho-invalid">
                Verification Failed
              </h2>
            </>
          )}
        </div>

        {/* Status */}
        <div className="flex justify-center mb-8">
          <StatusIndicator status={result.status} size="lg" label={result.status.toUpperCase()} />
        </div>

        {/* Metadata */}
        {isValid ? (
          <div className="space-y-4 mb-8">
            {result.issuer && (
              <div className="flex justify-between items-center p-4 bg-slate-800/50 rounded">
                <span className="text-slate-400">Issuer:</span>
                <span className="font-semibold text-slate-200">{result.issuer}</span>
              </div>
            )}
            {result.timestamp && (
              <div className="flex justify-between items-center p-4 bg-slate-800/50 rounded">
                <span className="text-slate-400">Registered:</span>
                <span className="font-mono text-slate-200">
                  {new Date(result.timestamp).toLocaleString()}
                </span>
              </div>
            )}
            {result.chip && (
              <div className="flex justify-between items-center p-4 bg-slate-800/50 rounded">
                <span className="text-slate-400">Chip:</span>
                <span className="font-semibold text-slate-200">{result.chip}</span>
              </div>
            )}
            {result.metadata?.cid && (
              <div className="flex justify-between items-center p-4 bg-slate-800/50 rounded">
                <span className="text-slate-400">Content ID:</span>
                <CidBadge cid={result.metadata.cid} algorithm="blake3" truncate={false} />
              </div>
            )}
          </div>
        ) : (
          <div className="space-y-4 mb-8">
            <div className="p-4 bg-rho-invalid/10 border border-rho-invalid/30 rounded">
              <h3 className="font-semibold text-rho-invalid mb-2">Failure Reasons:</h3>
              <ul className="list-disc list-inside space-y-1 text-slate-300">
                {result.errors?.map((error, idx) => (
                  <li key={idx} className="font-mono text-sm">{error}</li>
                ))}
              </ul>
            </div>
          </div>
        )}

        {/* Actions */}
        <div className="flex gap-4">
          {isValid && (
            <button
              className="flex-1 px-6 py-3 bg-rho-valid hover:bg-rho-valid/80 text-white font-semibold rounded transition-colors"
              onClick={() => {
                // Download proof bundle
                console.log('Download proof bundle');
              }}
            >
              Download Proof Bundle
            </button>
          )}
          <button
            className="flex-1 px-6 py-3 bg-slate-700 hover:bg-slate-600 text-white font-semibold rounded transition-colors"
            onClick={onClose}
          >
            Close
          </button>
        </div>
      </motion.div>
    </motion.div>
  );
}
