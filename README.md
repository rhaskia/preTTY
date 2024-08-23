# PreTTY
PreTTY is a Dioxus-based terminal emulator, with full customizability through JS and CSS.

## Dependencies 
The main dependency is just a web view to render the app, which is usually these:

Linux: WebkitGtk,

Windows: WebView2 (packaged with Edge),

MacOS: Built in

You also need libxdo on linux and a nerd font for the icons to work.

# Installation 
The following command will work for any OS once I upload the project to crates.io:
`cargo install --locked pretty-term`
on Linux you may also need the webkitgtk package for PreTTY to function.
For now you will just have to run cargo run --release in the directory and move the executable into somewhere where the system will find it.

You can also install it on the following package managers as well:
 * Nothing yet 

# Plugins
To use plugins, either use the plugin manager (not yet containing anything), or copy a plugin you find ([or make!](https://github.com/rhaskia/PreTTYExamplePlugin)) into the config folder for PreTTY.
