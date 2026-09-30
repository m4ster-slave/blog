# Blogging site

This is my blogging site, feel free to do with it whatever you want. I probably won't update it much or will try to make it anything but my own personal site. A few other features are planned as of now. Feature- and Pull requests are always welcome :)

All of the code is human written - an code snippets pasted from AI have been ostensibly reviewed.

## The stack

I challenged myself with this project by not using standard html/css/js but try to do everything in rust, which worked pretty amazing actually.

### backend

For the backend i used Axum, which i was already quite familiar with, all the routes for this project where pretty simple so I didn't really struggle at all using that.

### frontend

What did have a quite a steep learning curve was the framework i used for the frontend, which was leptos-csr.

I compile the project to WASM and serve it with Trunk. I found that way of programming incredibly satisfying sometimes but also incredibly frustrating at other times. I feel as if the web really doesn't care about memory and type safety (as u can see with Javascript etc.), so I had to fight the compiler to get simple form submissions working most of the time... but after countless tries and some time spent reading the docs, it does make sense to build a project in Leptos - Maybe not a blogging website since its basically just sending API calls back and forth, but i could imagine building a fast and memory safe interactive frontend application again using this tool :>.

## Usage

Since this website is only for me I wont go into details on how to run everything and set everything up but creating a `.env` file out of the `.env.template` file and then running `$ docker compose up -d` is a good place to start :p

## LICENSE

GPL - do what u want with it, keep it open tho thx~
