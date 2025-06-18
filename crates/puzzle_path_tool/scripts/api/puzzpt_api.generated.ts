export type ExampleStuffPuzzptApi = { "tag": "StuffA", "value": number } | { "tag": "StuffB", "value": boolean } | { "tag": "StuffC" };

export type ExamplePuzzptApi = { "export_tag": "ExamplePuzzptApi", value: string, number: number, stuff: ExampleStuffPuzzptApi, };