const pkg = import("../pkg");

let rustModule = undefined;

pkg
  .then((m) => {
    rustModule = m;
    // In case of a page reload, the input can contain remembered text, so linkify that.
    handleInput(input.value, schemeOption.checked);
  })
  .catch((e) => console.error("Failed to load WebAssembly module", e));

function linkifyText(input, allowWithoutScheme) {
  return rustModule !== undefined ? rustModule.linkify_text(input, allowWithoutScheme) : undefined;
}

function handleInput(text, allowWithoutScheme) {
  const start = performance.now();
  output.innerHTML = linkifyText(text, allowWithoutScheme);
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
  handleInput(text, schemeOption.checked);
}

function setLongExample(desc, n) {
  let s = "";
  for (let i = 1; i <= n; i++) {
    s += "Example: https://example.org/" + desc + "/link-" + i + "\n";
  }
  setExample(s);
}

const input = document.getElementById("input");
const schemeOption = document.getElementById("allow-without-scheme");
const output = document.getElementById("output");
const timing = document.getElementById("timing");
input.oninput = (e) => handleInput(e.target.value, schemeOption.checked);
schemeOption.oninput = (e) => handleInput(input.value, e.target.checked);

const urlsExample =
  "Some links: https://example.org, https://example.com/a.\n" +
  "See how ',' and '.' are not included in the link?\n\n" +
  "What about parentheses (https://example.org)? What if they are part of the link?: https://en.wikipedia.org/wiki/Link_(The_Legend_of_Zelda)\n\n" +
  "Without scheme: example.com or example.com/a";
const emailExample = "abc@example.org, foo+bar@example.org;hi@example.org";

document.getElementById("example-urls").onclick = () => setExample(urlsExample);
document.getElementById("example-emails").onclick = () =>
  setExample(emailExample);
document.getElementById("example-long").onclick = () =>
  setLongExample("long", 100);
document.getElementById("example-very-long").onclick = () =>
  setLongExample("very-long", 5000);
