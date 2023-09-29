import fs from "fs";


export function createDirSync(dir: string, clearDir: boolean = false) {
    // remove trailing slashes
    dir = stripSlashes(dir);
    // If the directory exists and clearDir is true
    if (fs.existsSync(dir) && clearDir) {
        // Wipe the directory
        fs.rmdirSync(dir, { recursive: true });
    }
    // make the directory
    fs.mkdirSync(dir, { recursive: true });
}

export function stripSlashes(path: string) {
    return path.replace(/\/$/, "");
}