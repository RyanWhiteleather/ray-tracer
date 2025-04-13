namespace RayTracer.Models;

/// <summary>
/// Represents a direction and magnitude in 3D space.
/// Vectors are immutable and support arithmetic operations.
/// </summary>
public readonly struct Vector : IEquatable<Vector>
{
    public double X {get;}
    public double Y {get;}
    public double Z {get;}

    public Vector(double x, double y, double z) => (X, Y, Z) = (x, y, z);

    /// <summary>
    /// Returns a hash code for this vector, rounded to match fuzzy equality rules.
    /// </summary>   
    public override int GetHashCode() {
        return HashCode.Combine(
            Math.Round(X, 5),
            Math.Round(Y, 5),
            Math.Round(Z, 5)
        );
    }

    /// <summary>
    /// Determines whether this vector is equal to another vector within a small margin of error.
    /// </summary>
    /// <param name="other">The vector to compare.</param>
    /// <returns><c>true</c> if the vectors are equal; otherwise, <c>false</c>.</returns>
    public bool Equals(Vector other) 
    {
        return Math.Abs(X - other.X) < MathConstants.Epsilon &&
               Math.Abs(Y - other.Y) < MathConstants.Epsilon &&
               Math.Abs(Z - other.Z) < MathConstants.Epsilon;
    }

    /// <inheritdoc/>
    public override bool Equals(object? obj)
    {
        return obj is Vector other && Equals(other);
    }

    public static bool operator ==(Vector a, Vector b) => a.Equals(b);
    public static bool operator !=(Vector a, Vector b) => !a.Equals(b);

    /// <summary>
    /// Adds two vectors together component-wise.
    /// </summary>
    /// <param name="v1">The first vector.</param>
    /// <param name="v2">The second vector.</param>
    /// <returns>A new <see cref="Vector"/> representing the sum.</returns>
    public static Vector operator +(Vector v1, Vector v2)
    {
        return new(v1.X + v2.X, v1.Y + v2.Y, v1.Z + v2.Z);
    }

    /// <summary>
    /// Adds a vector to a point, returning a new translated point.
    /// </summary>
    /// <param name="v">The vector to add.</param>
    /// <param name="p">The point to translate.</param>
    /// <returns>A new <see cref="Point"/> after applying the vector offset.</returns>
    public static Point operator +(Vector v, Point p) => p + v;

    /// <summary>
    /// Subtracts one vector from another, resulting in a vector representing the directional difference.
    /// </summary>
    /// <param name="v1">The vector to subtract from.</param>
    /// <param name="v2">The vector to subtract.</param>
    /// <returns>A new <see cref="Vector"/> representing the difference.</returns>
    public static Vector operator -(Vector v1, Vector v2)
    {
        return new Vector(v1.X - v2.X, v1.Y - v2.Y, v1.Z - v2.Z);
    }

    /// <summary>
    /// Negates the Vector, used to know the opposite of some vector.
    /// </summary>
    /// <param name="v"></param>
    /// <returns></returns>
    public static Vector operator -(Vector v)
    {
        return new Vector(-v.X, -v.Y, -v.Z);
    }

    /// <summary>
    /// Mutiplies the Vector by some scale
    /// </summary>
    /// <param name="v"></param>
    /// <param name="scale"></param>
    /// <returns>A scalar vector</returns>
    public static Vector operator *(Vector v, double scale)
    {
        return new Vector(v.X * scale, v.Y * scale, v.Z * scale);
    }

    /// <summary>
    /// Multiplication operator provided for symmetry
    /// </summary>
    /// <param name="scale"></param>
    /// <param name="v"></param>
    /// <returns></returns>
    public static Vector operator *(double scale, Vector v) => v * scale;

    /// <summary>
    /// Divides the Vector by some scale
    /// </summary>
    /// <param name="v"></param>
    /// <param name="scale"></param>
    /// <returns>A scalar vector</returns>
    public static Vector operator /(Vector v, double scale)
    {
        return new Vector(v.X / scale, v.Y / scale, v.Z / scale);
    }

    /// <summary>
    /// Division operator provided for symmetry
    /// </summary>
    /// <param name="scale"></param>
    /// <param name="v"></param>
    /// <returns></returns>
    public static Vector operator /(double scale, Vector v) => v / scale;
}
