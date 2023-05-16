import { PolywrapClient } from "@polywrap/client-js";
import { parseSchema, Abi } from "@polywrap/schema-parse";
import path from "path";
import fs from "fs";

jest.setTimeout(60000);

describe("e2e", () => {

  const client: PolywrapClient = new PolywrapClient();
  let wrapperUri: string;
  let abi: Abi;

  beforeAll(() => {
    // Cache wrap URI in build dir
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;

    // Parse the input schema
    const schema = fs.readFileSync(path.join(__dirname, "./input.graphql"), "utf-8");
    abi = parseSchema(schema);
  });

  it("generateBindings - test", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "generateBindings",
      args: {
        wrapAbi: JSON.stringify(abi),
        projectName: "foo"
      }
    });

    console.log(result);
    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    console.log((result.value as any).dirs[0].files);
  });
});
