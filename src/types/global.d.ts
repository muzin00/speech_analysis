import { TauriCommand } from '@/lib/tauri-command';

declare global {
    interface Window {
        readonly tauriCommand: TauriCommand;
    }
}

export { };