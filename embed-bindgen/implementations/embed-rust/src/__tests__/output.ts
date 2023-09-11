import { Output, File, Directory } from "./wrap";

import fs from "fs";
import path from "path";

export function orderOutput(output: Output): Output {
  orderDirectory(output);
  return output;
}

function orderDirectory(directory: Output | Directory): void {
  directory.dirs.sort((a, b) => a.name > b.name ? -1 : 1);
  directory.files.sort((a, b) => a.name > b.name ? -1 : 1);
  for (const dir of directory.dirs) {
    orderDirectory(dir);
  }
}

export function loadOutput(dir: string, ignore: string[]): Output {
  const output: Output = {
    files: [],
    dirs: []
  };

  const outputDirents = fs.readdirSync(dir, { withFileTypes: true });

  for (const outputDirent of outputDirents) {
    if (ignore.includes(outputDirent.name)) {
      continue;
    }

    if (outputDirent.isDirectory()) {
      output.dirs.push(loadDirectory(dir, outputDirent.name, ignore));
    } else {
      output.files.push(loadFile(dir, outputDirent.name));
    }
  }

  return output;
}

function loadFile(dir: string, name: string): File {
  const filePath = path.join(dir, name);
  const fileData = fs.readFileSync(filePath, "utf-8");
  return {
    name: name,
    data: fileData
  };
}

function loadDirectory(dir: string, name: string, ignore: string[]): Directory {
  const directory: Directory = {
    name: name,
    files: [],
    dirs: []
  };
  const directoryPath = path.join(dir, name);
  const directoryEnts = fs.readdirSync(
    directoryPath,
    { withFileTypes: true }
  );

  for (const directoryEnt of directoryEnts) {
    if (ignore.includes(directoryEnt.name)) {
      continue;
    }

    if (directoryEnt.isDirectory()) {
      directory.dirs.push(
        loadDirectory(directoryPath, directoryEnt.name, ignore)
      );
    } else {
      directory.files.push(
        loadFile(directoryPath, directoryEnt.name)
      );
    }
  }

  return directory;
}
