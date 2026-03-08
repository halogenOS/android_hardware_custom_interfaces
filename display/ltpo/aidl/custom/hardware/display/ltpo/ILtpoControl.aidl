package custom.hardware.display.ltpo;

@VintfStability
interface ILtpoControl {
    void setEnabled(boolean enabled);
    void onIdle();
    void onActive();
    int getCurrentHz();
    int setTargetHz(int targetHz);
    int[] getSupportedHz();
}
