module.exports = {
  purge: [
    "./**/*.svelte", // Look for .svelte files
    "./**/*.html", // Look for .html files
  ],
  theme: {
    extend: {
      colors: {
        foreground: "#001858",
        background: "#fef6e4",
        primary: "#f582ae",
        secondary: "#8bd3dd",
      },
    },
  },
  variants: {},
  plugins: [],
};
