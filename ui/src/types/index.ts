// Rho OS Core Types

export type CryptoStatus = 'valid' | 'invalid' | 'processing' | 'immutable';

export type SignatureAlgorithm = 'blake3' | 'ed25519' | 'mldsa3';

export interface CidInfo {
  cid: string;
  algorithm: SignatureAlgorithm;
}

export interface ReciboCard {
  cid: string;
  content_cid: string;
  spec_cid: string;
  inputs: Record<string, any>;
  timestamp: string;
  signatures: Signature[];
  status: CryptoStatus;
}

export interface Signature {
  algorithm: SignatureAlgorithm;
  public_key: string;
  signature: string;
}

export interface ChipSpec {
  name: string;
  version: string;
  schema: any; // JSON Schema
  policy?: string;
  description?: string;
  icon_cid?: string;
}

export interface VerificationResult {
  valid: boolean;
  status: CryptoStatus;
  issuer?: string;
  timestamp?: string;
  chip?: string;
  errors?: string[];
  metadata?: Record<string, any>;
}

export interface FormField {
  name: string;
  type: 'string' | 'number' | 'boolean' | 'object' | 'array' | 'file';
  required?: boolean;
  description?: string;
  pattern?: string;
  enum?: string[];
  minimum?: number;
  maximum?: number;
  contentMediaType?: string;
}
