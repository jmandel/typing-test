/* tslint:disable */
/* eslint-disable */
/**
*/
export class AlignmentTable {
  free(): void;
/**
* @param {string} a
* @returns {AlignmentTable}
*/
  static new(a: string): AlignmentTable;
/**
* @param {number} row
* @param {number} col
* @returns {number}
*/
  score_at(row: number, col: number): number;
/**
* @param {string} bitems
*/
  type_into_b(bitems: string): void;
/**
* @param {number} count
*/
  backspace_into_b(count: number): void;
/**
* @param {number} count
*/
  backword_into_b(count: number): void;
/**
* @param {number} previous_b_length
* @returns {boolean | undefined}
*/
  align(previous_b_length: number): boolean | undefined;
/**
* @returns {ScoredAlignment}
*/
  best_scored_alignment(): ScoredAlignment;
}
/**
*/
export class ScoredAlignment {
  free(): void;
/**
* @returns {number}
*/
  score(): number;
/**
* @returns {string}
*/
  alignment(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_scoredalignment_free: (a: number) => void;
  readonly scoredalignment_score: (a: number) => number;
  readonly scoredalignment_alignment: (a: number, b: number) => void;
  readonly __wbg_alignmenttable_free: (a: number) => void;
  readonly alignmenttable_new: (a: number, b: number) => number;
  readonly alignmenttable_score_at: (a: number, b: number, c: number) => number;
  readonly alignmenttable_type_into_b: (a: number, b: number, c: number) => void;
  readonly alignmenttable_backspace_into_b: (a: number, b: number) => void;
  readonly alignmenttable_backword_into_b: (a: number, b: number) => void;
  readonly alignmenttable_align: (a: number, b: number) => number;
  readonly alignmenttable_best_scored_alignment: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
