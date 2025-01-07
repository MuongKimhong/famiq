# Contributing to Famiq

Thank you for considering contributing to Famiq! Whether it's fixing a bug, adding a new feature, or improving the documentation, your contribution is valuable.

- If you have any questions about this library, you can ask in [Q&A section](https://github.com/MuongKimhong/famiq/discussions/categories/q-a).
- If you have any suggestions or ideas, please leave it in [Ideas section](https://github.com/MuongKimhong/famiq/discussions/categories/ideas).
- Chat about anything and everything in [General section](https://github.com/MuongKimhong/famiq/discussions/categories/general).

If you want make any changes to source code, you can do so by:

1. Fork the repository on GitHub.
2. Clone your fork:

  ```bash
  git clone https://github.com/your-username/famiq.git
   ```

3. You can use the example app `simple_signup` or create a new project to test `Famiq` UI.

  ```bash
  cd examples/simple_signup
  cargo run
   ```
   - or create a new project inside examples dir
  ```bash
  cd examples/
  cargo new my_project
  cd my_project
  ```
   then install `Famiq` by adding this line to `Cargo.toml`

  ```toml
  [dependencies]
  famiq = { git = "https://github.com/muongkimhong/famiq", tag = "v0.2.1" }
  ```