# Learning Rust: Tic-Tac-Toe

Learning rust by creating a basic version of tic-tac-toe


# Creating the Environment
1. The environment for Tic-Tac-Toe is a two dimensional board with 3 X 3 rows and columns. The environment we want is a board that can work with any amount of rows and columns. The rows and columns MUST be equal to form a square board. The board should have a single number value mapped to a row/column position. Use a dictionary/hash map to map a single number to a row column tuple.

2. For the environment, create a function that saves states of the Tic-Tac-Toe board at certain intervals. Save the state of the board to a json file. The json file should store keys at certain time steps in the game.


# Game Logic
1. The player wins if they get three in a row either vertically, horizontally, diagonally(left or right). The game should check for all 4 of these possibilties. If any 4 of these posssibilties are true for X or O, the player should win.

2. For checking diagonally, it should check if the board has all (X's or O's) from left to right and vice versa.

3. Create a function for checking if a there contiguous X's and O's both vertically or horizontally.

4. The game should return an end state, that indicates whether the player won or lost. Use 1 to represent a win, use 0, to represent a loss.


# Playing the game
1. There are function needed to take actions and play the game. The agent, or player should be able to place a peice and understand the reward distribution of their action.

2. Create a function that lets the player choose where to place their peice. The player should be able to choose a number from the keys in the hash map(dictionary), and move to the desired row/column position.

3. After each action is taken, a reward of the action should be given. The timelime of actions and rewards

