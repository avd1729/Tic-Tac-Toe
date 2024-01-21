def main():

    board = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']]
    player = 'X'
    gameOver = False

    while (not gameOver):

        printBoard(board)
        print("Player ", player, " enter: ")

        val = input()
        val = val.split()

        row = int(val[0])
        col = int(val[1])

        if board[row][col] == ' ':
            board[row][col] = player
            gameOver = haveWon(board, player)
            if gameOver:
                print("Player "+player+" has won! ")
            else:
                if player == 'X':
                    player = 'O'
                else:
                    player = 'X'
        else:
            print("Invalid move. Try again! ")

    printBoard(board)


def printBoard(board):

    for i in range(3):
        for j in range(3):
            print(board[i][j], '|', end=' ')
        print()


def haveWon(board, player):

    # rows
    for i in range(3):
        if board[i][0] == player and board[i][1] == player and board[i][2] == player:
            return True

    # columns
    for i in range(3):
        if board[0][i] == player and board[1][i] == player and board[2][i] == player:
            return True

    # diagonals
    if board[0][0] == player and board[1][1] == player and board[2][2] == player:
        return True

    if board[0][2] == player and board[1][1] == player and board[2][0] == player:
        return True

    return False


if __name__ == "__main__":

    main()
