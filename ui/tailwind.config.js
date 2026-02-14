/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
      colors: {
        // Rho OS Color System (The Traffic Light of Truth)
        'rho-valid': '#10b981',      // Emerald-500 - Cryptographically Valid
        'rho-invalid': '#f43f5e',    // Rose-500 - Hash Mismatch/Invalid
        'rho-processing': '#fbbf24', // Amber-400 - Processing/Unverified
        'rho-immutable': '#94a3b8',  // Slate-400 - Historical/Cold Storage
        'glass': {
          DEFAULT: 'rgba(30, 41, 59, 0.7)',
          light: 'rgba(51, 65, 85, 0.5)',
        },
      },
      backdropBlur: {
        'glass': '12px',
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glow': 'glow 2s ease-in-out infinite',
      },
      keyframes: {
        glow: {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.5' },
        },
      },
    },
  },
  plugins: [],
}
