/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  //mode: "all",

  theme: {
    //container: { center: true, },
    extend: {
      boxShadow: {
        'border-1': "1px 0, 0 1px, 1px 1px, inset 1px 0, inset 0 1px", 'spread-2': "0 0 2px 2px",
        'black-b': "0 2px black", 'black-l': "-2px 0 black",
        'black-bl': "-2px 0 black, 0 2px black",
      }
    },
  },

  plugins: [
    //require('@tailwindcss/forms'),
    //require('tailwindcss-children'),
    //require('@tailwindcss/typography'),
    //require('@tailwindcss/aspect-ratio'),
    //require('@tailwindcss/line-clamp'),
    //require('tw-elements/plugin.cjs'), // npm install tw-elements
  ],

  //presets: [ require('@acmecorp/tailwind-base') ],
  // https://github.com/tailwindlabs/tailwindcss/blob/master/stubs/defaultConfig.stub.js
  // npm install -D tailwindcss // npx tailwindcss init #--full
}
