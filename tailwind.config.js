/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  mode: 'jit',
  theme: {
    extend: {
    },
  },
  plugins: [
    require("@catppuccin/tailwindcss")({
      defaultFlavour: "mocha",
    })
  ],
}

