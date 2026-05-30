export type ExampleItem = {
  id: string;
  name: string;
  createdAt: string;
};

const ITEMS: ExampleItem[] = [
  {
    id: "1",
    name: "First item",
    createdAt: new Date().toISOString(),
  },
  {
    id: "2",
    name: "Second item",
    createdAt: new Date().toISOString(),
  },
];

export function listItems(): ExampleItem[] {
  return ITEMS;
}

export function getItemById(id: string): ExampleItem | undefined {
  return ITEMS.find((item) => item.id === id);
}
