const pkg = import("./pkg");

let rustModule = undefined;

pkg
  .then((m) => {
    rustModule = m;
    // In case of a page reload, the input can contain remembered text, so linkify that.
    handleInput(input.value);
  })
  .catch((e) => console.error("Failed to load WebAssembly module", e));

function linkifyText(input) {
  return rustModule !== undefined ? rustModule.linkify_text(input) : undefined;
}

function handleInput(text) {
  const start = performance.now();
  output.innerHTML = linkifyText(text);
  const millis = performance.now() - start;
  if (text.length === 0) {
    timing.innerHTML = "";
  } else {
    const links = document.getElementsByTagName("a").length;
    timing.innerHTML =
      "✓ <strong>" +
      millis +
      " ms</strong> to linkify " +
      links +
      " links in text with " +
      text.length +
      " characters";
  }
}

const input = document.getElementById("input");
const output = document.getElementById("output");
const timing = document.getElementById("timing");
input.oninput = (e) => handleInput(e.target.value);
