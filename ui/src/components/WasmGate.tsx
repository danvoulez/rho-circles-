// WasmGate Component - Chapter 6.4
// Blocks rendering until WASM is loaded

import { type ReactNode } from 'react';
import { useRhoCore } from '@/hooks/useRhoCore';

interface WasmGateProps {
  children: ReactNode;
  fallback?: ReactNode;
}

export function WasmGate({ children, fallback }: WasmGateProps) {
  const { isReady } = useRhoCore();

  if (!isReady) {
    return (
      fallback || (
        <div className="flex items-center justify-center min-h-screen bg-slate-900">
          <div className="text-center space-y-4">
            <div className="inline-block w-16 h-16 border-4 border-slate-700 border-t-rho-processing rounded-full animate-spin" />
            <p className="font-mono text-slate-400">
              Initializing Cryptographic Core...
            </p>
          </div>
        </div>
      )
    );
  }

  return <>{children}</>;
}
