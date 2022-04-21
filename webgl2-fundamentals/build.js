import esbuild from "esbuild"

import { nodeExternalsPlugin } from "esbuild-node-externals"
import { copy } from "esbuild-plugin-copy"
import { glsl } from "esbuild-plugin-glsl"

const lesson = process.argv[2]

if (!lesson) {
  console.log("invalid lesson arg")
  process.exit(1)
}

console.log(
  `
esbuild lesson: ${lesson}
`
)

const config = {
  entryPoints: [`./lessons/${lesson}/index.ts`],
  outdir: `./build/${lesson}`,
  bundle: true,
  minify: true,
  platform: "browser",
  sourcemap: true,
  target: "es2020",
  plugins: [
    nodeExternalsPlugin(),
    glsl({ minify: false }),
    copy({
      // this is equal to process.cwd(), which means we use cwd path as base path to resolve `to` path
      // if not specified, this plugin uses ESBuild.build outdir/outfile options as base path.
      resolveFrom: "out",
      assets: [
        {
          from: "./common/*",
          to: "./",
        },
      ],
    }),
  ],
}

esbuild.build(config).catch(() => process.exit(1))
