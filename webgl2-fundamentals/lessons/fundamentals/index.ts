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

  // draw
  for (var ii = 0; ii < 50; ++ii) {
    // Setup a random rectangle
    setRectangle(
      gl,
      randomInt(300),
      randomInt(300),
      randomInt(300),
      randomInt(300)
    )

    // Set a random color
    gl.uniform4f(colorLocation, Math.random(), Math.random(), Math.random(), 1)

    // Draw the rectangle
    let primitiveType = gl.TRIANGLES
    let offset = 0
    let count = 6

    gl.drawArrays(primitiveType, offset, count)
  }

  // Returns a random integer from 0 to range - 1.
  function randomInt(range: number) {
    return Math.floor(Math.random() * range)
  }

  function setRectangle(
    gl: WebGL2RenderingContext,
    x: number,
    y: number,
    width: number,
    height: number
  ) {
    var x1 = x
    var x2 = x + width
    var y1 = y
    var y2 = y + height

    // NOTE: gl.bufferData(gl.ARRAY_BUFFER, ...) will affect
    // whatever buffer is bound to the `ARRAY_BUFFER` bind point
    // but so far we only have one buffer. If we had more than one
    // buffer we'd want to bind that buffer to `ARRAY_BUFFER` first.

    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array([x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2]),
      gl.STATIC_DRAW
    )
  }
}

main()

export {}
