import json
from pathlib import Path
from app.board import Board

SAVE_PATH = Path(__file__).resolve().parent.parent / "saved" / "match.json"
SAVE_PATH.parent.mkdir(exist_ok=True)

def save_board(board: Board):
    """
    Save the current board state to a JSON file.

    Args:
        board (Board): The board instance containing the game state.
    """
    data = board.to_dict()

    with open(SAVE_PATH, "w") as f:
        json.dump(data, f)

def load_board():
    """
    Load a previously saved board state from disk.

    Returns:
        Board | None: A Board instance if loading succeeds, otherwise None.
    """
    if not SAVE_PATH.exists():
        return None

    try:
        with open(SAVE_PATH, "r") as f:
            content = f.read().strip()

            # empty file or whitespace only
            if not content:
                return None

            data = json.loads(content)

        return Board.from_dict(data)

    except (json.JSONDecodeError, ValueError, KeyError):
        # corrupted or incomplete save → ignore it
        return None

def clear_save():
    """
    Delete the saved match file if it exists.
    """
    if SAVE_PATH.exists():
        SAVE_PATH.unlink()