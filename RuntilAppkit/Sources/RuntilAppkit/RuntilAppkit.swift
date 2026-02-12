import AppKit
import CoreFoundation

struct State {
    var source: CFRunLoopSource;
    var loop: CFRunLoop;
}

class RAppDelegate {
    
}

@MainActor
class RuntilAppkitState {
    static let shared = RuntilAppkitState();
    
    var state: State?;
}

@MainActor
@_cdecl("runtilappkit_init")
func runtilAppkitInit(ud: UnsafeMutableRawPointer, callback: @convention(c) (UnsafeMutableRawPointer?) -> Void) {
    let loop = RunLoop.current.getCFRunLoop();
    
    assert(RuntilAppkitState.shared.state == nil, "RuntilAppkitState is already initialized");
    
    var source_cx = CFRunLoopSourceContext();
    source_cx.info = ud;
    source_cx.perform = callback;
    let source = CFRunLoopSourceCreate(nil, 1, &source_cx)!;
    CFRunLoopAddSource(loop, source, .commonModes)
    
    let app = NSApplication.shared;
    app.setActivationPolicy(.regular);
    
    
    RuntilAppkitState.shared.state = State(source: source, loop: loop)
}

@_cdecl("runtilappkit_schedule")
func runtilAppkitSchedule() {
    Task { @MainActor in
        guard let state = RuntilAppkitState.shared.state else {
            assertionFailure("RuntilAppkitState is not initialized")
            return
        }
        
        CFRunLoopSourceSignal(state.source);
        CFRunLoopWakeUp(state.loop);
    }
}

@MainActor
@_cdecl("runtilappkit_run")
func runtilAppkitRun() {
    guard let state = RuntilAppkitState.shared.state else {
        assertionFailure("RuntilAppkitState is not initialized")
        return
    }
    NSApp.activate(ignoringOtherApps: true)
    NSApp.run()
}

@MainActor
@_cdecl("runtilappkit_destroy")
func runtilAppkitDestroy() {
    RuntilAppkitState.shared.state = nil
}

@MainActor
class RWindow {
    var window: NSWindow;
    
    @MainActor
    init() {
        self.window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 480, height: 300),
            styleMask: [.titled, .closable, .miniaturizable, .resizable],
            backing: .buffered,
            defer: false
        );
    }
    
    @MainActor
    deinit {
        self.window.close();
    }
}

@MainActor
@_cdecl("runtilappkit_create_window")
func runtilAppkitCreateWindow() -> UnsafeMutableRawPointer {
    let wnd = RWindow();
    let ptr = Unmanaged.passRetained(wnd).toOpaque();
    return ptr;
}

@MainActor
@_cdecl("runtilappkit_show_window")
func runtilAppkitShowWindow(ptr: UnsafeMutableRawPointer) {
    let wnd = Unmanaged<RWindow>.fromOpaque(ptr).takeUnretainedValue();
    wnd.window.makeKeyAndOrderFront(nil);
}

@MainActor
@_cdecl("runtilappkit_destroy_window")
func runtilAppkitDestroyWindow(ptr: UnsafeMutableRawPointer) {
    let _ = Unmanaged<RWindow>.fromOpaque(ptr).takeRetainedValue();
}
