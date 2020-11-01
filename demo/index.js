const rust = import("./pkg");

let module = undefined;

rust
  .then((m) => (module = m))
  .catch((e) => console.error("Failed to load WebAssembly module", e));

export function linkifyText(input) {
  return module !== undefined ? module.linkify_text(input) : undefined;
}

function handleInput(e) {
  const text = e.target.value;
  output.innerHTML = linkifyText(text);
}

const input = document.getElementById("input");
const output = document.getElementById("output");
input.oninput = handleInput;
