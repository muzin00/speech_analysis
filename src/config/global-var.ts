import { TauriCommand } from "@/lib/tauri-command";

Object.defineProperty(window, 'tauriCommand', {
    value: new TauriCommand(),
    writable: false,
    configurable: false
});
