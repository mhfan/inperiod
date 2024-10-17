
1. Install npm: <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>
2. Install the tailwind css cli: <https://tailwindcss.com/docs/installation>
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Launch the Dioxus Web/Desktop app:

```bash
dx serve --platform web
dx serve --platform desktop
cd dist && cargo r --features desktop
```
