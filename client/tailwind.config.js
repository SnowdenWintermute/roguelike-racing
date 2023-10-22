/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      keyframes: {
        "slide-left-appear-keyframes": {
          "0%": { transform: "translateX(-1rem)", opacity: "0%" },
          "100%": { transform: "translateX(0rem)", opacity: "100%" },
        },
      },
      animation: {
        "slide-appear-from-left": "slide-left-appear-keyframes .3s linear",
      },
    },
  },
  plugins: [],
};
