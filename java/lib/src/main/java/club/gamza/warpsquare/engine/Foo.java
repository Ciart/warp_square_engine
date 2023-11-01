// Automatically generated by flapigen
package club.gamza.warpsquare.engine;


public final class Foo {

    public Foo(int a0) {
        mNativeObj = init(a0);
    }
    private static native long init(int a0);

    public final void setField(int a0) {
        do_setField(mNativeObj, a0);
    }
    private static native void do_setField(long self, int a0);

    public final int val() {
        int ret = do_val(mNativeObj);

        return ret;
    }
    private static native int do_val(long self);

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
    /*package*/ Foo(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;

    static {
        try {
            NativeUtils.loadLibraryFromJar("/engine_java.dll");
        } catch (java.io.IOException e) {
            e.printStackTrace();
        }
    }
}