// Rho Circles WASM Hook
// This hook provides access to the core Rho Circles functionality compiled to WASM

import { useState, useEffect } from 'react';
import * as RhoWasm from '../../../packages/rho-wasm/index';

interface RhoCore {
  normalize: (input: any) => Promise<{ normalized: string; cid: string }>;
  validate: (value: any, schemaCid: string) => Promise<{ ok: boolean; errors?: string[] }>;
  hash: (data: Uint8Array) => Promise<string>;
  isReady: boolean;
}

export function useRhoCore(): RhoCore {
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    console.log('[RhoCore] Initializing Cryptographic Core...');
    RhoWasm.initialize()
      .then(() => {
        console.log('[RhoCore] Cryptographic Core initialized successfully');
        setIsReady(true);
      })
      .catch((error) => {
        console.error('[RhoCore] Failed to initialize:', error);
      });
  }, []);

  return {
    normalize: async (input: any) => {
      return await RhoWasm.normalize(input);
    },
    validate: async (value: any, schemaCid: string) => {
      return await RhoWasm.validate(schemaCid, value);
    },
    hash: async (data: Uint8Array) => {
      return await RhoWasm.cid(data);
    },
    isReady,
  };
}
