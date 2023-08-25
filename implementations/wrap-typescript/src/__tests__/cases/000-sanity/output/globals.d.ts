declare const __wrap_args: any;
declare const __wrap_method: string;
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
declare const __wrap_abort: (args: any) => void;
