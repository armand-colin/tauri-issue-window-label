# tauri-issue-window-label
 Bug reproduction example for tauri window communication system  

## Scenario
 The program creates 2 windows at startup: `A` then `B`, directly in rust, after the creation of the tauri app.  
 After 2 seconds, `A` creates two windows `A_child1` and `A_child2`.
 Every window refreshes the list of its accessible webview windows every 2 seconds, using `window.getAll()`, and then displays it.

## Issue
 In this scenario, not all the webviews have access to each other.
 | A                     | B                        | A_child1       | A_child2       |
 |-----------------------|--------------------------|----------------|----------------|
 | A, A_child1, A_child2 | A, B, A_child1, a_child2 | A, B, A_child1 | A, B, A_child2 |