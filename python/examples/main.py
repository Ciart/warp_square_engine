from warp_square_engine import *

game = Game()

game.push_piece_move(PyPieceMove(PySquare(Rank.Two, File.A, Level.White), PySquare(Rank.Four, File.A, Level.White)))

game.print()
