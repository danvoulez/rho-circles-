// Rho Core WASM Wrapper
// Clean TypeScript API for the Rho Circles core functionality

import init, { wasm_normalize, wasm_validate, wasm_cid, init as wasmInit } from './rho_core.js';

export interface NormalizeOutput {
  normalized: string;
  cid: string;
}

export interface ValidateOutput {
  ok: boolean;
  errors?: string[];
}

let isInitialized = false;
let initPromise: Promise<void> | null = null;

/**
 * Initialize the WASM module
 * Must be called before using any other functions
 */
export async function initialize(): Promise<void> {
  // Check for existing initialization first to avoid race conditions
  if (initPromise) {
    await initPromise;
    return;
  }
  
  if (isInitialized) return;

  initPromise = (async () => {
    try {
      // Initialize the WASM module
      await init();
      
      // Call the init function from the WASM module
      wasmInit();
      
      isInitialized = true;
    } catch (error) {
      console.error('[RhoCore] Failed to initialize WASM:', error);
      throw error;
    }
  })();

  await initPromise;
}

/**
 * Normalize a JSON value to canonical form
 * 
 * @param input - Any JSON-serializable value
 * @returns Object with normalized string and CID
 */
export async function normalize(input: unknown): Promise<NormalizeOutput> {
  await initialize();
  
  try {
    const result = wasm_normalize(input);
    return result as NormalizeOutput;
  } catch (error) {
    throw new Error(`Normalization failed: ${error}`);
  }
}

/**
 * Validate a JSON value against a schema
 * 
 * @param schemaCid - CID of the schema to validate against
 * @param input - Value to validate
 * @returns Validation result with ok flag and optional errors
 */
export async function validate(schemaCid: string, input: unknown): Promise<ValidateOutput> {
  await initialize();
  
  try {
    const result = wasm_validate(schemaCid, input);
    return result as ValidateOutput;
  } catch (error) {
    throw new Error(`Validation failed: ${error}`);
  }
}

/**
 * Compute CID from raw bytes
 * 
 * @param bytes - Raw bytes to hash
 * @returns Base64url-encoded CID (no padding)
 */
export async function cid(bytes: Uint8Array): Promise<string> {
  await initialize();
  
  try {
    return wasm_cid(bytes);
  } catch (error) {
    throw new Error(`CID computation failed: ${error}`);
  }
}

/**
 * Check if WASM is initialized
 */
export function isReady(): boolean {
  return isInitialized;
}
