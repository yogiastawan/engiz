/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html}"
  ],
  theme: {
    screens: {
      xs:'320px',
      sm: '480px',
      md: '768px',
      lg: '976px',
      xl: '1440px',
    },
  },
  plugins: [],
};
