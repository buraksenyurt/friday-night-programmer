namespace NetBevy.Game;

public class Position
    : IComponent
{
    public float X { get; set; }
    public float Y { get; set; }
}

public class Velocity
    : IComponent
{
    public float X { get; set; }
    public float Y { get; set; }
}

public class Range
    : IComponent
{
    public float Value { get; set; }

}
