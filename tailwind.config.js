/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html", "./src/components/*.{rs,html,css}"],
  theme: {
    extend: {},
  },
  plugins: [],
};
