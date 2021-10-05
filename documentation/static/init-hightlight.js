new MutationObserver((mutated) =>
  mutated.forEach((m) => {
    let t = m.target;
    if (t.tagName === "CODE" && !t.getAttribute("data-highlight")) {
      t.setAttribute("data-highlight", "true");
      hljs.highlightElement(t);
    }
  })
).observe(document, {
  subtree: true,
  childList: true,
});
