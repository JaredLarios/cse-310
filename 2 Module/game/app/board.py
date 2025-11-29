class Board:
    def __init__(self):
        self.reset()

    def reset(self):
        self.board = [[None for _ in range(3)] for _ in range(3)]
        self.current_player = "x"
        self.winner = None
        self.draw = False

    def make_move(self, row, col):
        if self.board[row][col] is None and not self.winner:
            self.board[row][col] = self.current_player
            self.current_player = "o" if self.current_player == "x" else "x"
            return True
        return False

    def is_full(self):
        return all(all(row) for row in self.board)