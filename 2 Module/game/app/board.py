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

    def to_dict(self):
        return {
            "board": self.board,
            "current_player": self.current_player,
            "winner": self.winner,
            "draw": self.draw
        }

    @staticmethod
    def from_dict(data):
        b = Board()
        b.board = data["board"]
        b.current_player = data["current_player"]
        b.winner = data["winner"]
        b.draw = data["draw"]
        return b