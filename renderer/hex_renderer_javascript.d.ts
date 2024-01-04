/* tslint:disable */
/* eslint-disable */
/**
* @param {any} grid_options
* @param {any[]} patterns
* @param {number} max_width
* @param {number} scale
* @returns {Uint8Array}
*/
export function draw_hex_grid(grid_options: any, patterns: any[], max_width: number, scale: number): Uint8Array;
/**
* @param {any} grid_options
* @param {any[]} patterns
* @param {number} max_width
* @param {number} width
* @param {number} height
* @returns {Uint8Array}
*/
export function draw_bound_hex_grid(grid_options: any, patterns: any[], max_width: number, width: number, height: number): Uint8Array;
/**
* @param {any} grid_options
* @param {any[]} patterns
* @param {number} max_width
* @param {number} max_scale
* @param {number} x_pad
* @param {number} y_pad
* @param {number} scale
* @returns {Uint8Array}
*/
export function draw_square_grid(grid_options: any, patterns: any[], max_width: number, max_scale: number, x_pad: number, y_pad: number, scale: number): Uint8Array;
/**
* @param {any} grid_options
* @param {any[]} patterns
* @param {number} max_width
* @param {number} max_scale
* @param {number} x_pad
* @param {number} y_pad
* @param {number} width
* @param {number} height
* @returns {Uint8Array}
*/
export function draw_bound_square_grid(grid_options: any, patterns: any[], max_width: number, max_scale: number, x_pad: number, y_pad: number, width: number, height: number): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly draw_hex_grid: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly draw_bound_hex_grid: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly draw_square_grid: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly draw_bound_square_grid: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
