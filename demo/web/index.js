const pkg = import("../pkg");

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
  const millis = Math.ceil(performance.now() - start);
  if (text.length === 0) {
    timing.innerHTML = "";
  } else {
    const links = output.getElementsByTagName("a").length;
    timing.innerHTML =
      "✓ <strong>" +
      millis.toLocaleString('en-US') +
      " ms</strong> to linkify " +
      links +
      " links in text with " +
      text.length +
      " characters";
  }
}

function setExample(text) {
  input.value = text;
  handleInput(text);
}

function setLongExample(desc, n) {
  let s = "";
  for (let i = 1; i <= n; i++) {
    s += "Example: https://example.org/" + desc + "/link-" + i + "\n";
  }
  setExample(s);
}

const input = document.getElementById("input");
const output = document.getElementById("output");
const timing = document.getElementById("timing");
input.oninput = (e) => handleInput(e.target.value);

const urlsExample =
  "- Some links: https://example.org, https://example.com/a/b/c. See how ',' and '.' are not included in the link?\n" +
  "- What about parentheses?: (https://example.org). What if they are part of the link?: https://en.wikipedia.org/wiki/Link_(The_Legend_of_Zelda)\n" +
  "- Unicode: https://en.wikipedia.org/wiki/\uD83D\uDE0A";
const emailExample = "abc@example.org, foo+bar@example.org;hi@example.org";

document.getElementById("example-urls").onclick = () => setExample(urlsExample);
document.getElementById("example-emails").onclick = () =>
  setExample(emailExample);
document.getElementById("example-long").onclick = () =>
  setLongExample("long", 100);
document.getElementById("example-very-long").onclick = () =>
  setLongExample("very-long", 5000);
