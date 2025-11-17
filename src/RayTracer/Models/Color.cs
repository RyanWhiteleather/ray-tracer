using System;

namespace RayTracer.Models;

/// <summary>
/// Colors are represented by a Tuple with red (R), green (G), and blue (B) components.
/// </summary>
public readonly struct Color : IEquatable<Color>
{
    public double R { get; }
    public double G { get; }
    public double B { get; }

    public Color(double r, double g, double b) => (R, G, B) = (r, g, b);

    /// <summary>
    /// Determines whether this instance is equal to another object.
    /// </summary>
    /// <param name="obj"></param>
    /// <returns><c>true</c> if the object is a <see cref="color"/> with equivalent coordinates</returns>
    public override bool Equals(object? obj) => obj is Color other && Equals(other);

    public override int GetHashCode() =>
        HashCode.Combine(Math.Round(R, 5), Math.Round(G, 5), Math.Round(B, 5));

    /// <summary>
    /// Determines whether two <see cref="Color"/> instances are equal within a small margin of error.
    /// </summary>
    /// <param name="other">The other Color to compare.</param>
    /// <returns><c>true</c> if the colors are equal; otherwise, <c>false</c>.</returns>
    public bool Equals(Color other) =>
        Math.Abs(R - other.R) < MathConstants.Epsilon
        && Math.Abs(G - other.G) < MathConstants.Epsilon
        && Math.Abs(B - other.B) < MathConstants.Epsilon;

    public override string ToString()
    {
        return $"({R:0.00}, {G:0.00}, {B:0.00})";
    }

    #region Operator Overloads

    public static bool operator ==(Color a, Color b) => a.Equals(b);

    public static bool operator !=(Color a, Color b) => !a.Equals(b);

    public static Color operator +(Color c1, Color c2) =>
        new(c1.R + c2.R, c1.G + c2.G, c1.B + c2.B);

    public static Color operator -(Color c1, Color c2) =>
        new(c1.R - c2.R, c1.G - c2.G, c1.B - c2.B);

    public static Color operator *(Color c, double s) => new(c.R * s, c.G * s, c.B * s);

    public static Color operator *(double s, Color c) => c * s;

    public static Color operator /(Color p, double scale) =>
        new(p.R / scale, p.G / scale, p.B / scale);

    /// <summary>
    /// Component-wise (Hadamard) multiplication.
    /// Used when mixing colors and computing lighting.
    /// </summary>
    /// <param name="c1"></param>
    /// <param name="c2"></param>
    /// <returns></returns>
    public static Color operator *(Color c1, Color c2) =>
        new(c1.R * c2.R, c1.G * c2.G, c1.B * c2.B);

    #endregion
}
