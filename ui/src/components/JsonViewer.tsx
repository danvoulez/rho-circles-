// JsonViewer Component - Chapter 6.3
// Displays JSON with syntax highlighting in human or canonical mode

interface JsonViewerProps {
  data: any;
  mode?: 'human' | 'canonical';
  maxHeight?: string;
}

export function JsonViewer({ data, mode = 'human', maxHeight = '400px' }: JsonViewerProps) {
  const jsonString = mode === 'canonical'
    ? JSON.stringify(data, Object.keys(data).sort())
    : JSON.stringify(data, null, 2);

  // Simple syntax highlighting
  const highlighted = jsonString
    .replace(/"([^"]+)":/g, '<span class="text-blue-400">"$1"</span>:')
    .replace(/: "([^"]+)"/g, ': <span class="text-green-400">"$1"</span>')
    .replace(/: (\d+)/g, ': <span class="text-purple-400">$1</span>')
    .replace(/: (true|false)/g, ': <span class="text-yellow-400">$1</span>')
    .replace(/: null/g, ': <span class="text-red-400">null</span>');

  return (
    <div 
      className="glass-panel p-4 overflow-auto" 
      style={{ maxHeight }}
    >
      <pre className="font-mono text-sm text-slate-200">
        <code dangerouslySetInnerHTML={{ __html: highlighted }} />
      </pre>
    </div>
  );
}
