using System;
using System.Text;

namespace RayTracer.Models;

public class Canvas
{
    public int Width { get; }
    public int Height { get; }

    /// <summary>
    /// Flat array to contain the pixels in the canvas.
    /// Using a one-dimensional array vs a 2D allows for faster memory access.
    ///
    /// To get a point on the array use, <c>index = y * width + x</c>
    /// </summary>
    /// <value></value>
    public Color[] Pixels { get; }

    public Canvas(int width, int height)
    {
        Width = width;
        Height = height;
        Pixels = new Color[width * height];
    }

    /// <summary>
    /// Writes a color to the canvas at the give coridinates
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="color"></param>
    public void WritePixel(int x, int y, Color color)
    {
        Pixels[y * Width + x] = color;
    }

    public Color PixelAt(int x, int y) => Pixels[y * Width + x];

    /// <summary>
    /// Returns the canvas in a PPM formatted string.
    ///
    /// Each PPM file begins with a header consiting of three lines:
    /// P3      //The type of PPM files being used
    /// 80 40   // Width and Height of the canvas
    /// 255     // The max color value that r,g,b values can have
    /// </summary>
    /// <returns></returns>
    public string ToPPM()
    {
        var sb = new StringBuilder();
        sb.AppendLine("P3");
        sb.AppendLine($"{Width} {Height}");
        sb.AppendLine("255");

        for (int y = 0; y < Height; y++)
        {
            // Each Color has 3 componetns, RGB, that is Width * 3 values for each row
            var components = new List<string>(Width * 3);

            // Build a list of "r g b" values for this row
            for (int x = 0; x < Width; x++)
            {
                var c = Pixels[y * Width + x];

                components.Add(ScaleTo255(c.R).ToString());
                components.Add(ScaleTo255(c.G).ToString());
                components.Add(ScaleTo255(c.B).ToString());
            }
            var row = string.Join(" ", components);

            // Wrap the row based on 70 character limit
            foreach (var line in WrapAt70(row))
            {
                sb.AppendLine(line);
            }
        }

        return sb.ToString();
    }

    /// <summary>
    /// Splits a long string into multiple lines, ensuring that no line exceeds
    /// 70 characters, as required by the PPM file format.
    /// </summary>
    /// <param name="text">
    /// </param>
    /// <returns>
    /// An <see cref="IEnumerable{String}"/> sequence of wrapped lines, each no longer
    /// than 70 characters. Lines are broken only at spaces so that numeric pixel
    /// components remain intact.
    /// </returns>
    /// <remarks>
    /// This method searches for the last space before the 70-character limit and
    /// yields the substring up to that point. Remaining text is processed
    /// iteratively until the full input has been split. No trailing spaces are
    /// introduced in the output.
    /// </remarks>
    private static IEnumerable<string> WrapAt70(string text)
    {
        const int max = 70;

        while (text.Length > max)
        {
            // break at the last space before the limit
            int breakPos = text.LastIndexOf(' ', max);

            if (breakPos == -1)
            {
                break;
            }

            yield return text.Substring(0, breakPos);

            text = text.Substring(breakPos + 1);
        }

        yield return text;
    }

    private static int ScaleTo255(double value)
    {
        var clamped = Math.Clamp(value, 0.0, 1.0);
        return (int)Math.Round(clamped * 255.0);
    }
}
