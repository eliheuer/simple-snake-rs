# Simple Snake RS
A simple snake game in Rust. This game started as the result of following [this](https://blog.scottlogic.com/2020/10/08/lets-build-snake-with-rust.html) blog post, but has been heavily modified.

## Running the Game

1. Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone this repository:
   ```bash
   git clone https://github.com/eliheuer/simple-snake-rs
   cd simple-snake-rs
   ```
3. Run the game:
   ```bash
   cargo run
   ```

### Controls
- Use WASD or arrow keys to control the snake's direction
- 'Q' or Esc to quit the game
- Ctrl+C to force quit

## How the Program Works

The game is built using Rust and implements the classic Snake game mechanics:

### Core Components

1. **Game Loop**: The main game loop runs at a variable speed (getting faster as you score more points) and handles:
   - Input processing
   - Game state updates
   - Collision detection
   - Rendering

2. **Snake**: The snake is represented as a collection of points, with:
   - A head point that leads the movement
   - A body that follows the head
   - A direction of movement
   - Growth mechanics when food is eaten

3. **Terminal UI**: The game uses the `crossterm` crate to:
   - Handle terminal manipulation
   - Process keyboard input
   - Render the game elements (snake, food, borders, score)
   - Display colored output

### Game Mechanics

- The snake moves continuously in the current direction
- Food appears randomly on the screen
- Eating food increases the score and snake length
- The game ends if the snake:
  - Hits the wall
  - Collides with itself
- Speed increases progressively as you score more points
- The snake changes color based on current speed

### Technical Features

- Raw terminal mode for immediate input processing
- Custom terminal size management
- Non-blocking input handling
- Efficient screen rendering
- Collision detection system
