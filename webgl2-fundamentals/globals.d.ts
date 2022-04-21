type ErrorCallback = (string) => any

type WebGLUtilsExport = {
  createProgram: (
    gl: WebGL2RenderingContext,
    shaders: string[],
    opt_attribs?: string[],
    opt_locations?: number[]
  ) => WebGLProgram | null
  createProgramFromScripts: (
    gl: WebGL2RenderingContext,
    shaderScriptIds: string[],
    opt_attribs?: string[],
    opt_locations?: number[]
  ) => WebGLProgram | null
  createProgramFromSources: (
    gl: WebGL2RenderingContext,
    shaderSources: string[],
    opt_attribs?: string[],
    opt_locations?: number[],
    opt_errorCallback?: ErrorCallback
  ) => WebGLProgram | null
  resizeCanvasToDisplaySize: (
    canvas: HTMLCanvasElement,
    multiplier: number
  ) => boolean
}

declare global {
  interface Window {
    webglUtils: WebGLUtilsExport
  }
}

declare const webglUtils: WebGLUtilsExport

export default global
