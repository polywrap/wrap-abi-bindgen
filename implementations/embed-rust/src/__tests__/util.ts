import {ImportedModuleDefinition} from "@polywrap/schema-parse";
import {PolywrapClient} from "@polywrap/client-js";

export interface WasmEmbed {
  uri: string;
  namespace: string;
  wrapInfo: Uint8Array;
  wrapWasm: Uint8Array;
}

export async function getWasmEmbeds(
  importedModules: ImportedModuleDefinition[],
  client: PolywrapClient,
): Promise<WasmEmbed[]> {
  const embeds: WasmEmbed[] = [];

  for (const importedModule of importedModules ?? []) {
    if (importedModule.isInterface) {
      continue;
    }
    const uri = importedModule.uri;

    const manifest = await client.getFile(uri, { path: "wrap.info" });
    if (!manifest.ok) {
      throw Error ("Failed to retrieve manifest for uri: " + uri);
    }

    const module = await client.getFile(uri, { path: "wrap.wasm" });
    if (!module.ok) {
      // The error is ignored because getFile is expected to fail for plugins and interfaces
      continue;
    }

    embeds.push({
      uri,
      namespace: importedModule.namespace,
      wrapInfo: manifest.value as Uint8Array,
      wrapWasm: module.value as Uint8Array,
    });
  }

  return embeds;
}
