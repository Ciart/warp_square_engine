// Automatically generated by flapigen
package club.gamza.warpsquare.engine;


public final class Square {

    public Square(Rank rank, File file, Level level) {
        int a0 = rank.getValue();        int a1 = file.getValue();        int a2 = level.getValue();
        mNativeObj = init(a0, a1, a2);
        JNIReachabilityFence.reachabilityFence3(rank, file, level);
    }
    private static native long init(int rank, int file, int level);

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
    /*package*/ Square(InternalPointerMarker marker, long ptr) {
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