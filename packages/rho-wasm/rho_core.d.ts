/* tslint:disable */
/* eslint-disable */

/**
 * Initialize the WASM module (can be called for any setup if needed)
 */
export function init(): void;

/**
 * Compute CID from raw bytes (base64url, no padding)
 */
export function wasm_cid(bytes: Uint8Array): string;

/**
 * Normalize a JSON value and return canonical form with CID
 *
 * Input: any JS value (object, array, string, number, etc.)
 * Output: { normalized: string, cid: string }
 */
export function wasm_normalize(input: any): any;

/**
 * Validate is a simplified version for now - in full implementation,
 * it would fetch schema from CAS and validate against it
 */
export function wasm_validate(_schema_cid: string, input: any): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly init: () => void;
    readonly wasm_cid: (a: number, b: number) => [number, number];
    readonly wasm_normalize: (a: any) => [number, number, number];
    readonly wasm_validate: (a: number, b: number, c: any) => [number, number, number];
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
