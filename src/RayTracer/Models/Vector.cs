namespace RayTracer.Models;

public readonly struct Vector : IEquatable<Vector>
{
    public double X {get;}
    public double Y {get;}
    public double Z {get;}

    public Vector(double x, double y, double z) => (X, Y, Z) = (x, y, z);
    public override bool Equals(object? obj)
    {
        return obj is Vector other && Equals(other);
    }
        
    public override int GetHashCode() {
        return HashCode.Combine(Math.Round(X / MathConstants.Epsilon),
                         Math.Round(Y / MathConstants.Epsilon),
                         Math.Round(Z / MathConstants.Epsilon));
    }

    /// <summary>
    /// Checks the equaility of two Points
    /// </summary>
    /// <param name="other"></param>
    /// <returns></returns>
    public bool Equals(Vector other) 
    {
        return Math.Abs(X - other.X) < MathConstants.Epsilon &&
               Math.Abs(Y - other.Y) < MathConstants.Epsilon &&
               Math.Abs(Z - other.Z) < MathConstants.Epsilon;
    }

    public static bool operator ==(Vector a, Vector b) => a.Equals(b);
    public static bool operator !=(Vector a, Vector b) => !a.Equals(b);

    public static Vector operator +(Vector p, Vector v)
    {
        return new(p.X + v.X, p.Y + v.Y, p.Z + v.Z);
    }

    public static Point operator +(Vector v, Point p) => p + v;
}
