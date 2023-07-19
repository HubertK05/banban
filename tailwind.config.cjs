/** @type {import('tailwindcss').Config}*/
const config = {
  darkMode: "class",
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    require('path').join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')
  ],

  theme: {
    extend: {},
  },

  plugins: [
    ...require('@skeletonlabs/skeleton/tailwind/skeleton.cjs')() // should be on the end of list
  ],
};

module.exports = config;
