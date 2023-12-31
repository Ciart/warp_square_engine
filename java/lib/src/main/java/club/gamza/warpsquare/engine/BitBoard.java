// Automatically generated by flapigen
package club.gamza.warpsquare.engine;


public final class BitBoard {

    private BitBoard() {}

    public final Rank getRank() {
        int ret = do_getRank(mNativeObj);
        Rank convRet = Rank.fromInt(ret);

        return convRet;
    }
    private static native int do_getRank(long self);

    public final File getFile() {
        int ret = do_getFile(mNativeObj);
        File convRet = File.fromInt(ret);

        return convRet;
    }
    private static native int do_getFile(long self);

    public final Level getLevel() {
        int ret = do_getLevel(mNativeObj);
        Level convRet = Level.fromInt(ret);

        return convRet;
    }
    private static native int do_getLevel(long self);

    public static BitBoard fromSquare(Square square) {
        long a0 = square.mNativeObj;
        long ret = do_fromSquare(a0);
        BitBoard convRet = new BitBoard(InternalPointerMarker.RAW_PTR, ret);

        JNIReachabilityFence.reachabilityFence1(square);

        return convRet;
    }
    private static native long do_fromSquare(long square);

    public final Square intoSquare() {
        long ret = do_intoSquare(mNativeObj);
        Square convRet = new Square(InternalPointerMarker.RAW_PTR, ret);

        return convRet;
    }
    private static native long do_intoSquare(long self);

    public static BitBoard fromHex(String hex) {
        long ret = do_fromHex(hex);
        BitBoard convRet = new BitBoard(InternalPointerMarker.RAW_PTR, ret);

        return convRet;
    }
    private static native long do_fromHex(String hex);

    public final String toHex() {
        String ret = do_toHex(mNativeObj);

        return ret;
    }
    private static native String do_toHex(long self);

    public final BitBoard removeLevel() {
        long ret = do_removeLevel(mNativeObj);
        BitBoard convRet = new BitBoard(InternalPointerMarker.RAW_PTR, ret);

        return convRet;
    }
    private static native long do_removeLevel(long self);

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
    /*package*/ BitBoard(InternalPointerMarker marker, long ptr) {
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