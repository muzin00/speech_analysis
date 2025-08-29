import { invoke } from '@tauri-apps/api/core';
import type { InvokeArgs, InvokeOptions } from '@tauri-apps/api/core';

export class TauriCommand {
    constructor() { }

    async invoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<T> {
        return invoke(cmd, args, options);
    }
}
