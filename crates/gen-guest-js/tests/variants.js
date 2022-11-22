const { invoke } = window.__TAURI__.tauri;
/**
 * @param {E1} x
 */
export async function e1Arg(x) {
  await invoke("plugin:imports|e1_arg", { x: x });
}
/**
 * @returns {Promise<E1>}
 */
export async function e1Result() {
  const result = await invoke("plugin:imports|e1_result");
  return result;
}
/**
 * @param {U1} x
 */
export async function u1Arg(x) {
  await invoke("plugin:imports|u1_arg", { x: x });
}
/**
 * @returns {Promise<U1>}
 */
export async function u1Result() {
  const result = await invoke("plugin:imports|u1_result");
  return result;
}
/**
 * @param {V1} x
 */
export async function v1Arg(x) {
  await invoke("plugin:imports|v1_arg", { x: x });
}
/**
 * @returns {Promise<V1>}
 */
export async function v1Result() {
  const result = await invoke("plugin:imports|v1_result");
  return result;
}
/**
 * @param {boolean} x
 */
export async function boolArg(x) {
  await invoke("plugin:imports|bool_arg", { x: x });
}
/**
 * @returns {Promise<boolean>}
 */
export async function boolResult() {
  const result = await invoke("plugin:imports|bool_result");
  return result;
}
/**
 * @param {boolean | null} a
 * @param {[] | null} b
 * @param {number | null} c
 * @param {E1 | null} d
 * @param {number | null} e
 * @param {U1 | null} f
 * @param {Option<boolean | null>} g
 */
export async function optionArg(a, b, c, d, e, f, g) {
  await invoke("plugin:imports|option_arg", {
    a: a,
    b: b,
    c: c,
    d: d,
    e: e,
    f: f,
    g: g,
  });
}
/**
 * @returns {Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, Option<boolean | null>]>}
 */
export async function optionResult() {
  const result = await invoke("plugin:imports|option_result");
  return result;
}
/**
 * @param {Casts1} a
 * @param {Casts2} b
 * @param {Casts3} c
 * @param {Casts4} d
 * @param {Casts5} e
 * @param {Casts6} f
 * @returns {Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]>}
 */
export async function casts(a, b, c, d, e, f) {
  const result = await invoke("plugin:imports|casts", {
    a: a,
    b: b,
    c: c,
    d: d,
    e: e,
    f: f,
  });
  return result;
}
/**
 * @param {void} a
 * @param {void} b
 * @param {E1} c
 * @param {[]} d
 * @param {number} e
 * @param {string} f
 */
export async function resultArg(a, b, c, d, e, f) {
  await invoke("plugin:imports|result_arg", {
    a: a,
    b: b,
    c: c,
    d: d,
    e: e,
    f: f,
  });
}
/**
 * @returns {Promise<[void, void, E1, [], number, string]>}
 */
export async function resultResult() {
  const result = await invoke("plugin:imports|result_result");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function returnResultSugar() {
  const result = await invoke("plugin:imports|return_result_sugar");
  return result;
}
/**
 * @returns {Promise<void>}
 */
export async function returnResultSugar2() {
  const result = await invoke("plugin:imports|return_result_sugar2");
  return result;
}
/**
 * @returns {Promise<MyErrno>}
 */
export async function returnResultSugar3() {
  const result = await invoke("plugin:imports|return_result_sugar3");
  return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function returnResultSugar4() {
  const result = await invoke("plugin:imports|return_result_sugar4");
  return result;
}
/**
 * @returns {Promise<number | null>}
 */
export async function returnOptionSugar() {
  const result = await invoke("plugin:imports|return_option_sugar");
  return result;
}
/**
 * @returns {Promise<MyErrno | null>}
 */
export async function returnOptionSugar2() {
  const result = await invoke("plugin:imports|return_option_sugar2");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function resultSimple() {
  const result = await invoke("plugin:imports|result_simple");
  return result;
}
/**
 * @param {IsClone} a
 */
export async function isCloneArg(a) {
  await invoke("plugin:imports|is_clone_arg", { a: a });
}
/**
 * @returns {Promise<IsClone>}
 */
export async function isCloneReturn() {
  const result = await invoke("plugin:imports|is_clone_return");
  return result;
}
/**
 * @returns {Promise<number | null>}
 */
export async function returnNamedOption() {
  const result = await invoke("plugin:imports|return_named_option");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function returnNamedResult() {
  const result = await invoke("plugin:imports|return_named_result");
  return result;
}