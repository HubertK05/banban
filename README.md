# Tauri + Svelte + Typescript

This template should help get you started developing with Tauri, Svelte and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


## Database

### Migrations
```bash
/banban$ sea-orm-cli migrate generate
```
```bash
/banban$ sea-orm-cli migrate
```
### Entities
```bash
/banban$ sea-orm-cli generate entity -o entity/src/entities
```