import basicVertexShader from "./shader.vert"
import basicFragmentShader from "./shader.frag"

function main() {
  let canvas: HTMLCanvasElement = document.querySelector("#view")!

  let vertexShaderSource = basicVertexShader
  let fragmentShaderSource = basicFragmentShader

  let gl = canvas.getContext("webgl2")!

  // Use our boilerplate utils to compile the shaders and link into a program
  var program = window.webglUtils.createProgramFromSources(gl, [
    vertexShaderSource,
    fragmentShaderSource,
  ])!

  let positionAttributeLocation = gl.getAttribLocation(program, "a_position")
  let positionBuffer = gl.createBuffer()
  let resolutionUniformLocation = gl.getUniformLocation(program, "u_resolution")
  let colorLocation = gl.getUniformLocation(program, "u_color")

  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)

  let vao = gl.createVertexArray()

  gl.bindVertexArray(vao)
  gl.enableVertexAttribArray(positionAttributeLocation)

  const size = 2 // 2 components per iteration
  const type = gl.FLOAT // the data is 32bit floats
  const normalize = false // don't normalize the data
  const stride = 0 // 0 = move forward size * sizeof(type) each iteration to get the next position
  const bufferOffset = 0 // start at the beginning of the buffer

  gl.vertexAttribPointer(
    positionAttributeLocation,
    size,
    type,
    normalize,
    stride,
    bufferOffset
  )

  window.webglUtils.resizeCanvasToDisplaySize(gl.canvas, 1)

  gl.viewport(0, 0, gl.canvas.width, gl.canvas.height)

  // Clear the canvas
  gl.clearColor(0, 0, 0, 1)
  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)

  // Tell it to use our program (pair of shaders)
  gl.useProgram(program)

  // Bind the attribute/buffer set we want.
  gl.bindVertexArray(vao)

  // Pass in the canvas resolution so we can convert from
  // pixels to clip space in the shader
  gl.uniform2f(resolutionUniformLocation, gl.canvas.width, gl.canvas.height)
}

main()

export {}
