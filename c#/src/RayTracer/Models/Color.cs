using System;

namespace RayTracer.Models;

/// <summary>
/// Tuple that uses Red, Green an Blue to create a color.
/// </summary>
public readonly struct Color : IEquatable<Color>
{
    public double Red { get; } = 0;
    public double Green { get; } = 0;
    public double Blue { get; } = 0;

    public Color(double red, double green, double blue) => (Red, Green, Blue) = (red, green, blue);

    /// <summary>
    /// Determines whether this instance is equal to another object.
    /// </summary>
    /// <param name="obj"></param>
    /// <returns><c>true</c> if the object is a <see cref="Color"/> with equivalent coordinates</returns>
    public override bool Equals(object? obj) => obj is Color other && Equals(other);

    public override int GetHashCode() =>
        HashCode.Combine(Math.Round(Red, 5), Math.Round(Green, 5), Math.Round(Blue, 5));

    public override string ToString()
    {
        return $"({Red:0.00}, {Green:0.00}, {Blue:0.00})";
    }

    public bool Equals(Color other) =>
        Math.Abs(Red - other.Red) < MathConstants.Epsilon
        && Math.Abs(Green - other.Green) < MathConstants.Epsilon
        && Math.Abs(Blue - other.Blue) < MathConstants.Epsilon;

    public static bool operator ==(Color a, Color b) => a.Equals(b);

    public static bool operator !=(Color a, Color b) => !a.Equals(b);

    public static Color operator +(Color c1, Color c2) =>
        new(c1.Red + c2.Red, c1.Green + c2.Green, c1.Blue + c2.Blue);

    public static Color operator -(Color c1, Color c2) =>
        new(c1.Red - c2.Red, c1.Green - c2.Green, c1.Blue - c2.Blue);

    public static Color operator *(Color c, double scale) =>
        new(c.Red * scale, c.Green * scale, c.Blue * scale);

    public static Color operator *(double scale, Color c) => c * scale;

    /// <summary>
    /// Multiplying two colors is how we can blend them together
    /// </summary>
    /// <param name="c1"></param>
    /// <param name="c2"></param>
    /// <returns></returns>
    public static Color operator *(Color c1, Color c2) =>
        new(c1.Red * c2.Red, c1.Green * c2.Green, c1.Blue * c2.Blue);
}
