function bar() {}

interface Foo {
  fields: Record<string, string>;
  object: {
    [key: string]: string;
  };
}

interface Bar extends Foo {}

function foo() {}
