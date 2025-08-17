"""
Author: Liam Laidlaw
Filename: board.py 
Purpose: a visual representation of a chess board. 
Acts as the primary interaction object between the user and the backend.
"""

import array
import math
import cairo
import pygame
import rsvg

class BoardView:
    def __init__(self, board_path: str, pieces_path: str, width: int, height: int):
        self.visible = False
        self.width = width
        self.height = height
        self.board_path = board_path
        self.pieces_path = pieces_path
        
        


    def show(self) -> None:
        self.visible = True


    def hide(self) -> None:
        self.visible = False


    def set_size(self, width: int, height: int) -> None:
        self.width = width
        self.height = height


    def set_board(self, path: str) -> None:
        self.board_path = path
        
