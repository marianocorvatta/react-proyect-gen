# react-proyect-gen

React proyect generator is a command-line tool that simplifies the process of selecting and creating React projects. With RPG, you can easily choose from a variety of React frameworks/templates and have a new project set up in no time.

## How It Works

1. Run the program in your terminal.
2. Enter your desired project name when prompted.
3. Select a project type from the provided options.
4. Confirm your choice when asked.
5. react-proyect-gen will run the necessary command to generate the selected project template.
6. After project generation, you can interact with the project's command-line tools directly.

## Features

- Easy project selection: Choose from a range of popular React project templates.
- User-friendly interface: Clearly presented options and prompts.
- Quick setup: Create new React projects without the hassle of manually setting up templates.

## Usage

1. Clone or download this repository.
2. Make sure you have Rust installed on your system.
3. Open your terminal and navigate to the project directory.
4. Run the program using the `cargo run` command.

## Recommended Steps

1. After running the program, you can generate an executable by building the project using the `cargo build --release` command.

2. Once the build is successful, the executable will be located in the `target/release/` directory within the project folder.

3. For convenience, consider moving the executable to a directory that's part of your system's PATH, such as `/usr/local/bin` or `~/bin`.

4. To make it even more convenient, you can create an alias that points directly to the executable. For example, you can add the following line to your shell's configuration file (e.g., `.bashrc`, `.zshrc`):
   ```sh
   alias react-gen='~/bin/react-proyect-gen'

## Supported Frameworks/Templates

- Next.js
- Vite.js
- T3-App
- Create-React-App
- React native bare
- React native expo

## License

This project is licensed under the [MIT License](LICENSE).

---

Feel free to contribute and improve this project! If you have any questions or suggestions, please open an issue or pull request.