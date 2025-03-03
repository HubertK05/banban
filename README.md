# Banban

[![Mentioned in Awesome Tauri][]][1]

A productivity app inspired by GitHub Projects Kanban built using [Tauri](https://tauri.app/) and [Svelte](https://svelte.dev/), with a [SQLite](https://www.sqlite.org/) database backend.

![Kanban](https://github.com/HubertK05/banban/assets/47300834/17263579-f179-498d-ad0d-6e28d4611c55)

## Tracking progress in work made simple

Banban is built with simplicity in mind. You can add, remove and modify your tasks to different columns. If you decide to change the category of a specific task, you can simply drag it. It is also possible to drag columns to change their order the same way.

Tasks can have different tags associated with them. There is a limit of one tag per category, but you may also add tags that do not belong to any category, and use them without any limits.

When you remove a column, all the tasks in it are moved to a stash, from where you can drag tasks to an actual column.

## Demo

https://github.com/HubertK05/banban/assets/47300834/a6c059a7-7b6d-4b77-8840-82bce6f3bff9

https://github.com/tauri-apps/awesome-tauri#productivity

[1]: https://github.com/tauri-apps/awesome-tauri#productivity
[Mentioned in Awesome Tauri]: https://awesome.re/mentioned-badge.svg

## Development

### Prerequisites

Read and follow [Tauri prerequisities](https://tauri.app/start/prerequisites/). Install required system dependencies, [Rust](https://www.rust-lang.org/tools/install) and [Node.js LTS](https://nodejs.org/en).

#### Install project dependencies

```bash
pnpm install
```

### Run

```
pnpm tauri dev
```