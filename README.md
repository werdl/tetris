# Tetris Game

This is a simple terminal-based Tetris game implemented in Rust. The game features classic Tetris mechanics, including shape rotation, line clearing, and scoring.

## Project Structure

The project is organized into the following files:

- `src/main.rs`: Entry point of the application. Initializes the terminal, parses command-line options, and runs the game loop.
- `src/grid.rs`: Contains the `Grid` struct that manages the game state, including rows, active shapes, score, and level.
- `src/shape.rs`: Defines the `Shape` enum representing the different Tetris shapes (I, O, T, S, Z, J, L) and includes a method to generate a random shape.
- `src/block.rs`: Defines the `Block` enum representing the state of each cell in the grid (Full, Empty, Active) and includes the `Row` struct.
- `src/ui.rs`: Handles user interface rendering, exporting the `draw` function to render the current game state.
- `src/utils.rs`: Contains utility functions for processing user input and handling game over scenarios.

## Getting Started

To build and run the game, follow these steps:

1. Ensure you have Rust and Cargo installed on your machine. You can install them from [rust-lang.org](https://www.rust-lang.org/).

2. Clone the repository:

   ```
   git clone <repository-url>
   cd tetris
   ```

3. Build the project:

   ```
   cargo build
   ```

4. Run the game:

   ```
   cargo run
   ```

## Controls

- **Left Arrow**: Move left
- **Right Arrow**: Move right
- **Up Arrow**: Rotate
- **Down Arrow**: Move down
- **Q**: Quit

## License

This project is licensed under the MIT License. See the LICENSE file for more details.