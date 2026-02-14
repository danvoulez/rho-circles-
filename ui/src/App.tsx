// Main App Component
// Routing and layout for the Rho OS UI

import { WasmGate } from '@/components/WasmGate';
import { HeroValidator } from '@/components/HeroValidator';

function App() {
  // For now, we'll start with just the public verifier
  // Future: Add routing for client and admin interfaces
  
  return (
    <WasmGate>
      <div className="min-h-screen bg-slate-900">
        <HeroValidator />
      </div>
    </WasmGate>
  );
}

export default App;
