// SecureDropZone Component
// Secure file drop zone that calculates hash client-side

import { useState, useCallback } from 'react';
import { useRhoCore } from '@/hooks/useRhoCore';

interface SecureDropZoneProps {
  onFileProcessed: (result: { cid: string; filename: string; size: number }) => void;
  accept?: string;
  maxSize?: number; // in bytes
}

export function SecureDropZone({ 
  onFileProcessed, 
  accept = '.json,.pdf,image/*',
  maxSize = 10 * 1024 * 1024 // 10MB default
}: SecureDropZoneProps) {
  const [isDragging, setIsDragging] = useState(false);
  const [processing, setProcessing] = useState(false);
  const [status, setStatus] = useState<string>('');
  const { hash } = useRhoCore();

  const processFile = useCallback(async (file: File) => {
    if (file.size > maxSize) {
      setStatus(`File too large: ${(file.size / 1024 / 1024).toFixed(2)}MB (max: ${(maxSize / 1024 / 1024).toFixed(2)}MB)`);
      return;
    }

    setProcessing(true);
    setStatus('Reading file...');

    try {
      const arrayBuffer = await file.arrayBuffer();
      const bytes = new Uint8Array(arrayBuffer);

      setStatus('Calculating Blake3 Hash...');
      const cid = await hash(bytes);

      setStatus('Complete');
      onFileProcessed({
        cid,
        filename: file.name,
        size: file.size,
      });
    } catch (error) {
      setStatus(`Error: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setProcessing(false);
    }
  }, [hash, maxSize, onFileProcessed]);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);

    const files = Array.from(e.dataTransfer.files);
    if (files.length > 0) {
      processFile(files[0]);
    }
  }, [processFile]);

  const handleFileSelect = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      processFile(files[0]);
    }
  }, [processFile]);

  return (
    <div
      className={`
        relative border-2 border-dashed rounded-lg p-12 text-center transition-all
        ${isDragging 
          ? 'border-rho-processing bg-rho-processing/10' 
          : 'border-slate-700 hover:border-slate-600'
        }
        ${processing ? 'pointer-events-none opacity-50' : 'cursor-pointer'}
      `}
      onDragOver={(e) => {
        e.preventDefault();
        setIsDragging(true);
      }}
      onDragLeave={() => setIsDragging(false)}
      onDrop={handleDrop}
      onClick={() => document.getElementById('file-input')?.click()}
    >
      <input
        id="file-input"
        type="file"
        className="hidden"
        accept={accept}
        onChange={handleFileSelect}
        disabled={processing}
      />

      <div className="space-y-4">
        {processing ? (
          <>
            <div className="inline-block w-12 h-12 border-4 border-slate-700 border-t-rho-processing rounded-full animate-spin" />
            <p className="font-mono text-rho-processing">{status}</p>
          </>
        ) : (
          <>
            <div className="text-6xl">📁</div>
            <h3 className="text-xl font-semibold text-slate-200">Drop Proof Here</h3>
            <p className="text-slate-400">
              or click to browse
            </p>
            <p className="text-xs text-slate-500 font-mono">
              Supported: {accept} (max {(maxSize / 1024 / 1024).toFixed(0)}MB)
            </p>
          </>
        )}
        {status && !processing && (
          <p className="text-sm text-slate-400 mt-2">{status}</p>
        )}
      </div>
    </div>
  );
}
