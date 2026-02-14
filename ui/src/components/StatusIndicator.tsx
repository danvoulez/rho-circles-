// StatusIndicator Component - Chapter 6.2
// Visual indicator for cryptographic verification status

import type { CryptoStatus } from '@/types';

interface StatusIndicatorProps {
  status: CryptoStatus;
  label?: string;
  size?: 'sm' | 'md' | 'lg';
}

const statusConfig = {
  valid: {
    color: 'bg-rho-valid',
    ring: 'ring-rho-valid/30',
    text: 'Valid',
    description: 'Cryptographically Valid',
  },
  invalid: {
    color: 'bg-rho-invalid',
    ring: 'ring-rho-invalid/30',
    text: 'Invalid',
    description: 'Verification Failed',
  },
  processing: {
    color: 'bg-rho-processing',
    ring: 'ring-rho-processing/30',
    text: 'Processing',
    description: 'Awaiting Confirmation',
  },
  immutable: {
    color: 'bg-rho-immutable',
    ring: 'ring-rho-immutable/30',
    text: 'Immutable',
    description: 'Historical Record',
  },
};

const sizeConfig = {
  sm: { dot: 'w-2 h-2', ring: 'ring-2', text: 'text-xs' },
  md: { dot: 'w-3 h-3', ring: 'ring-4', text: 'text-sm' },
  lg: { dot: 'w-4 h-4', ring: 'ring-4', text: 'text-base' },
};

export function StatusIndicator({ status, label, size = 'md' }: StatusIndicatorProps) {
  const config = statusConfig[status];
  const sizes = sizeConfig[size];

  return (
    <div className="inline-flex items-center gap-2">
      <div className="relative">
        <div
          className={`${sizes.dot} ${config.color} rounded-full animate-pulse`}
        />
        <div
          className={`absolute inset-0 rounded-full ${config.color} ${config.ring} ${sizes.ring} animate-glow`}
        />
      </div>
      {label && (
        <span className={`${sizes.text} font-medium text-slate-200`}>
          {label || config.text}
        </span>
      )}
    </div>
  );
}
