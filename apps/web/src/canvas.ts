export function drawCanvas(
    canvas: HTMLCanvasElement,
    width: number,
    height: number,
    pixels: Uint8ClampedArray,
) {
    canvas.width = width;
    canvas.height = height;

    const ctx = canvas.getContext("2d");
    if (!ctx) throw new Error("No 2D context");

    const imageData = new ImageData(pixels as ImageDataArray, width, height);
    ctx.putImageData(imageData, 0, 0);
}
