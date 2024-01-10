type Obj = {};
type arry = [];
type lit = string;
type generic<T> = T;
type genericOtherSide<T> = Readonly<T>;
type UnionObjects =
  | { kind: 'circle'; radius: number }
  | { kind: 'square'; x: number }
  | { kind: 'triangle'; x: number; y: number };

function foo() {}
