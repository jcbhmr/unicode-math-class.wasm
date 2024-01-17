import { test, instantiate2 } from "./common.js";
import assert from "node:assert/strict";

const { crate } = await instantiate2({});
const { revision, class: class_ } = crate;

test("revision()", () => {
  assert.equal(revision(), 15);
});

test("class()", () => {
  assert.equal(class_("0"), "normal");
});
