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

  // Safe syntax highlighting without dangerouslySetInnerHTML
  // Split by lines and apply styling per line
  const lines = jsonString.split('\n');

  return (
    <div 
      className="glass-panel p-4 overflow-auto" 
      style={{ maxHeight }}
    >
      <pre className="font-mono text-sm text-slate-200">
        {lines.map((line, idx) => {
          // Simple safe coloring based on content patterns
          if (line.includes('"') && line.includes(':')) {
            // Property name
            const parts = line.split(':');
            return (
              <div key={idx}>
                <span className="text-blue-400">{parts[0]}</span>
                {parts.length > 1 && (
                  <>
                    <span>:</span>
                    <span className={
                      parts[1].includes('true') || parts[1].includes('false') 
                        ? 'text-yellow-400'
                        : parts[1].includes('null')
                        ? 'text-red-400'
                        : /\d+/.test(parts[1])
                        ? 'text-purple-400'
                        : 'text-green-400'
                    }>{parts.slice(1).join(':')}</span>
                  </>
                )}
              </div>
            );
          }
          
          return <div key={idx}>{line || ' '}</div>;
        })}
      </pre>
    </div>
  );
}
