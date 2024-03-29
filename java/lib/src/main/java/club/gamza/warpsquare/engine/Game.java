// Automatically generated by flapigen
package club.gamza.warpsquare.engine;


public final class Game {

    public Game() {
        mNativeObj = init();
    }
    private static native long init();

    public final Square [] getAttackSquares(Square square) {
        long a0 = square.mNativeObj;
        Square [] ret = do_getAttackSquares(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(square);

        return ret;
    }
    private static native Square [] do_getAttackSquares(long self, long square);

    public final boolean legalPieceMove(PieceMove pieceMove) {
        long a0 = pieceMove.mNativeObj;
        pieceMove.mNativeObj = 0;

        boolean ret = do_legalPieceMove(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(pieceMove);

        return ret;
    }
    private static native boolean do_legalPieceMove(long self, long pieceMove);

    public final boolean legalBoardMove(BoardMove boardMove) {
        long a0 = boardMove.mNativeObj;
        boardMove.mNativeObj = 0;

        boolean ret = do_legalBoardMove(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(boardMove);

        return ret;
    }
    private static native boolean do_legalBoardMove(long self, long boardMove);

    public final void pushPieceMove(PieceMove pieceMove) throws Exception {
        long a0 = pieceMove.mNativeObj;
        pieceMove.mNativeObj = 0;

        do_pushPieceMove(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(pieceMove);
    }
    private static native void do_pushPieceMove(long self, long pieceMove) throws Exception;

    public final void pushBoardMove(BoardMove boardMove) throws Exception {
        long a0 = boardMove.mNativeObj;
        boardMove.mNativeObj = 0;

        do_pushBoardMove(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(boardMove);
    }
    private static native void do_pushBoardMove(long self, long boardMove) throws Exception;

    public final void popMove(BoardMove boardMove) throws Exception {
        long a0 = boardMove.mNativeObj;
        boardMove.mNativeObj = 0;

        do_popMove(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(boardMove);
    }
    private static native void do_popMove(long self, long boardMove) throws Exception;

    public final void print() {
        do_print(mNativeObj);
    }
    private static native void do_print(long self);

    public final Color getTurn() {
        int ret = do_getTurn(mNativeObj);
        Color convRet = Color.fromInt(ret);

        return convRet;
    }
    private static native int do_getTurn(long self);

    public final long getFullMoveNumber() {
        long ret = do_getFullMoveNumber(mNativeObj);

        return ret;
    }
    private static native long do_getFullMoveNumber(long self);

    public final long getHalfMoveClock() {
        long ret = do_getHalfMoveClock(mNativeObj);

        return ret;
    }
    private static native long do_getHalfMoveClock(long self);

    public final Piece [] getPieces() {
        Piece [] ret = do_getPieces(mNativeObj);

        return ret;
    }
    private static native Piece [] do_getPieces(long self);

    public final Piece [] getCapturedPieces() {
        Piece [] ret = do_getCapturedPieces(mNativeObj);

        return ret;
    }
    private static native Piece [] do_getCapturedPieces(long self);

    public final boolean isCheck() {
        boolean ret = do_isCheck(mNativeObj);

        return ret;
    }
    private static native boolean do_isCheck(long self);

    public final boolean isCheckmate() {
        boolean ret = do_isCheckmate(mNativeObj);

        return ret;
    }
    private static native boolean do_isCheckmate(long self);

    public final boolean isPromotion(PieceMove pieceMove) {
        long a0 = pieceMove.mNativeObj;
        pieceMove.mNativeObj = 0;

        boolean ret = do_isPromotion(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(pieceMove);

        return ret;
    }
    private static native boolean do_isPromotion(long self, long pieceMove);

    public final boolean isEnPassant(PieceMove pieceMove) {
        long a0 = pieceMove.mNativeObj;
        pieceMove.mNativeObj = 0;

        boolean ret = do_isEnPassant(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(pieceMove);

        return ret;
    }
    private static native boolean do_isEnPassant(long self, long pieceMove);

    public final boolean isKingSideCastling(PieceMove pieceMove) {
        long a0 = pieceMove.mNativeObj;
        pieceMove.mNativeObj = 0;

        boolean ret = do_isKingSideCastling(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(pieceMove);

        return ret;
    }
    private static native boolean do_isKingSideCastling(long self, long pieceMove);

    public final boolean isQueenSideCastling(PieceMove pieceMove) {
        long a0 = pieceMove.mNativeObj;
        pieceMove.mNativeObj = 0;

        boolean ret = do_isQueenSideCastling(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(pieceMove);

        return ret;
    }
    private static native boolean do_isQueenSideCastling(long self, long pieceMove);

    public final boolean isCastling(PieceMove pieceMove) {
        long a0 = pieceMove.mNativeObj;
        pieceMove.mNativeObj = 0;

        boolean ret = do_isCastling(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(pieceMove);

        return ret;
    }
    private static native boolean do_isCastling(long self, long pieceMove);

    public final boolean isCapture(PieceMove pieceMove) {
        long a0 = pieceMove.mNativeObj;
        pieceMove.mNativeObj = 0;

        boolean ret = do_isCapture(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(pieceMove);

        return ret;
    }
    private static native boolean do_isCapture(long self, long pieceMove);

    public synchronized void delete() {
        if (mNativeObj != 0) {
            do_delete(mNativeObj);
            mNativeObj = 0;
       }
    }
    @Override
    protected void finalize() throws Throwable {
        try {
            delete();
        }
        finally {
             super.finalize();
        }
    }
    private static native void do_delete(long me);
    /*package*/ Game(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;

    static {
        try {
            NativeUtils.loadLibraryFromJar();
        } catch (java.io.IOException e) {
            e.printStackTrace();
        }
    }
}