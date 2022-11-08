import { invoke, window } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";

let h1;
if (h1 = document.querySelector('h1'))
  h1.innerText = appWindow.label

setInterval(() => {
  const listItems = window.getAll()
    .map(window => `<li>${window.label}</li>`)
    .join('\n')

  const list = document.querySelector('ul')

  if (list)
    list.innerHTML = listItems
}, 2000)

if (appWindow.label === "A") {
  setTimeout(() => {
    invoke("command_create_window", { label: "A_child1", x: -150, y: 150 })
    invoke("command_create_window", { label: "A_child2", x: 150, y: 150 })
  }, 3000)
}