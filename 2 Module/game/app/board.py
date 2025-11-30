class Board:
    """
    Represents the Tic Tac Toe game board and its state.

    This class stores the 3x3 grid, the current player,
    and game outcome flags such as winner or draw. It also
    provides methods for modifying and serializing board state.
    """
    def __init__(self):
        """Initialize a new board by resetting all values."""
        self.reset()

    def reset(self):
        """
        Reset the board to its initial state.
        """
        self.board = [[None for _ in range(3)] for _ in range(3)]
        self.current_player = "x"
        self.winner = None
        self.draw = False

    def make_move(self, row, col):
        """
        Attempt to place the current player's mark on the board.

        Args:
            row (int): Row index (0–2).
            col (int): Column index (0–2).

        Returns:
            bool: True if the move was successful, False otherwise.
        """
        if self.board[row][col] is None and not self.winner:
            self.board[row][col] = self.current_player
            self.current_player = "o" if self.current_player == "x" else "x"
            return True
        return False

    def is_full(self):
        """
        Check if the board is full.

        Returns:
            bool: True if all cells are filled, False otherwise.
        """
        return all(all(row) for row in self.board)

    def to_dict(self):
        """
        Convert board state into a dictionary.

        Returns:
            dict:   A dictionary with board data, current player,
                    winner, and draw flags.

        """
        return {
            "board": self.board,
            "current_player": self.current_player,
            "winner": self.winner,
            "draw": self.draw
        }

    @staticmethod
    def from_dict(data):
        """
        Reconstruct a Board object from previously saved data.

        Args:
            data (dict): Serialized board information.

        Returns:
            Board: A new Board instance with restored values.
        """
        b = Board()
        b.board = data["board"]
        b.current_player = data["current_player"]
        b.winner = data["winner"]
        b.draw = data["draw"]
        return b