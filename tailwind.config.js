/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/main.rs"], //"./src/**/*.{rs,html,css}"
  //mode: "all",

  theme: {
    //container: { center: true, },
    extend: {
      boxShadow: { 'spread-2': "0 0 2px 2px",
        'border-1': "1px 0, 0 1px, 1px 1px, inset 1px 0, inset 0 1px",
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
    //require('tw-elements/plugin.cjs'), // npm install tw-elements -D
  ],

  //presets: [ require('@acmecorp/tailwind-base') ],
  // https://github.com/tailwindlabs/tailwindcss/blob/master/stubs/defaultConfig.stub.js
  // npm install tailwindcss -D #-g // npx tailwindcss init #--full
  // npx tailwindcss -i tailwind_base.css -o assets/tailwind.css -w #-m
}
