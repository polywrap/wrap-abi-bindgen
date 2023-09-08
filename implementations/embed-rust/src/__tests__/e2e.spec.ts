import { Output, WrapInfo } from "./wrap";
import { loadTestCases, TestCase } from "./cases";
import { orderOutput } from "./output";

import { PolywrapClient } from "@polywrap/client-js";
import { parseSchema } from "@polywrap/schema-parse";
import diff from "jest-diff";
import path from "path";
import fs from "fs";

jest.setTimeout(60000);

interface WasmEmbed {
  uri: string;
  namespace: string;
  wrapInfo: number[];
  wrapWasm: number[];
}

describe("e2e", () => {

  const client: PolywrapClient = new PolywrapClient();
  let wrapperUri: string;
  let testCases: TestCase[] = loadTestCases();

  beforeAll(() => {
    // Cache wrap URI in build dir
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  });

  for (const testCase of testCases) {
    it(testCase.name, async () => {
      const abi = parseSchema(testCase.input);

      const wrapInfo: WrapInfo = {
        version: "0.1",
        name: testCase.name,
        type: "plugin",
        abi: JSON.stringify(abi),
      }

      const embeds: WasmEmbed[] = [
        {
          uri: "testimport.uri.eth",
          namespace: "TestImport",
          wrapInfo: [1, 2, 3],
          wrapWasm: [4, 5, 6]
        },
        {
          uri: "secondtestimport.uri.eth",
          namespace: "SecondTestImport",
          wrapInfo: [7, 8, 9],
          wrapWasm: [10, 11, 12]
        }
      ]

      const result = await client.invoke<Output>({
        uri: wrapperUri,
        method: "generateBindings",
        args: {
          wrapInfo,
          context: JSON.stringify({ embeds })
        }
      });

      if (!result.ok) fail(result.error);

      const received = orderOutput(result.value);
      const expected = orderOutput(testCase.output);

      const debugDir = path.join(__dirname, "debug", testCase.name);
      const receivedPath = path.join(debugDir, "received.json");
      const expectedPath = path.join(debugDir, "expected.json");
      fs.mkdirSync(debugDir, { recursive: true });
      fs.writeFileSync(receivedPath, JSON.stringify(received, null, 2));
      fs.writeFileSync(expectedPath, JSON.stringify(expected, null, 2));

      const differences = diff(expected, received, { expand: false });

      if (differences && !differences.includes("Compared values have no visual difference")) {
        fail(differences);
      }
    });
  }
});
