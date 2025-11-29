import pygame as pg
import sys
from app.settings import *
from app.board import Board
from app.graphics import Graphics
from app.rules import GameLogic

def main():
    pg.init()
    screen = pg.display.set_mode((WIDTH, HEIGHT + 100))
    pg.display.set_caption("Modular Tic Tac Toe")

    clock = pg.time.Clock()

    board = Board()
    gfx = Graphics(screen)
    logic = GameLogic()

    # Show cover
    screen.blit(gfx.cover, (0, 0))
    pg.display.update()
    pg.time.wait(1500)

    screen.fill(WHITE)
    gfx.draw_board_lines()
    gfx.draw_status(board)

    running = True
    while running:
        for event in pg.event.get():
            if event.type == pg.QUIT:
                running = False
                pg.quit()
                sys.exit()

            if event.type == pg.MOUSEBUTTONDOWN and not board.winner and not board.draw:
                x, y = pg.mouse.get_pos()
                if y < HEIGHT:
                    row = int(y // (HEIGHT / 3))
                    col = int(x // (WIDTH / 3))

                    if board.make_move(row, col):
                        gfx.draw_move(row, col, board.board[row][col])
                        logic.check_winner(board, screen)

        gfx.draw_status(board)
        pg.display.update()
        clock.tick(FPS)

        # Reset after finish
        if board.winner or board.draw:
            pg.time.wait(2000)
            board.reset()
            screen.fill(WHITE)
            gfx.draw_board_lines()

    pg.quit()


if __name__ == "__main__":
    main()
