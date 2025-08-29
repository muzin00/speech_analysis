import { invoke } from '@tauri-apps/api/core';
import type { InvokeArgs, InvokeOptions } from '@tauri-apps/api/core';

const ROOT_COMMAND = 'root_handler';

export class TauriCommand {
    constructor() { }

    async invoke<T>(path: string, args?: InvokeArgs, options?: InvokeOptions): Promise<T> {
        const { headers } = options || {};
        return invoke(ROOT_COMMAND, args, {
            headers: {
                Path: path,
                ...headers || {},
            },
        });
    }
}
