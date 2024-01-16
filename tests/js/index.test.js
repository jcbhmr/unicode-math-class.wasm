import { test, instantiate2 } from "./common.js";
import assert from "node:assert/strict";

const { unicodeMathClass } = await instantiate2({});
const { revision, class: class_ } = unicodeMathClass;

test("revision()", () => {
  assert.equal(revision(), 15);
});

test("class()", () => {
  assert.equal(class_("0"), "normal");
});
