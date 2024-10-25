/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  mode: "all",

  theme: {
    //container: { center: true, }
    extend: {
      gridColumn: { 'span-16': 'span 16 / span 16', },
      gridTemplateColumns: { '18': 'repeat(18, 1fr)', },
      boxShadow: {
        'border-1': "1px 0, 0 1px, 1px 1px, inset 1px 0, inset 0 1px",
        'border-2': "2px 0, 0 2px, 2px 2px, inset 2px 0, inset 0 2px",
      }
    },
  },

  plugins: [
    //require('@tailwindcss/forms'),
    //require('tailwindcss-children'),
    //require('@tailwindcss/typography'),
    //require('@tailwindcss/aspect-ratio'),
    //require('@tailwindcss/line-clamp'),
    //require('tw-elements/dist/plugin'),
  ],

  //presets: [ require('@acmecorp/tailwind-base') ],
  // https://github.com/tailwindlabs/tailwindcss/blob/master/stubs/defaultConfig.stub.js
  // npm install -D tailwindcss // npx tailwindcss init #--full
};
