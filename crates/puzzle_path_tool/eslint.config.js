import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import { defineConfig } from "eslint/config";

const files = ["scripts/**/*.ts", "generated/**/*.ts"];

export default defineConfig([
    {
        files: files,
        plugins: { js },
        extends: ["js/recommended"],
    },
    tseslint.configs.recommended.map((c) => ({ files: files, ...c })),
    {
        files: files,
        languageOptions: { globals: globals.es2020 },
        rules: {
            "no-unused-vars": "off",
            "@typescript-eslint/no-unused-vars": "off",
        },
    },
]);
