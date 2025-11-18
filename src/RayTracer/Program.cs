using System.Security.Principal;
using RayTracer.Models;

var environment = new Environment { Gravity = new(0, -0.1, 0), Wind = new(-0.01, 0, 0) };
var projectile = new Projectile
{
    Position = new(0, 1, 0),
    Velocity = new Vector(1, 1.8, 0).Normalize() * 11.25,
};

var canvas = new Canvas(900, 550);

var tick = 0;
var color = new Color(1, 0, 0);
while (projectile.Position.Y > 0)
{
    projectile = Tick(environment, projectile);
    Console.WriteLine(
        $"Tick {tick}: Position = {projectile.Position} Coridinates = {(int)projectile.Position.X},{550 - (int)projectile.Position.Y}"
    );

    canvas.WritePixel((int)projectile.Position.X, 550 - (int)projectile.Position.Y, color);

    tick++;
}

var ppm = canvas.ToPPM();
var home = System.Environment.GetFolderPath(System.Environment.SpecialFolder.UserProfile);
var path = Path.Combine(home, "ray-tracer-images", "image.ppm");

Directory.CreateDirectory(Path.GetDirectoryName(path)!);
File.WriteAllText(path, ppm);

Projectile Tick(Environment environemnt, Projectile projectile)
{
    var position = projectile.Position + projectile.Velocity;
    var velocity = projectile.Velocity + environemnt.Gravity + environemnt.Wind;
    return new Projectile { Position = position, Velocity = velocity };
}

public struct Projectile
{
    public Point Position { get; set; }
    public Vector Velocity { get; set; }
}

public struct Environment
{
    public Vector Gravity { get; set; }
    public Vector Wind { get; set; }
}
