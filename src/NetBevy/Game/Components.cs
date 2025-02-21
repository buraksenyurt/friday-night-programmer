namespace NetBevy.Game;

[Component]
public class Position
    : IComponent
{
    public float X { get; set; }
    public float Y { get; set; }
}

[Component]
public class Velocity
    : IComponent
{
    public float X { get; set; }
    public float Y { get; set; }
}

[Component]
public class Range
    : IComponent
{
    public float Value { get; set; }

}
