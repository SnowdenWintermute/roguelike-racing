/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        beigepaper: "#988962",
        firered: "#ad252f",
        iceblue: "#2b9799",
        windgreen: "#2faa36",
        earthyellow: "#afa915",
        lightningpurple: "#703c91",
        waterblue: "#332e92",
        darknessblack: "#2e2514",
        lightwhite: "#a7a08d",
      },
      keyframes: {
        "slide-left-appear-keyframes": {
          "0%": { transform: "translateX(-1rem)", opacity: "0%" },
          "100%": { transform: "translateX(0rem)", opacity: "100%" },
        },
        "up-and-down-keyframes": {
          "0%": {
            transform: "translate(-50%,-.35rem)",
          },
          "100%": {
            transform: "translate(-50%, 0rem)",
          },
        },
      },
      animation: {
        "slide-appear-from-left": "slide-left-appear-keyframes .3s linear",
        "up-and-down":
          "up-and-down-keyframes 1.5s ease-in-out infinite alternate-reverse",
      },
    },
  },
  plugins: [],
};
