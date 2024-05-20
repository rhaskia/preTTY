function getTextSize(size, family) {
    // Create a hidden div with the letter "M"
    const hiddenDiv = document.createElement("pre");
    hiddenDiv.style.cssText = "position: absolute; visibility: hidden;";
    hiddenDiv.style.fontSize = size;
    hiddenDiv.style.fontFamily = family;
    hiddenDiv.style.maxHeight = "999px";
    hiddenDiv.style.maxWidth = "999px";
    hiddenDiv.style.margin = 0;
    hiddenDiv.className = "cellspan";
    hiddenDiv.textContent = "M";
    document.body.appendChild(hiddenDiv);

    // Get the dimensions of the hidden div
    const bounds = hiddenDiv.getBoundingClientRect()
    const width = bounds.width;
    const height = bounds.height;
    console.log(bounds);

    // Remove the hidden div
    document.body.removeChild(hiddenDiv);

    // Return the size as an object
    return { width, height };
}

let size = await dioxus.recv();
let width = getTextSize(size, "JetBrainsMono Nerd Font");
dioxus.send(width);
