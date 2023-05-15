## How To Play:
### Picking First Player:
    Choose whether first player is X or O by typing X or O. 
    Any non X/O or blank input will randomly choose a starting player.
### Placing Mark On Board:
    Simply type the x and y position of the spot on the board separated by a comma. 
        Ex:
            Insert at top left:     0, 0
            Insert at bottom right: 2, 2
    Any weird inputs will be ignored and you will need to re-enter the position you wish to mark.
    Input is split by the comma and both resulting strings are parsed and matched to a number, inserting additional characters will not work!
### Win:
    If you somehow haven't played Tic-Tac-Toe before, the objective is to place your mark so they form a line of 3 cells, vertically, horizontally or diagonally.
### Not Win:
    If all cells are filled and no one has formed a continuous line of 3, both players lose.

## Features: 
    - Working win state.
    - Game ends in draw when board is full.

## Dependencies:
    - colored v2.0.0
        - Usage: 
            - To highlight last placed X/O on board as well as making console output easier to read.
    - rand v0.8.5
        - Usage:
            - To generate randomness.
            - Randomly selecting first player.

## Planned Features:
    - AI
    - UI improvements
