import Cocoa
import SystemMonitorObjC

class AppDelegate: NSObject, NSApplicationDelegate {
    let statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
    func applicationDidFinishLaunching(_ notification: Notification) {
        if let button = statusItem.button { 
            let metrics = run_system_monitor()
            button.title = String(format: "CPU: %.1f%% (%.0f MHz) | Temp: CPU %.1f°C | GPU %.1f°C | RAM: %.2f/%.2f GB (%.1f%%)",
                metrics.cpu_usage, metrics.cpu_freq, metrics.cpu_temp, metrics.gpu_temp, metrics.ram_usage,
                metrics.ram_total, metrics.ram_usage / metrics.ram_total);

            let menu = NSMenu()
            menu.addItem(NSMenuItem(title: "Quit", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q"))
            statusItem.menu = menu
        }
    }
}