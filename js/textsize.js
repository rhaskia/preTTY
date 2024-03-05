function getTextSize(size, family) {
    console.log(size, family);

    // Create a hidden div with the letter "M"
    const hiddenDiv = document.createElement("div");
    hiddenDiv.style.cssText = "position: absolute; visibility: hidden;";
    hiddenDiv.style.fontSize = size;
    hiddenDiv.style.fontFamily = family;
    hiddenDiv.textContent = "M";
    document.body.appendChild(hiddenDiv);

    // Get the dimensions of the hidden div
    const width = hiddenDiv.offsetWidth;
    const height = hiddenDiv.offsetHeight;

    // Remove the hidden div
    document.body.removeChild(hiddenDiv);

    // Return the size as an object
    return { width, height };
}
