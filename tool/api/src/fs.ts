import {invoke} from "./core";

export interface FileEntry {
    name: string,
    path: string,
    children: FileEntry[] | null
}

export interface FsWriteFileOptions {
    dir?: BaseDirectory,
    append?: boolean,
    recursive?: boolean,
}

export interface FsBaseDirectoryOption {
    dir?: BaseDirectory,
}

export interface FsDirOptions {
    dir?: BaseDirectory,
    recursive?: boolean,
}

export type BaseDirectory =
    "ConfigLocal" |
    "Data" |
    "LocalData" |
    "Audio" |
    "Cache" |
    "Config" |
    "Desktop" |
    "Document" |
    "Download" |
    "Executable" |
    "Font" |
    "Home" |
    "Picture" |
    "Public" |
    "Runtime" |
    "Temp" |
    "Template" |
    "Video"


export interface CopyFileOptions {
    fromBaseDir?: BaseDirectory,
    toBaseDir?: BaseDirectory,
}

export interface RenameFileOptions {
    oldDir?: BaseDirectory,
    newDir?: BaseDirectory,
}

export namespace fs {
    /**
     * Copies a file to a destination.
     *
     * @example
     * import {fs} from "bevy_flurx_api";
     * await fs.copyFile("from", "to");
     */
    export const copyFile = async (
        from: string,
        to: string,
        options?: CopyFileOptions,
    ): Promise<void> => {
        await invoke("FLURX|fs::copy_file", {
            from,
            to,
            ...options
        })
    }

    /**
     * Creates a directory.
     *
     *  If you need to create the parent directory recursively, set `recursive` to `true`.
     *
     *  @example
     *  import {fs} from "bevy_flurx_api";
     *  await fs.createDir("path");
     */
    export const createDir = async (
        path: string,
        options?: FsDirOptions
    ): Promise<void> => {
        await invoke("FLURX|fs::create_dir", {path, ...options});
    }

    /**
     *  Check if a path exists.
     *
     *  @example
     *  import {fs} from "bevy_flurx_api";
     *  const exists: boolean = await fs.exists("path");
     */
    export const exists = async (
        path: string,
        options?: FsBaseDirectoryOption,
    ): Promise<boolean> => {
        return await invoke("FLURX|fs::exists", {
            path,
            ...options
        });
    }

    /**
     * Reads a file as byte array.
     *
     * @example
     * import {fs} from "bevy_flurx_api";
     * const bytes: Uint8Array = await fs.readBinaryFile("path");
     */
    export const readBinaryFile = async (
        path: string,
        options?: FsBaseDirectoryOption,
    ): Promise<Uint8Array> => {
        return await invoke("FLURX|fs::read_binary_file", {
            path,
            ...options
        });
    }

    /**
     * Reads a file as a UTF-8 encoded string.
     *
     * @example
     * import {fs} from "bevy_flurx_api";
     * const text: string = await fs.readTextFile("path");
     */
    export const readTextFile = async (
        path: string,
        options?: FsBaseDirectoryOption,
    ): Promise<string> => {
        return await invoke("FLURX|fs::read_text_file", {
            path,
            ...options
        });
    }

    /**
     * Removes a file.
     *
     * @example
     * import {fs} from "bevy_flurx_api";
     * await fs.removeFile("path");
     */
    export const removeFile = async (
        path: string,
        options?: FsBaseDirectoryOption,
    ): Promise<void> => {
        await invoke("FLURX|fs::remove_file", {
            path,
            ...options
        });
    }

    /**
     * Renames a file.
     *
     * @example
     * import {fs} from "bevy_flurx_api";
     * await fs.renameFile("oldPath", "newPath");
     */
    export const renameFile = async (
        oldPath: string,
        newPath: string,
        options?: RenameFileOptions,
    ): Promise<void> => {
        await invoke("FLURX|fs::rename_file", {
            oldPath,
            newPath,
            ...options
        });
    }

    /**
     * Writes a UTF-8 text file.
     *
     * @example
     * import {fs} from "bevy_flurx_api";
     * await fs.writeTextFile("path", "contents");
     */
    export const writeTextFile = async (
        path: string,
        contents: string,
        options?: FsWriteFileOptions
    ): Promise<void> => {
        await invoke("FLURX|fs::write_text_file", {
            path,
            contents,
            ...options
        });
    }

    /**
     * Writes a file.
     *
     * @example
     * import {fs} from "bevy_flurx_api";
     * await fs.writeBinaryFile("path", new Uint8Array());
     */
    export const writeBinaryFile = async (
        path: string,
        contents: Uint8Array | Iterable<number> | ArrayLike<number> | ArrayBuffer,
        options?: FsWriteFileOptions
    ): Promise<void> => {
        await invoke("FLURX|fs::write_binary_file", {
            path,
            contents,
            ...options
        });
    }

    /**
     * List directory files.
     *
     * @example
     * import {fs} from "bevy_flurx_api";
     * const files: FileEntry[] = await fs.readDir("path");
     */
    export const readDir = async (
        path: string,
        options?: FsBaseDirectoryOption,
    ): Promise<FileEntry[]> => {
        return await invoke("FLURX|fs::read_dir", {
            path,
            ...options
        });
    }

    /**
     * Remove a directory.
     *
     * @example
     * import {fs} from "bevy_flurx_api";
     * await fs.removeDir("path");
     */
    export const removeDir = async (
        path: string,
        options?: FsDirOptions,
    ): Promise<void> => {
        await invoke("FLURX|fs::remove_dir", {path, ...options});
    }
}