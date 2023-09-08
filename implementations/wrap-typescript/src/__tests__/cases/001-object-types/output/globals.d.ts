declare interface Result<T = any> {
  ok: boolean;
  error: string | undefined;
  value: T | undefined;
}
declare const __wrap_subinvoke: (
  uri: string,
  name: string,
  args: any
) => Result;
declare const __wrap_getImplementations: (
  uri: string
) => string[];
declare const __wrap_abort: (args: any) => void;
