#!/usr/bin/env node

import { execSync } from "child_process";
import { mkdirSync, cpSync, existsSync } from "fs";
import { join } from "path";

const ROOT_DIR = join(process.cwd());
const RUBY_WASI_DIR = join(ROOT_DIR, "compilers", "ruby-wasi");
const RUBY_TS_LIB_DIR = join(ROOT_DIR, "frontends", "ruby-ts", "lib");
const WASM_OUTPUT_DIR = join(RUBY_WASI_DIR, "target", "wasm32-wasip1", "debug");
const WASM_FILE = join(WASM_OUTPUT_DIR, "ruby_wasi.wasm");

function runCommand(command, cwd = ROOT_DIR) {
    console.log(`Running: ${command}`);
    try {
        const output = execSync(command, { cwd, stdio: "inherit" });
        return output.toString();
    } catch (error) {
        console.error(`Error running command: ${error.message}`);
        process.exit(1);
    }
}

function main() {
    console.log("=== Ruby WASI Build Script ===\n");

    // 1. 编译 ruby-wasi 模块
    console.log("1. Building ruby-wasi module...");
    runCommand("cargo build --target wasm32-wasip1", RUBY_WASI_DIR);
    console.log("✅ ruby-wasi module built successfully\n");

    // 2. 检查 wasm 文件是否存在
    if (!existsSync(WASM_FILE)) {
        console.error("❌ WASM file not found. Build failed.");
        process.exit(1);
    }

    // 3. 确保 ruby-ts/lib 目录存在
    console.log("2. Preparing ruby-ts/lib directory...");
    mkdirSync(RUBY_TS_LIB_DIR, { recursive: true });
    console.log("✅ ruby-ts/lib directory prepared\n");

    // 4. 使用 jco 转译 wasm 文件
    console.log("3. Translating WASM to TypeScript bindings...");
    try {
        runCommand(
            `jco transpile ${WASM_FILE} --out-dir ${RUBY_TS_LIB_DIR} --name ruby-wasi --typescript`,
        );
        console.log("✅ TypeScript bindings generated successfully\n");
    } catch (error) {
        console.warn("⚠️  jco not found or failed to run. Skipping TypeScript binding generation.");
        console.warn("Please install jco using: npm install -g @bytecodealliance/jco");
    }

    // 5. 完成构建
    console.log("=== Build Complete ===");
    console.log("📁 Ruby WASM module:", WASM_FILE);
    console.log("📁 TypeScript bindings:", RUBY_TS_LIB_DIR);
    console.log("\nTo run the Ruby playground:");
    console.log("1. cd frontends/homepage");
    console.log("2. pnpm install");
    console.log("3. pnpm run dev");
}

if (import.meta.url === `file://${process.argv[1]}`) {
    main();
}
