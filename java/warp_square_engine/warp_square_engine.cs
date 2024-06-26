
// Generated by flapigen. Do not edit.

// This warning occurs, because both Rust and C# have mehtod `ToString()`.
#pragma warning disable CS0114

using System;
using System.Runtime.InteropServices;

namespace warp_square_engine
{
    internal static class RustString {
        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void c_string_delete(IntPtr c_char_ptr);

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* *mut RustString */ IntPtr c_str_u16_to_string(/* *const u16 */ IntPtr c_string_ptr);

        internal static string rust_to_dotnet(/* *const u16 */ IntPtr c_string_ptr)
        {
            var dotnet_str = Marshal.PtrToStringUni(c_string_ptr);
            RustString.c_string_delete(c_string_ptr);
            return dotnet_str;
        }

        internal static /* *mut RustString */ IntPtr dotnet_to_rust(string dotnet_str)
        {
            var c_string_ptr = Marshal.StringToHGlobalUni(dotnet_str);
            var rust_string_ptr = c_str_u16_to_string(c_string_ptr);
            Marshal.FreeHGlobal(c_string_ptr);
            return rust_string_ptr;
        }
    }

    [System.Serializable]
    public class Error : System.Exception
    {
        public Error(string message) : base(message) { }
    }

    
    public enum PieceType {
        Pawn = 0,Knight = 1,Bishop = 2,Rook = 3,Queen = 4,King = 5
    }
    
    public enum Rank {
        Zero = 0,One = 1,Two = 2,Three = 3,Four = 4,Five = 5,Six = 6,Seven = 7,Eight = 8,Nine = 9
    }
    
    public enum File {
        Z = 0,A = 1,B = 2,C = 3,D = 4,E = 5
    }
    
    public enum Level {
        White = 0,Neutral = 1,Black = 2,Ql1 = 3,Ql2 = 4,Ql3 = 5,Ql4 = 6,Ql5 = 7,Ql6 = 8,Kl1 = 9,Kl2 = 10,Kl3 = 11,Kl4 = 12,Kl5 = 13,Kl6 = 14
    }
    
    public enum Color {
        White = 0,Black = 1
    }
    
    public class BitBoard: IDisposable {
        internal IntPtr nativePtr;

        internal BitBoard(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                BitBoard_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void BitBoard_delete(IntPtr __this);

        ~BitBoard() {
            DoDispose();
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint BitBoard_getRank(/* BitBoard */ IntPtr __this);

        
        public  Rank GetRank() {
            var __this_0 = this.nativePtr;

            var __ret_0 = BitBoard_getRank(__this_0);
            var __ret_1 = (Rank)__ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint BitBoard_getFile(/* BitBoard */ IntPtr __this);

        
        public  File GetFile() {
            var __this_0 = this.nativePtr;

            var __ret_0 = BitBoard_getFile(__this_0);
            var __ret_1 = (File)__ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint BitBoard_getLevel(/* BitBoard */ IntPtr __this);

        
        public  Level GetLevel() {
            var __this_0 = this.nativePtr;

            var __ret_0 = BitBoard_getLevel(__this_0);
            var __ret_1 = (Level)__ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* BitBoard */ IntPtr BitBoard_fromSquare(/* Square */ IntPtr square);

        
        public static BitBoard FromSquare(/* ref */ Square square_0) {
            var square_1 = square_0.nativePtr;
            var __ret_0 = BitBoard_fromSquare(square_1);
            var __ret_1 = new BitBoard(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Square */ IntPtr BitBoard_intoSquare(/* BitBoard */ IntPtr __this);

        
        public  Square IntoSquare() {
            var __this_0 = this.nativePtr;

            var __ret_0 = BitBoard_intoSquare(__this_0);
            var __ret_1 = new Square(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* BitBoard */ IntPtr BitBoard_fromHex(/* RustString */ IntPtr hex);

        
        public static BitBoard FromHex(string hex_0) {
            var hex_1 = RustString.dotnet_to_rust(hex_0);
            var __ret_0 = BitBoard_fromHex(hex_1);
            var __ret_1 = new BitBoard(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr BitBoard_toHex(/* BitBoard */ IntPtr __this);

        
        public  string ToHex() {
            var __this_0 = this.nativePtr;

            var __ret_0 = BitBoard_toHex(__this_0);
            var __ret_1 = RustString.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* BitBoard */ IntPtr BitBoard_removeLevel(/* BitBoard */ IntPtr __this);

        
        public  BitBoard RemoveLevel() {
            var __this_0 = this.nativePtr;

            var __ret_0 = BitBoard_removeLevel(__this_0);
            var __ret_1 = new BitBoard(__ret_0);
            return __ret_1;
        }
} // class

    
    public class Piece: IDisposable {
        internal IntPtr nativePtr;

        internal Piece(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                Piece_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void Piece_delete(IntPtr __this);

        ~Piece() {
            DoDispose();
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint Piece_getPieceType(/* Piece */ IntPtr __this);

        
        public  PieceType GetPieceType() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Piece_getPieceType(__this_0);
            var __ret_1 = (PieceType)__ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint Piece_getColor(/* Piece */ IntPtr __this);

        
        public  Color GetColor() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Piece_getColor(__this_0);
            var __ret_1 = (Color)__ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* BitBoard */ IntPtr Piece_getPosition(/* Piece */ IntPtr __this);

        
        public  BitBoard GetPosition() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Piece_getPosition(__this_0);
            var __ret_1 = new BitBoard(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Square */ IntPtr Piece_getSquare(/* Piece */ IntPtr __this);

        
        public  Square GetSquare() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Piece_getSquare(__this_0);
            var __ret_1 = new Square(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr Piece_getChar(/* Piece */ IntPtr __this);

        
        public  string GetChar() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Piece_getChar(__this_0);
            var __ret_1 = RustString.rust_to_dotnet(__ret_0);
            return __ret_1;
        }
} // class

    
    public class PieceMove: IDisposable {
        internal IntPtr nativePtr;

        internal PieceMove(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                PieceMove_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void PieceMove_delete(IntPtr __this);

        ~PieceMove() {
            DoDispose();
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* PieceMove */ IntPtr PieceMove_new_move(/* Square */ IntPtr source, /* Square */ IntPtr destination, /* Option */ IntPtr promotion);

        
        public  PieceMove (/* ref */ Square source_0, /* ref */ Square destination_0, Option<PieceType> promotion_0) {
            var source_1 = source_0.nativePtr;
            var destination_1 = destination_0.nativePtr;
            var promotion_1 = RustOptionPieceType.dotnet_to_rust(promotion_0);
            this.nativePtr = PieceMove_new_move(source_1, destination_1, promotion_1);
            
            
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Square */ IntPtr PieceMove_getSource(/* PieceMove */ IntPtr __this);

        
        public  Square GetSource() {
            var __this_0 = this.nativePtr;

            var __ret_0 = PieceMove_getSource(__this_0);
            var __ret_1 = new Square(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Square */ IntPtr PieceMove_getDestination(/* PieceMove */ IntPtr __this);

        
        public  Square GetDestination() {
            var __this_0 = this.nativePtr;

            var __ret_0 = PieceMove_getDestination(__this_0);
            var __ret_1 = new Square(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr PieceMove_getPromotion(/* PieceMove */ IntPtr __this);

        
        public  Option<PieceType> GetPromotion() {
            var __this_0 = this.nativePtr;

            var __ret_0 = PieceMove_getPromotion(__this_0);
            var __ret_1 = RustOptionPieceType.rust_to_dotnet(__ret_0);
            return __ret_1;
        }
} // class

    
    public class BoardMove: IDisposable {
        internal IntPtr nativePtr;

        internal BoardMove(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                BoardMove_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void BoardMove_delete(IntPtr __this);

        ~BoardMove() {
            DoDispose();
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* BoardMove */ IntPtr BoardMove_new(uint source, uint destination, /* Option */ IntPtr promotion);

        
        public  BoardMove (Level source_0, Level destination_0, Option<PieceType> promotion_0) {
            var source_1 = (uint)source_0;
            var destination_1 = (uint)destination_0;
            var promotion_1 = RustOptionPieceType.dotnet_to_rust(promotion_0);
            this.nativePtr = BoardMove_new(source_1, destination_1, promotion_1);
            
            
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint BoardMove_getSource(/* BoardMove */ IntPtr __this);

        
        public  Level GetSource() {
            var __this_0 = this.nativePtr;

            var __ret_0 = BoardMove_getSource(__this_0);
            var __ret_1 = (Level)__ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint BoardMove_getDestination(/* BoardMove */ IntPtr __this);

        
        public  Level GetDestination() {
            var __this_0 = this.nativePtr;

            var __ret_0 = BoardMove_getDestination(__this_0);
            var __ret_1 = (Level)__ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr BoardMove_getPromotion(/* BoardMove */ IntPtr __this);

        
        public  Option<PieceType> GetPromotion() {
            var __this_0 = this.nativePtr;

            var __ret_0 = BoardMove_getPromotion(__this_0);
            var __ret_1 = RustOptionPieceType.rust_to_dotnet(__ret_0);
            return __ret_1;
        }
} // class

    
    public class Square: IDisposable {
        internal IntPtr nativePtr;

        internal Square(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                Square_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void Square_delete(IntPtr __this);

        ~Square() {
            DoDispose();
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Square */ IntPtr Square_new(uint rank, uint file, uint level);

        
        public  Square (Rank rank_0, File file_0, Level level_0) {
            var rank_1 = (uint)rank_0;
            var file_1 = (uint)file_0;
            var level_1 = (uint)level_0;
            this.nativePtr = Square_new(rank_1, file_1, level_1);
            
            
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint Square_getRank(/* Square */ IntPtr __this);

        
        public  Rank GetRank() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Square_getRank(__this_0);
            var __ret_1 = (Rank)__ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint Square_getFile(/* Square */ IntPtr __this);

        
        public  File GetFile() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Square_getFile(__this_0);
            var __ret_1 = (File)__ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint Square_getLevel(/* Square */ IntPtr __this);

        
        public  Level GetLevel() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Square_getLevel(__this_0);
            var __ret_1 = (Level)__ret_0;
            return __ret_1;
        }
} // class

    
    public class Game: IDisposable {
        internal IntPtr nativePtr;

        internal Game(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                Game_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void Game_delete(IntPtr __this);

        ~Game() {
            DoDispose();
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Game */ IntPtr Game_new();

        
        public  Game () {
            
            this.nativePtr = Game_new();
            
            
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr Game_getAttackSquares(/* Game */ IntPtr __this, /* Square */ IntPtr square);

        
        public  System.Collections.Generic.List<Square> GetAttackSquares(/* ref */ Square square_0) {
            var __this_0 = this.nativePtr;
var square_1 = square_0.nativePtr;
            var __ret_0 = Game_getAttackSquares(__this_0, square_1);
            var __ret_1 = RustVecSquare.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte Game_legalPieceMove(/* Game */ IntPtr __this, /* PieceMove */ IntPtr pieceMove);

        
        public  bool LegalPieceMove(/* ref */ PieceMove pieceMove_0) {
            var __this_0 = this.nativePtr;
var pieceMove_1 = pieceMove_0.nativePtr;
            var __ret_0 = Game_legalPieceMove(__this_0, pieceMove_1);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte Game_legalBoardMove(/* Game */ IntPtr __this, /* BoardMove */ IntPtr boardMove);

        
        public  bool LegalBoardMove(/* ref */ BoardMove boardMove_0) {
            var __this_0 = this.nativePtr;
var boardMove_1 = boardMove_0.nativePtr;
            var __ret_0 = Game_legalBoardMove(__this_0, boardMove_1);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void Game_pushPieceMove(/* Game */ IntPtr __this, /* PieceMove */ IntPtr pieceMove);

        
        public  void PushPieceMove(/* ref */ PieceMove pieceMove_0) {
            var __this_0 = this.nativePtr;
var pieceMove_1 = pieceMove_0.nativePtr;
            Game_pushPieceMove(__this_0, pieceMove_1);
            
            
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void Game_pushBoardMove(/* Game */ IntPtr __this, /* BoardMove */ IntPtr boardMove);

        
        public  void PushBoardMove(/* ref */ BoardMove boardMove_0) {
            var __this_0 = this.nativePtr;
var boardMove_1 = boardMove_0.nativePtr;
            Game_pushBoardMove(__this_0, boardMove_1);
            
            
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void Game_popMove(/* Game */ IntPtr __this);

        
        public  void PopMove() {
            var __this_0 = this.nativePtr;

            Game_popMove(__this_0);
            
            
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void Game_print(/* Game */ IntPtr __this);

        
        public  void Print() {
            var __this_0 = this.nativePtr;

            Game_print(__this_0);
            
            
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint Game_getTurn(/* Game */ IntPtr __this);

        
        public  Color GetTurn() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Game_getTurn(__this_0);
            var __ret_1 = (Color)__ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint Game_getFullMoveNumber(/* Game */ IntPtr __this);

        
        public  uint GetFullMoveNumber() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Game_getFullMoveNumber(__this_0);
            var __ret_1 = __ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint Game_getHalfMoveClock(/* Game */ IntPtr __this);

        
        public  uint GetHalfMoveClock() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Game_getHalfMoveClock(__this_0);
            var __ret_1 = __ret_0;
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr Game_getPieces(/* Game */ IntPtr __this);

        
        public  System.Collections.Generic.List<Piece> GetPieces() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Game_getPieces(__this_0);
            var __ret_1 = RustVecPiece.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr Game_getCapturedPieces(/* Game */ IntPtr __this);

        
        public  System.Collections.Generic.List<Piece> GetCapturedPieces() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Game_getCapturedPieces(__this_0);
            var __ret_1 = RustVecPiece.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte Game_isCheck(/* Game */ IntPtr __this);

        
        public  bool IsCheck() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Game_isCheck(__this_0);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte Game_isCheckmate(/* Game */ IntPtr __this);

        
        public  bool IsCheckmate() {
            var __this_0 = this.nativePtr;

            var __ret_0 = Game_isCheckmate(__this_0);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte Game_isPromotion(/* Game */ IntPtr __this, /* PieceMove */ IntPtr pieceMove);

        
        public  bool IsPromotion(/* ref */ PieceMove pieceMove_0) {
            var __this_0 = this.nativePtr;
var pieceMove_1 = pieceMove_0.nativePtr;
            var __ret_0 = Game_isPromotion(__this_0, pieceMove_1);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte Game_isEnPassant(/* Game */ IntPtr __this, /* PieceMove */ IntPtr pieceMove);

        
        public  bool IsEnPassant(/* ref */ PieceMove pieceMove_0) {
            var __this_0 = this.nativePtr;
var pieceMove_1 = pieceMove_0.nativePtr;
            var __ret_0 = Game_isEnPassant(__this_0, pieceMove_1);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte Game_isKingSideCastling(/* Game */ IntPtr __this, /* PieceMove */ IntPtr pieceMove);

        
        public  bool IsKingSideCastling(/* ref */ PieceMove pieceMove_0) {
            var __this_0 = this.nativePtr;
var pieceMove_1 = pieceMove_0.nativePtr;
            var __ret_0 = Game_isKingSideCastling(__this_0, pieceMove_1);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte Game_isQueenSideCastling(/* Game */ IntPtr __this, /* PieceMove */ IntPtr pieceMove);

        
        public  bool IsQueenSideCastling(/* ref */ PieceMove pieceMove_0) {
            var __this_0 = this.nativePtr;
var pieceMove_1 = pieceMove_0.nativePtr;
            var __ret_0 = Game_isQueenSideCastling(__this_0, pieceMove_1);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte Game_isCastling(/* Game */ IntPtr __this, /* PieceMove */ IntPtr pieceMove);

        
        public  bool IsCastling(/* ref */ PieceMove pieceMove_0) {
            var __this_0 = this.nativePtr;
var pieceMove_1 = pieceMove_0.nativePtr;
            var __ret_0 = Game_isCastling(__this_0, pieceMove_1);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte Game_isCapture(/* Game */ IntPtr __this, /* PieceMove */ IntPtr pieceMove);

        
        public  bool IsCapture(/* ref */ PieceMove pieceMove_0) {
            var __this_0 = this.nativePtr;
var pieceMove_1 = pieceMove_0.nativePtr;
            var __ret_0 = Game_isCapture(__this_0, pieceMove_1);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }
} // class

    internal static class RustOptionPieceType {
        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustOptionPieceType_new_none();

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustOptionPieceType_new_some(uint value);
        
        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint RustOptionPieceType_take(IntPtr optPtr);

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustOptionPieceType_is_some(IntPtr optPtr);

        internal static Option<PieceType> rust_to_dotnet(IntPtr optPtr)
        {
            if (RustOptionPieceType_is_some(optPtr) != 0)
            {
                var value_0 = RustOptionPieceType_take(optPtr);
                var value_1 = (PieceType)value_0;
                return new Option<PieceType>(value_1);
            }
            else
            {
                return new Option<PieceType>();
            }
        }

        internal static IntPtr dotnet_to_rust(Option<PieceType> opt)
        {
            if (opt.IsSome)
            {
                var value_0 = (uint)opt.Value;
                return RustOptionPieceType_new_some(value_0);
            }
            else
            {
                return RustOptionPieceType_new_none();
            }
        }
    }
    

        public class Option<T> {
        
            [System.Serializable]
            public class OptionNoneException : System.Exception
            {
                public OptionNoneException() :
                    base("Trying to get the value of an `Option` that is `None`") 
                {
                }
            }
        
            private T value;
            private bool isSome;
        
            public bool IsSome
            {
                get
                {
                    return isSome;
                }
            }
        
            public T Value
            {
                get {
                    if (!isSome) {
                        throw new OptionNoneException();
                    }
                    return value;
                }
            }
        
            public Option()
            {
                value = default(T);
                isSome = false;
            }
        
            public Option(T value)
            {
                if (value == null) 
                {
                    this.value = value;
                    this.isSome = false;
                }
                else
                {
                    this.value = value;
                    this.isSome = true;
                }
            }
        }        
        
    public static class RustVecPiece {
        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustVecPiece_new();
        
        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void RustVecPiece_push(IntPtr vecPtr, /* Piece */ IntPtr element);

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option<i_type> */ IntPtr RustVecPiece_iter_next(IntPtr iterPtr);
        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void RustVecPiece_iter_delete(IntPtr iterPtr);

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Piece */ IntPtr RustVecPiece_option_take(IntPtr optPtr);

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustVecPiece_option_is_some(IntPtr optPtr);


        internal static System.Collections.Generic.List<Piece> rust_to_dotnet(IntPtr iterPtr) {
            var list = new System.Collections.Generic.List<Piece>();
            while (true)
            {
                var next_rust_opt = RustVecPiece.RustVecPiece_iter_next(iterPtr);
                if (RustVecPiece_option_is_some(next_rust_opt) == 0)
                {
                    break;
                }
                var value_rust = RustVecPiece_option_take(next_rust_opt);
                var value = new Piece(value_rust);
                list.Add(value);
            }
            RustVecPiece_iter_delete(iterPtr);
            return list;
        }

        internal static IntPtr dotnet_to_rust(System.Collections.Generic.List<Piece> list) {
            var vec = RustVecPiece_new();
            foreach (var element in list)
            {
                var i_element = element.nativePtr;
                RustVecPiece.RustVecPiece_push(vec, i_element);
            }
            return vec;
        }
    }
        
    public static class RustVecSquare {
        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustVecSquare_new();
        
        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void RustVecSquare_push(IntPtr vecPtr, /* Square */ IntPtr element);

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option<i_type> */ IntPtr RustVecSquare_iter_next(IntPtr iterPtr);
        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void RustVecSquare_iter_delete(IntPtr iterPtr);

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Square */ IntPtr RustVecSquare_option_take(IntPtr optPtr);

        [DllImport("warp_square_engine_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustVecSquare_option_is_some(IntPtr optPtr);


        internal static System.Collections.Generic.List<Square> rust_to_dotnet(IntPtr iterPtr) {
            var list = new System.Collections.Generic.List<Square>();
            while (true)
            {
                var next_rust_opt = RustVecSquare.RustVecSquare_iter_next(iterPtr);
                if (RustVecSquare_option_is_some(next_rust_opt) == 0)
                {
                    break;
                }
                var value_rust = RustVecSquare_option_take(next_rust_opt);
                var value = new Square(value_rust);
                list.Add(value);
            }
            RustVecSquare_iter_delete(iterPtr);
            return list;
        }

        internal static IntPtr dotnet_to_rust(System.Collections.Generic.List<Square> list) {
            var vec = RustVecSquare_new();
            foreach (var element in list)
            {
                var i_element = element.nativePtr;
                RustVecSquare.RustVecSquare_push(vec, i_element);
            }
            return vec;
        }
    }
        } // namespace
