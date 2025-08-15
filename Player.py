"""
Author: Liam Laidlaw
Filename: player.py 
Purpose: Abstract representation of a player in a chess game
"""


class Player:

    def __init__(self, role: str, orientation: str) -> None:
        """Player constructor

        Args:
            role: Either white or black
            orientation: Either bottom or top
        """

        self.role = role
        self.is_turn = role == "white"
        
        self.num_pieces = {
            "pons": 8,
            "rooks": 2,
            "knights": 2,
            "bishops": 2, 
            "queen": 1, 
            "king": 1,
        }

        self.pieces = {
            "pons": self.pons,
            "rooks": self.rooks,
            "knights": self.knights,
            "bishops": self.bishops, 
            "queen": self.queen,
            "king": self.king,
        }

        for piece_type, bit_board in self.pieces.items():
            bit_board = self.generate_bitboard(role, piece_type)


    def generate_bitboard(self, role: str, piece_tupe: str) -> long:
        """Bitboard generator

        Generates a bitboard represented as a 64 bit integer (one bit for every square)

        Args:
            role: white or black
            piece_type: The type of piece to generate the bitboard for.

        Returns:
            A 64 bit integer which represents the location of each piece
            of the specified piece type in binary. 
        """

        



    def print_pieces(self):
        




