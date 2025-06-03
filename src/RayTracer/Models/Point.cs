namespace RayTracer.Models;

/// <summary>
/// Represents a point in 3D space with immutable X, Y, and Z coordinates.
/// Points support value-based equality and arithmetic with vectors.
/// </summary>
public readonly struct Point : IEquatable<Point>
{
    public double X { get; }
    public double Y { get; }
    public double Z { get; }

    private double W { get; } = 1;

    public Point(double x, double y, double z) => (X, Y, Z) = (x, y, z);

    /// <summary>
    /// Determines whether this instance is equal to another object.
    /// </summary>
    /// <param name="obj"></param>
    /// <returns><c>true</c> if the object is a <see cref="Point"/> with equivalent coordinates</returns>
    public override bool Equals(object? obj)
    {
        return obj is Point other && Equals(other);
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(Math.Round(X, 5), Math.Round(Y, 5), Math.Round(Z, 5));
    }

    /// <summary>
    /// Determines whether two <see cref="Point"/> instances are equal within a small margin of error.
    /// </summary>
    /// <param name="other">The other point to compare.</param>
    /// <returns><c>true</c> if the points are equal; otherwise, <c>false</c>.</returns>
    public bool Equals(Point other)
    {
        return Math.Abs(X - other.X) < MathConstants.Epsilon
            && Math.Abs(Y - other.Y) < MathConstants.Epsilon
            && Math.Abs(Z - other.Z) < MathConstants.Epsilon;
    }

    public static bool operator ==(Point a, Point b) => a.Equals(b);

    public static bool operator !=(Point a, Point b) => !a.Equals(b);

    /// <summary>
    /// Translates the give point by a vecto, returing a new point
    /// </summary>
    /// <param name="p">The original point.</param>
    /// <param name="v">The vector used to translate the point.</param>
    /// <returns>A new <see cref="Point"/> after applying the vector offset.</returns>
    public static Point operator +(Point p, Vector v)
    {
        return new(p.X + v.X, p.Y + v.Y, p.Z + v.Z);
    }

    /// <summary>
    /// Subtracts one point from another, resulting in a vector that points from <paramref name="p2"/> to <paramref name="p1"/>.
    /// </summary>
    /// <param name="p1">The point to subtract from.</param>
    /// <param name="p2">The point to subtract.</param>
    /// <returns>A <see cref="Vector"/> representing the displacement from <paramref name="p2"/> to <paramref name="p1"/>.</returns>
    public static Vector operator -(Point p1, Point p2)
    {
        return new Vector(p1.X - p2.X, p1.Y - p2.Y, p1.Z - p2.Z);
    }

    /// <summary>
    /// Subtracts a vector from a point, translating the point in the opposite direction of the vector.
    /// </summary>
    /// <param name="p">The original point.</param>
    /// <param name="v">The vector to subtract.</param>
    /// <returns>A new <see cref="Point"/> after moving in the reverse direction of the vector.</returns>
    public static Point operator -(Point p, Vector v)
    {
        return new Point(p.X - v.X, p.Y - v.Y, p.Z - v.Z);
    }

    /// <summary>
    /// Mutiplies the Point by some scale
    /// </summary>
    /// <param name="p"></param>
    /// <param name="scale"></param>
    /// <returns>A scalar point</returns>
    public static Point operator *(Point p, double scale)
    {
        return new Point(p.X * scale, p.Y * scale, p.Z * scale);
    }

    /// <summary>
    /// Multiplication operator provided for symmetry
    /// </summary>
    /// /// <param name="scale"></param>
    /// <param name="p"></param>
    /// <returns></returns>
    public static Point operator *(double scale, Point p) => p * scale;

    /// <summary>
    /// Divides the Point by some scale
    /// </summary>
    /// <param name="p"></param>
    /// <param name="scale"></param>
    /// <returns>A scalar point</returns>
    public static Point operator /(Point p, double scale)
    {
        return new Point(p.X / scale, p.Y / scale, p.Z / scale);
    }
}
