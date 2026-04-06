/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      fontFamily: { mono: ['ui-monospace', 'SFMono-Regular', 'monospace'] },
    },
  },
  plugins: [],
};
