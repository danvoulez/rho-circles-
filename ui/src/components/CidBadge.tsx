// CidBadge Component - Chapter 6.1
// Displays Content Identifiers with copy-to-clipboard functionality

import { useState } from 'react';
import type { SignatureAlgorithm } from '@/types';

interface CidBadgeProps {
  cid: string;
  algorithm: SignatureAlgorithm;
  truncate?: boolean;
}

export function CidBadge({ cid, algorithm, truncate = true }: CidBadgeProps) {
  const [copied, setCopied] = useState(false);
  const [showFull, setShowFull] = useState(false);

  const displayCid = truncate && !showFull
    ? `${cid.slice(0, 8)}...${cid.slice(-6)}`
    : cid;

  const handleCopy = async () => {
    await navigator.clipboard.writeText(cid);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div
      className="inline-flex items-center gap-2 px-3 py-1.5 bg-slate-800 border border-slate-700 rounded-full cursor-pointer hover:bg-slate-700 transition-colors"
      onClick={handleCopy}
      onMouseEnter={() => setShowFull(true)}
      onMouseLeave={() => setShowFull(false)}
      title={`Click to copy full CID\n${cid}`}
    >
      <span className="text-xs text-slate-400 uppercase font-medium">{algorithm}</span>
      <div className="w-px h-4 bg-slate-600" />
      <code className="font-mono text-sm text-slate-200">{displayCid}</code>
      {copied && (
        <span className="text-xs text-rho-valid">✓ Copied</span>
      )}
    </div>
  );
}
