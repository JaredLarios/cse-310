from app.settings import WIDTH, HEIGHT, RED
import pygame as pg

class GameLogic:
    def check_winner(self, board, screen):
        # Rows
        for r in range(3):
            if board.board[r][0] == board.board[r][1] == board.board[r][2] and board.board[r][0]:
                board.winner = board.board[r][0]
                y = r * (HEIGHT / 3) + HEIGHT / 6
                pg.draw.line(screen, RED, (0, y), (WIDTH, y), 5)
                return

        # Columns
        for c in range(3):
            if board.board[0][c] == board.board[1][c] == board.board[2][c] and board.board[0][c]:
                x = c * (WIDTH / 3) + WIDTH / 6
                board.winner = board.board[0][c]
                pg.draw.line(screen, RED, (x, 0), (x, HEIGHT), 5)
                return

        # Diagonals
        if board.board[0][0] == board.board[1][1] == board.board[2][2] and board.board[0][0]:
            board.winner = board.board[0][0]
            pg.draw.line(screen, RED, (50, 50), (350, 350), 5)
            return

        if board.board[0][2] == board.board[1][1] == board.board[2][0] and board.board[0][2]:
            board.winner = board.board[0][2]
            pg.draw.line(screen, RED, (350, 50), (50, 350), 5)
            return

        # Draw
        if board.is_full() and board.winner is None:
            board.draw = True
