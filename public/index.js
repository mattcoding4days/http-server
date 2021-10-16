const createElement = (anchortext, anchorlink) => {
  let atag = document.createElement("a");
  atag.href = anchorlink;
  atag.innerHTML = anchortext;
  return atag;
};

let parent = document.getElementById("para");
const customElement = createElement("Javascript test", "hello.html");
parent.appendChild(customElement);
