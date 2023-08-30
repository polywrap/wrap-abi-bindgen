import { Output } from "../wrap";
import { loadOutput } from "../output";

import fs from "fs";
import path from "path";

export interface TestCase {
  name: string;
  input: string;
  output: Output;
}

export function loadTestCases(): TestCase[] {
  const testCases: TestCase[] = [];
  const casesDirents = fs.readdirSync(__dirname, { withFileTypes: true });

  for (const casesDirent of casesDirents) {
    if (!casesDirent.isDirectory()) {
      continue;
    }

    const caseDir = path.join(__dirname, casesDirent.name);
    const caseInputPath = path.join(caseDir, "input.graphql");
    const caseInput = fs.readFileSync(caseInputPath, "utf-8");
    const caseOutputDir = path.join(caseDir, "output");
    const caseOutput = loadOutput(caseOutputDir, ["input.graphql"]);
    testCases.push({
      name: casesDirent.name,
      input: caseInput,
      output: caseOutput
    });
  }

  return testCases;
}
