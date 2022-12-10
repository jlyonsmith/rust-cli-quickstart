#!/usr/bin/env -S deno run --unstable --allow-run --allow-read --allow-net

import * as path from "https://deno.land/std@0.167.0/path/mod.ts";
import * as colors from "https://deno.land/std@0.167.0/fmt/colors.ts";
import { parse } from "https://deno.land/std@0.167.0/flags/mod.ts";
import { File, Dir } from "http://deno.land/x/fs_pro@3.11.0/mod.ts";

const log = {
  info: (s: string) => console.log("👉 " + colors.green(s)),
  error: (s: string) => console.error("💥 " + colors.red(s)),
  warning: (s: string) => console.error("🐓 " + colors.yellow(s)),
};

const args = parse(Deno.args);

if (args.help || args._.length < 1) {
  console.log(`Usage: ${path.basename(Deno.mainModule)} [PROJECT-NAME]`);
  Deno.exit(1);
}

try {
  customizeProject(args._[0]);
} catch (error) {
  log.error(error.message);
  Deno.exit(1);
}

Deno.exit(0);

function customizeProject(project_name: any) {
  log.info(`${project_name}`);
}
