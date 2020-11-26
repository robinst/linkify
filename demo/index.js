const pkg = import("./pkg");

let rust = undefined;

pkg
  .then((m) => (rust = m))
  .catch((e) => console.error("Failed to load WebAssembly module", e));

export function linkifyText(input) {
  return rust !== undefined ? rust.linkify_text(input) : undefined;
}

function handleInput(e) {
  const text = e.target.value;
  output.innerHTML = linkifyText(text);
}

const input = document.getElementById("input");
const output = document.getElementById("output");
input.oninput = handleInput;
