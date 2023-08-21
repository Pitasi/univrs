/** @type {import('tailwindcss').Config} */
const config = {
  content: [
    './src/**/*.{html,rs}',
    './articles/**/*.md',
  ],
  theme: {
    extend: {
      typography: ({ theme }) => ({
        DEFAULT: {
          css: {
            "--tw-prose-headings": theme("colors.black"),
            "--tw-prose-body": theme("colors.eerie"),
          },
        },
      }),
      colors: {
        midnight: {
          200: "#FED9FF",
          400: "#8C63FF",
          600: "#321D6A",
          700: "#1D0D42",
        },
        neutral: {
          DEFAULT: "#FFFEFE",
        },
        forest: "#144E5A",
        beige: {
          200: "#FEF2E6",
        },
        sun: {
          300: "#FEEBA6",
          400: "#FEDE6C",
        },
        wood: "#BF7D53",
        lightsky: "#C1D5DA",

        //neubrutalism
        seafoam: "#ABE5BC",
        vomit: "#BAFFA3",
        ligthsalmon: "#FFA776",
        jasmine: "#FFD787",
        // violet: "#CFADE8",
        salmon: "#FFA89A",
        // lightviolet: "#F8C3EB",
        // Accent colors for text hierarchies:
        liver: "#505050",
        eerie: "#1E1E1E",
        // Secondary white for base of the app:
        floralwhite: "#FFFAF0",

        // pimp saturation up
        lightviolet: "#fe7dd0",
        acid: "#DAFD3C",
        yellow: "#f6ff5f",
        cyan: "#71f2ff",
        violet: "#d67fff",
        darkviolet: "#e20093",
      },
      data: {
        active: 'active="true"',
      },
      boxShadow: shadows(),
      dropShadow: dropShadows(),
      backgroundImage: {
        "pattern-hideout":
          "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='40' height='40' viewBox='0 0 40 40'%3E%3Cg fill-rule='evenodd'%3E%3Cg fill='%23000000' fill-opacity='0.03'%3E%3Cpath d='M0 38.59l2.83-2.83 1.41 1.41L1.41 40H0v-1.41zM0 1.4l2.83 2.83 1.41-1.41L1.41 0H0v1.41zM38.59 40l-2.83-2.83 1.41-1.41L40 38.59V40h-1.41zM40 1.41l-2.83 2.83-1.41-1.41L38.59 0H40v1.41zM20 18.6l2.83-2.83 1.41 1.41L21.41 20l2.83 2.83-1.41 1.41L20 21.41l-2.83 2.83-1.41-1.41L18.59 20l-2.83-2.83 1.41-1.41L20 18.59z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\")",
        "pattern-plus":
          "url(\"data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%23000000' fill-opacity='0.07'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\")",
      },
      fontFamily: {
        sans: ["var(--font-inter)", "ui-sans-serif", "system-ui", "-apple-system", "BlinkMacSystemFont", "\"Segoe UI\"", "Roboto", "\"Helvetica Neue\"", "Arial", "\"Noto Sans\"", "sans-serif", "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"],
        neu: ["var(--font-clash-display)", "ui-sans-serif", "system-ui", "-apple-system", "BlinkMacSystemFont", "\"Segoe UI\"", "Roboto", "\"Helvetica Neue\"", "Arial", "\"Noto Sans\"", "sans-serif", "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"],
      },
    },
    keyframes: {
      "accordion-down": {
        from: { height: "0" },
        to: { height: "var(--radix-accordion-content-height)" },
      },
      "accordion-up": {
        from: { height: "var(--radix-accordion-content-height)" },
        to: { height: "0" },
      },
    },
    animation: {
      "accordion-down": "accordion-down 0.2s ease-out",
      "accordion-up": "accordion-up 0.2s ease-out",
    },
  },
};

function shadows() {
  const shadows = new Map();
  for (let i = 1; i < 6; i++) {
    let s = "";
    for (let j = 1; j < i + 1; j++) {
      s += `${j}px ${j}px 0px black,`;
    }
    shadows.set(`neu-${i}`, s.slice(0, -1));
  }
  return Object.fromEntries(shadows);
}

function dropShadows() {
  const shadows = new Map();
  for (let i = 1; i < 6; i++) {
    let s = `${i}px ${i}px 0px black`;
    shadows.set(`neu-${i}`, s);
  }
  return Object.fromEntries(shadows);
}

module.exports = config;
