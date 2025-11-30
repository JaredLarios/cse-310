import pygame as pg
from app.settings import *

class Graphics:
    """
    Handles all rendering for the Tic Tac Toe game.

    This class draws the board grid, player moves, status text,
    cover screen, and restored saved-game boards.
    """
    def __init__(self, screen):
        """
        Initialize the graphics system and load all image assets.

        Args:
            screen (pygame.Surface): The surface where everything is drawn.
        """
        self.screen = screen

        # Load images
        self.cover = pg.image.load(ASSET_PATH + "modified_cover.png")
        self.x_img = pg.image.load(ASSET_PATH + "X_modified.png")
        self.o_img = pg.image.load(ASSET_PATH + "o_modified.png")

        self.cover = pg.transform.scale(self.cover, (WIDTH, HEIGHT + 100))
        self.x_img = pg.transform.scale(self.x_img, (80, 80))
        self.o_img = pg.transform.scale(self.o_img, (80, 80))

    def draw_board_lines(self):
        """
        Draw the Tic Tac Toe grid lines on the screen.
        """
        pg.draw.line(self.screen, BLACK, (WIDTH/3, 0), (WIDTH/3, HEIGHT), 5)
        pg.draw.line(self.screen, BLACK, (WIDTH/3*2, 0), (WIDTH/3*2, HEIGHT), 5)
        pg.draw.line(self.screen, BLACK, (0, HEIGHT/3), (WIDTH, HEIGHT/3), 5)
        pg.draw.line(self.screen, BLACK, (0, HEIGHT/3*2), (WIDTH, HEIGHT/3*2), 5)

    def draw_status(self, board):
        """
        Display the current game status below the board.

        Args:
            board (Board): The game board state object.
        """
        self.screen.fill(BLACK, (0, HEIGHT, WIDTH, 100))
        font = pg.font.Font(None, 36)

        if board.winner:
            msg = f"{board.winner.upper()} Wins!"
        elif board.draw:
            msg = "Draw!"
        else:
            msg = f"{board.current_player.upper()}'s Turn"

        text = font.render(msg, True, WHITE)
        rect = text.get_rect(center=(WIDTH // 2, HEIGHT + 50))
        self.screen.blit(text, rect)

    def draw_move(self, row, col, player):
        """
        Draw a single X or O on the board at a specific position.

        Args:
            row (int): Board row index (0–2).
            col (int): Board column index (0–2).
            player (str): "x" or "o".
        """
        x = col * (WIDTH/3) + 30
        y = row * (HEIGHT/3) + 30
        img = self.x_img if player == "x" else self.o_img
        self.screen.blit(img, (x, y))

    def draw_loaded_board(self, board):
        """
        Draw all previously saved moves onto the board.
        This is used when the game loads a saved match.

        Args:
            board (Board): The board containing previously stored moves.
        """
        cell_size = WIDTH // 3

        for row in range(3):
            for col in range(3):
                mark = board.board[row][col]
                if mark is None:
                    continue

                pos_x = col * cell_size + 30
                pos_y = row * cell_size + 30

                if mark == "x":
                    self.screen.blit(self.x_img, (pos_x, pos_y))
                else:
                    self.screen.blit(self.o_img, (pos_x, pos_y))
