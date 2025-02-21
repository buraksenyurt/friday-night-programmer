namespace NetBevy.Game;

public static class GameApp
{
    public static void Run()
    {
        World world = new();

        var player = world.CreateEntity();
        player.AddComponent(new Position { X = 10, Y = 10 });
        player.AddComponent(new Velocity { X = 1, Y = 0 });

        var enemy = world.CreateEntity();
        enemy.AddComponent(new Position { X = 100, Y = 10 });
        enemy.AddComponent(new Velocity { X = -1, Y = 0 });

        var tower = world.CreateEntity();
        tower.AddComponent(new Position { X = 0, Y = 0 });
        tower.AddComponent(new Range { Value = 85 });

        var scheduler = new Scheduler();

        scheduler.AddSystems(SystemState.Startup, [new SetupSystem()]);
        scheduler.AddSystems(SystemState.Update, [new MovementSystem(), new PlayerCollisionCheckSystem()]);

        world.Run(scheduler);
    }
}
