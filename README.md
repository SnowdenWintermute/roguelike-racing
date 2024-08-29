Superceded by [speed dungeon](https://github.com/SnowdenWintermute/speed-dungeon)

# Roguelike Racing

A multiplayer turn based game in the spirit of For the King, Diablo and Final Fantasy.
Live at [https://roguelikeracing.com](https://roguelikeracing.com)

UI written with the Yew frontend framework, backend uses websockets via Actix.

To compile tailwind:
From the client directory run `npx tailwindcss -i ./input.css -o ./style/output.css --watch`

To run the server:
From the server directory run `cargo-watch -x run`

To run the frontend:
From the client directory run `TRUNK_PROD=false trunk serve`
